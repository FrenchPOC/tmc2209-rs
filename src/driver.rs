//! TMC2209 high-level driver.
//!
//! This module provides the main `Tmc2209` struct for communicating with
//! TMC2209 stepper motor drivers via UART.

use crate::datagram::{ReadRequest, ReadResponse, ResponseReader, WriteRequest};
use crate::error::Error;
use crate::registers::{
    Chopconf, Coolconf, DrvStatus, Gconf, Gstat, Ifcnt, IholdIrun, Ioin, MicrostepResolution,
    Mscnt, Pwmconf, ReadableRegister, SgResult, Sgthrs, Tcoolthrs, Tpwmthrs, Tstep, Vactual,
    WritableRegister,
};

/// TMC2209 driver over UART.
///
/// This struct provides methods for reading and writing TMC2209 registers
/// via a UART interface. Both blocking and async versions are available
/// depending on enabled features.
///
/// # Type Parameters
///
/// * `U` - UART peripheral type implementing `embedded_io::Read + embedded_io::Write`
///         or `embedded_io_async::Read + embedded_io_async::Write`
///
/// # Example (blocking)
///
/// ```ignore
/// use tmc2209::{Tmc2209, registers::IholdIrun};
///
/// let mut driver = Tmc2209::new(uart, 0);
///
/// // Read a register
/// let status = driver.read_register::<DrvStatus>()?;
///
/// // Write a register
/// let mut irun = IholdIrun::new();
/// irun.set_irun(16).set_ihold(8);
/// driver.write_register(&irun)?;
/// ```
pub struct Tmc2209<U> {
    /// UART peripheral.
    uart: U,
    /// Slave address (0-3).
    slave_addr: u8,
    /// Response reader for parsing incoming data.
    reader: ResponseReader,
}

impl<U> Tmc2209<U> {
    /// Create a new TMC2209 driver.
    ///
    /// # Arguments
    ///
    /// * `uart` - UART peripheral for communication
    /// * `slave_addr` - Slave address (0-3)
    ///
    /// # Panics
    ///
    /// Panics if `slave_addr` is greater than 3.
    pub fn new(uart: U, slave_addr: u8) -> Self {
        assert!(slave_addr <= 3, "Slave address must be 0-3");
        Self {
            uart,
            slave_addr,
            reader: ResponseReader::new(),
        }
    }

    /// Get the slave address.
    pub fn slave_addr(&self) -> u8 {
        self.slave_addr
    }

    /// Set the slave address.
    ///
    /// # Panics
    ///
    /// Panics if `addr` is greater than 3.
    pub fn set_slave_addr(&mut self, addr: u8) {
        assert!(addr <= 3, "Slave address must be 0-3");
        self.slave_addr = addr;
    }

    /// Get a reference to the UART peripheral.
    pub fn uart(&self) -> &U {
        &self.uart
    }

    /// Get a mutable reference to the UART peripheral.
    pub fn uart_mut(&mut self) -> &mut U {
        &mut self.uart
    }

    /// Release the UART peripheral.
    pub fn release(self) -> U {
        self.uart
    }

    /// Create a read request for a register.
    fn read_request<R: ReadableRegister>(&self) -> ReadRequest {
        ReadRequest::new(self.slave_addr, R::ADDRESS)
    }

    /// Create a write request for a register.
    fn write_request<R: WritableRegister>(&self, reg: &R) -> WriteRequest {
        WriteRequest::new(self.slave_addr, R::ADDRESS, (*reg).into())
    }
}

// ============================================================================
// Blocking API
// ============================================================================

#[cfg(feature = "blocking")]
impl<U, E> Tmc2209<U>
where
    U: embedded_io::Read<Error = E> + embedded_io::Write<Error = E>,
{
    /// Read a register (blocking).
    ///
    /// Sends a read request and waits for the response.
    ///
    /// # Type Parameters
    ///
    /// * `R` - Register type to read (must implement `ReadableRegister`)
    ///
    /// # Returns
    ///
    /// The register value, or an error if communication fails.
    pub fn read_register<R: ReadableRegister>(&mut self) -> Result<R, Error<E>> {
        let request = self.read_request::<R>();

        // Send the read request
        self.uart
            .write_all(request.as_bytes())
            .map_err(Error::Uart)?;
        self.uart.flush().map_err(Error::Uart)?;

        // Read the response
        // TMC2209 echoes back the request, then sends the response
        // We need to skip the echo (4 bytes) and read the response (8 bytes)
        let mut echo_buf = [0u8; 4];
        self.read_exact(&mut echo_buf)?;

        let response = self.read_response()?;

        // Verify the register address matches
        let expected_addr = R::ADDRESS as u8;
        if response.reg_addr() != expected_addr {
            return Err(Error::AddressMismatch {
                expected: expected_addr,
                actual: response.reg_addr(),
            });
        }

        Ok(R::from(response.data()))
    }

    /// Write a register (blocking).
    ///
    /// Sends a write request to update a register value.
    ///
    /// # Arguments
    ///
    /// * `reg` - Register value to write
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or an error if communication fails.
    pub fn write_register<R: WritableRegister>(&mut self, reg: &R) -> Result<(), Error<E>> {
        let request = self.write_request(reg);

        // Send the write request
        self.uart
            .write_all(request.as_bytes())
            .map_err(Error::Uart)?;
        self.uart.flush().map_err(Error::Uart)?;

        // Read back the echo (8 bytes) - TMC2209 echoes write requests
        let mut echo_buf = [0u8; 8];
        self.read_exact(&mut echo_buf)?;

        Ok(())
    }

    /// Read a register by raw address (blocking).
    ///
    /// Use this when you need to read a register by its raw address value.
    pub fn read_raw(&mut self, reg_addr: u8) -> Result<u32, Error<E>> {
        let request = ReadRequest::from_raw_addr(self.slave_addr, reg_addr);

        self.uart
            .write_all(request.as_bytes())
            .map_err(Error::Uart)?;
        self.uart.flush().map_err(Error::Uart)?;

        // Skip echo
        let mut echo_buf = [0u8; 4];
        self.read_exact(&mut echo_buf)?;

        let response = self.read_response()?;
        Ok(response.data())
    }

    /// Write a register by raw address (blocking).
    ///
    /// Use this when you need to write a register by its raw address value.
    pub fn write_raw(&mut self, reg_addr: u8, data: u32) -> Result<(), Error<E>> {
        let request = WriteRequest::from_raw(self.slave_addr, reg_addr, data);

        self.uart
            .write_all(request.as_bytes())
            .map_err(Error::Uart)?;
        self.uart.flush().map_err(Error::Uart)?;

        // Read back echo
        let mut echo_buf = [0u8; 8];
        self.read_exact(&mut echo_buf)?;

        Ok(())
    }

    /// Helper to read exact number of bytes.
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), Error<E>> {
        let mut total_read = 0;
        while total_read < buf.len() {
            let n = self.uart.read(&mut buf[total_read..]).map_err(Error::Uart)?;
            if n == 0 {
                return Err(Error::NoResponse);
            }
            total_read += n;
        }
        Ok(())
    }

    /// Helper to read a complete response.
    fn read_response(&mut self) -> Result<ReadResponse, Error<E>> {
        self.reader.reset();
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf)?;

        let (_, result) = self.reader.feed(&buf);
        result.ok_or(Error::NoResponse)?
    }

    // ========================================================================
    // Convenience methods (blocking)
    // ========================================================================

    /// Check if the driver is communicating properly.
    ///
    /// Reads IFCNT and verifies we get a valid response.
    pub fn is_connected(&mut self) -> bool {
        self.read_register::<Ifcnt>().is_ok()
    }

    /// Get the interface transmission counter.
    ///
    /// This counter increments on each successful UART write.
    /// Useful for verifying communication.
    pub fn ifcnt(&mut self) -> Result<u8, Error<E>> {
        let reg = self.read_register::<Ifcnt>()?;
        Ok(reg.count())
    }

    /// Get the global status flags.
    pub fn gstat(&mut self) -> Result<Gstat, Error<E>> {
        self.read_register()
    }

    /// Clear the global status flags (write to clear).
    pub fn clear_gstat(&mut self) -> Result<(), Error<E>> {
        // Writing 1s clears the flags
        let gstat = Gstat::from(0x07);
        self.write_register(&gstat)
    }

    /// Get the input pin states.
    pub fn ioin(&mut self) -> Result<Ioin, Error<E>> {
        self.read_register()
    }

    /// Get the driver status.
    pub fn drv_status(&mut self) -> Result<DrvStatus, Error<E>> {
        self.read_register()
    }

    /// Get the current step time (inverse of velocity).
    pub fn tstep(&mut self) -> Result<u32, Error<E>> {
        let reg = self.read_register::<Tstep>()?;
        Ok(reg.tstep())
    }

    /// Get the StallGuard result.
    pub fn sg_result(&mut self) -> Result<u16, Error<E>> {
        let reg = self.read_register::<SgResult>()?;
        Ok(reg.result())
    }

    /// Get the microstep counter position (0-1023).
    pub fn mscnt(&mut self) -> Result<u16, Error<E>> {
        let reg = self.read_register::<Mscnt>()?;
        Ok(reg.count())
    }

    /// Set the motor currents.
    ///
    /// # Arguments
    ///
    /// * `run_current` - Run current (0-31)
    /// * `hold_current` - Hold current (0-31)
    /// * `hold_delay` - Delay before reducing to hold current (0-15)
    pub fn set_current(
        &mut self,
        run_current: u8,
        hold_current: u8,
        hold_delay: u8,
    ) -> Result<(), Error<E>> {
        let mut reg = IholdIrun::new();
        reg.set_irun(run_current)
            .set_ihold(hold_current)
            .set_iholddelay(hold_delay);
        self.write_register(&reg)
    }

    /// Set the microstep resolution.
    pub fn set_microsteps(&mut self, resolution: MicrostepResolution) -> Result<(), Error<E>> {
        let mut chopconf = self.read_register::<Chopconf>()?;
        chopconf.set_microstep_resolution(resolution);
        self.write_register(&chopconf)
    }

    /// Enable or disable the driver.
    ///
    /// When TOFF=0, the driver is disabled.
    pub fn set_enabled(&mut self, enabled: bool) -> Result<(), Error<E>> {
        let mut chopconf = self.read_register::<Chopconf>()?;
        if enabled {
            // Use default TOFF=3 if currently disabled
            if chopconf.toff() == 0 {
                chopconf.set_toff(3);
            }
        } else {
            chopconf.set_toff(0);
        }
        self.write_register(&chopconf)
    }

    /// Set velocity for internal motion controller (VACTUAL).
    ///
    /// # Arguments
    ///
    /// * `velocity` - Velocity value (signed, 23-bit range)
    ///   - Positive: Forward motion
    ///   - Negative: Reverse motion
    ///   - 0: Stop
    pub fn set_velocity(&mut self, velocity: i32) -> Result<(), Error<E>> {
        let mut reg = Vactual::new();
        reg.set_velocity(velocity);
        self.write_register(&reg)
    }

    /// Stop the motor (set VACTUAL to 0).
    pub fn stop(&mut self) -> Result<(), Error<E>> {
        self.set_velocity(0)
    }

    /// Set the StallGuard threshold.
    ///
    /// Higher values make stall detection more sensitive.
    pub fn set_stall_threshold(&mut self, threshold: u8) -> Result<(), Error<E>> {
        let mut reg = Sgthrs::new();
        reg.set_threshold(threshold);
        self.write_register(&reg)
    }

    /// Enable StealthChop mode.
    pub fn enable_stealthchop(&mut self) -> Result<(), Error<E>> {
        let mut gconf = self.read_register::<Gconf>()?;
        gconf.set_en_spreadcycle(false);
        self.write_register(&gconf)
    }

    /// Enable SpreadCycle mode.
    pub fn enable_spreadcycle(&mut self) -> Result<(), Error<E>> {
        let mut gconf = self.read_register::<Gconf>()?;
        gconf.set_en_spreadcycle(true);
        self.write_register(&gconf)
    }

    /// Check if motor is in standstill.
    pub fn is_standstill(&mut self) -> Result<bool, Error<E>> {
        let status = self.drv_status()?;
        Ok(status.stst())
    }

    /// Check if overtemperature warning is active.
    pub fn is_overtemperature_warning(&mut self) -> Result<bool, Error<E>> {
        let status = self.drv_status()?;
        Ok(status.otpw())
    }

    /// Check if overtemperature shutdown is active.
    pub fn is_overtemperature_shutdown(&mut self) -> Result<bool, Error<E>> {
        let status = self.drv_status()?;
        Ok(status.ot())
    }

    // ========================================================================
    // CoolStep and StallGuard methods (blocking)
    // ========================================================================

    /// Enable CoolStep adaptive current control.
    ///
    /// CoolStep automatically reduces motor current when load is low,
    /// saving power and reducing heat.
    ///
    /// # Arguments
    ///
    /// * `semin` - Minimum StallGuard value for current increase (1-15, 0 disables)
    /// * `semax` - Hysteresis for current decrease (0-15)
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Enable CoolStep with moderate sensitivity
    /// driver.enable_coolstep(4, 2)?;
    /// ```
    pub fn enable_coolstep(&mut self, semin: u8, semax: u8) -> Result<(), Error<E>> {
        let mut coolconf = Coolconf::new();
        coolconf
            .set_semin(semin.min(15))
            .set_semax(semax.min(15))
            .set_seup(0)  // +1 current step
            .set_sedn(0); // -32 current step
        self.write_register(&coolconf)
    }

    /// Disable CoolStep.
    pub fn disable_coolstep(&mut self) -> Result<(), Error<E>> {
        let coolconf = Coolconf::new(); // semin=0 disables CoolStep
        self.write_register(&coolconf)
    }

    /// Set the CoolStep velocity threshold (TCOOLTHRS).
    ///
    /// CoolStep and StallGuard are only active when TSTEP < TCOOLTHRS.
    /// Below this velocity, CoolStep and stall detection are disabled.
    ///
    /// # Arguments
    ///
    /// * `threshold` - TSTEP threshold value (0 = disabled, 0xFFFFF = always active)
    pub fn set_coolstep_threshold(&mut self, threshold: u32) -> Result<(), Error<E>> {
        let mut tcoolthrs = Tcoolthrs::new();
        tcoolthrs.set_threshold(threshold);
        self.write_register(&tcoolthrs)
    }

    /// Set the StealthChop velocity threshold (TPWMTHRS).
    ///
    /// Above this velocity, the driver switches from StealthChop to SpreadCycle.
    ///
    /// # Arguments
    ///
    /// * `threshold` - TSTEP threshold value (0 = only SpreadCycle, 0xFFFFF = only StealthChop)
    pub fn set_stealthchop_threshold(&mut self, threshold: u32) -> Result<(), Error<E>> {
        let mut tpwmthrs = Tpwmthrs::new();
        tpwmthrs.set_threshold(threshold);
        self.write_register(&tpwmthrs)
    }

    // ========================================================================
    // Sensorless homing methods (blocking)
    // ========================================================================

    /// Configure stall detection for sensorless homing.
    ///
    /// This sets up the StallGuard feature to detect when the motor
    /// hits an endstop or obstacle.
    ///
    /// # Arguments
    ///
    /// * `threshold` - StallGuard threshold (0-255, higher = more sensitive)
    ///
    /// # Note
    ///
    /// For sensorless homing to work properly:
    /// - Use SpreadCycle mode (not StealthChop)
    /// - Set appropriate TCOOLTHRS (stall detection only works above this velocity)
    /// - Move at a consistent, moderate speed
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Configure for sensorless homing
    /// driver.configure_stall_detection(50)?;
    /// driver.enable_spreadcycle()?;
    /// driver.set_coolstep_threshold(0)?; // Enable at all velocities
    ///
    /// // Move towards endstop
    /// driver.set_velocity(-1000)?;
    ///
    /// // Poll for stall
    /// loop {
    ///     if driver.is_stalled()? {
    ///         driver.stop()?;
    ///         break;
    ///     }
    /// }
    /// ```
    pub fn configure_stall_detection(&mut self, threshold: u8) -> Result<(), Error<E>> {
        // Set StallGuard threshold
        self.set_stall_threshold(threshold)?;

        // Enable DIAG output for stall indication
        let mut gconf = self.read_register::<Gconf>()?;
        gconf.set_diag0_stall(true);
        self.write_register(&gconf)?;

        Ok(())
    }

    /// Check if the motor is currently stalled.
    ///
    /// Returns true if the StallGuard result is below the threshold.
    /// Only valid when motor is moving and TSTEP < TCOOLTHRS.
    pub fn is_stalled(&mut self) -> Result<bool, Error<E>> {
        let sg_result = self.sg_result()?;
        // SG_RESULT of 0 indicates stall
        Ok(sg_result == 0)
    }

    /// Get the current load indicator from StallGuard.
    ///
    /// Returns a value from 0 (high load/stall) to 510 (no load).
    /// Useful for load monitoring and stall detection tuning.
    pub fn load_indicator(&mut self) -> Result<u16, Error<E>> {
        self.sg_result()
    }

    // ========================================================================
    // PWM and StealthChop configuration (blocking)
    // ========================================================================

    /// Configure StealthChop PWM settings.
    ///
    /// # Arguments
    ///
    /// * `pwm_ofs` - PWM amplitude offset (0-255)
    /// * `pwm_grad` - PWM amplitude gradient (0-255)
    /// * `autoscale` - Enable automatic current scaling
    /// * `autograd` - Enable automatic gradient adaptation
    pub fn configure_stealthchop(
        &mut self,
        pwm_ofs: u8,
        pwm_grad: u8,
        autoscale: bool,
        autograd: bool,
    ) -> Result<(), Error<E>> {
        let mut pwmconf = self.read_register::<Pwmconf>()?;
        pwmconf
            .set_pwm_ofs(pwm_ofs)
            .set_pwm_grad(pwm_grad)
            .set_pwm_autoscale(autoscale)
            .set_pwm_autograd(autograd);
        self.write_register(&pwmconf)
    }

    /// Set VSENSE for current sense resistor scaling.
    ///
    /// # Arguments
    ///
    /// * `high_sensitivity` - true for high sensitivity (low current range),
    ///                        false for low sensitivity (high current range)
    pub fn set_vsense(&mut self, high_sensitivity: bool) -> Result<(), Error<E>> {
        let mut chopconf = self.read_register::<Chopconf>()?;
        chopconf.set_vsense(high_sensitivity);
        self.write_register(&chopconf)
    }

    /// Configure chopper settings for optimal performance.
    ///
    /// # Arguments
    ///
    /// * `toff` - Off-time (1-15, 0 disables driver)
    /// * `hstrt` - Hysteresis start (0-7)
    /// * `hend` - Hysteresis end (0-15)
    /// * `tbl` - Comparator blank time (0-3)
    pub fn configure_chopper(
        &mut self,
        toff: u8,
        hstrt: u8,
        hend: u8,
        tbl: u8,
    ) -> Result<(), Error<E>> {
        let mut chopconf = self.read_register::<Chopconf>()?;
        chopconf
            .set_toff(toff.min(15))
            .set_hstrt(hstrt.min(7))
            .set_hend(hend.min(15))
            .set_tbl(tbl.min(3));
        self.write_register(&chopconf)
    }

    /// Enable interpolation to 256 microsteps.
    ///
    /// When enabled, the driver interpolates between microsteps
    /// for smoother motion, even at lower microstep settings.
    pub fn set_interpolation(&mut self, enabled: bool) -> Result<(), Error<E>> {
        let mut chopconf = self.read_register::<Chopconf>()?;
        chopconf.set_intpol(enabled);
        self.write_register(&chopconf)
    }

    /// Read the actual motor current scale being used.
    ///
    /// This reflects CoolStep adjustments if enabled.
    /// Returns a value from 0-31.
    pub fn actual_current_scale(&mut self) -> Result<u8, Error<E>> {
        let status = self.drv_status()?;
        Ok(status.cs_actual())
    }

    /// Check if the driver is currently in StealthChop mode.
    pub fn is_stealthchop_active(&mut self) -> Result<bool, Error<E>> {
        let status = self.drv_status()?;
        Ok(status.stealth())
    }

    /// Get a comprehensive status summary.
    ///
    /// Returns a tuple of (errors_present, warnings_present, is_running).
    pub fn status_summary(&mut self) -> Result<(bool, bool, bool), Error<E>> {
        let status = self.drv_status()?;

        let errors = status.has_error();
        let warnings = status.otpw() || status.open_load_detected();
        let running = !status.stst();

        Ok((errors, warnings, running))
    }
}

// ============================================================================
// Async API
// ============================================================================

#[cfg(feature = "async")]
impl<U, E> Tmc2209<U>
where
    U: embedded_io_async::Read<Error = E> + embedded_io_async::Write<Error = E>,
{
    /// Read a register (async).
    ///
    /// Sends a read request and waits for the response.
    pub async fn read_register_async<R: ReadableRegister>(&mut self) -> Result<R, Error<E>> {
        let request = self.read_request::<R>();

        // Send the read request
        self.uart
            .write_all(request.as_bytes())
            .await
            .map_err(Error::Uart)?;
        self.uart.flush().await.map_err(Error::Uart)?;

        // Skip the echo (4 bytes)
        let mut echo_buf = [0u8; 4];
        self.read_exact_async(&mut echo_buf).await?;

        // Read the response
        let response = self.read_response_async().await?;

        // Verify the register address matches
        let expected_addr = R::ADDRESS as u8;
        if response.reg_addr() != expected_addr {
            return Err(Error::AddressMismatch {
                expected: expected_addr,
                actual: response.reg_addr(),
            });
        }

        Ok(R::from(response.data()))
    }

    /// Write a register (async).
    ///
    /// Sends a write request to update a register value.
    pub async fn write_register_async<R: WritableRegister>(
        &mut self,
        reg: &R,
    ) -> Result<(), Error<E>> {
        let request = self.write_request(reg);

        // Send the write request
        self.uart
            .write_all(request.as_bytes())
            .await
            .map_err(Error::Uart)?;
        self.uart.flush().await.map_err(Error::Uart)?;

        // Read back the echo (8 bytes)
        let mut echo_buf = [0u8; 8];
        self.read_exact_async(&mut echo_buf).await?;

        Ok(())
    }

    /// Read a register by raw address (async).
    pub async fn read_raw_async(&mut self, reg_addr: u8) -> Result<u32, Error<E>> {
        let request = ReadRequest::from_raw_addr(self.slave_addr, reg_addr);

        self.uart
            .write_all(request.as_bytes())
            .await
            .map_err(Error::Uart)?;
        self.uart.flush().await.map_err(Error::Uart)?;

        // Skip echo
        let mut echo_buf = [0u8; 4];
        self.read_exact_async(&mut echo_buf).await?;

        let response = self.read_response_async().await?;
        Ok(response.data())
    }

    /// Write a register by raw address (async).
    pub async fn write_raw_async(&mut self, reg_addr: u8, data: u32) -> Result<(), Error<E>> {
        let request = WriteRequest::from_raw(self.slave_addr, reg_addr, data);

        self.uart
            .write_all(request.as_bytes())
            .await
            .map_err(Error::Uart)?;
        self.uart.flush().await.map_err(Error::Uart)?;

        // Read back echo
        let mut echo_buf = [0u8; 8];
        self.read_exact_async(&mut echo_buf).await?;

        Ok(())
    }

    /// Helper to read exact number of bytes (async).
    async fn read_exact_async(&mut self, buf: &mut [u8]) -> Result<(), Error<E>> {
        let mut total_read = 0;
        while total_read < buf.len() {
            let n = self
                .uart
                .read(&mut buf[total_read..])
                .await
                .map_err(Error::Uart)?;
            if n == 0 {
                return Err(Error::NoResponse);
            }
            total_read += n;
        }
        Ok(())
    }

    /// Helper to read a complete response (async).
    async fn read_response_async(&mut self) -> Result<ReadResponse, Error<E>> {
        self.reader.reset();
        let mut buf = [0u8; 8];
        self.read_exact_async(&mut buf).await?;

        let (_, result) = self.reader.feed(&buf);
        result.ok_or(Error::NoResponse)?
    }

    // ========================================================================
    // Convenience methods (async)
    // ========================================================================

    /// Check if the driver is communicating properly (async).
    pub async fn is_connected_async(&mut self) -> bool {
        self.read_register_async::<Ifcnt>().await.is_ok()
    }

    /// Get the interface transmission counter (async).
    pub async fn ifcnt_async(&mut self) -> Result<u8, Error<E>> {
        let reg = self.read_register_async::<Ifcnt>().await?;
        Ok(reg.count())
    }

    /// Get the driver status (async).
    pub async fn drv_status_async(&mut self) -> Result<DrvStatus, Error<E>> {
        self.read_register_async().await
    }

    /// Set the motor currents (async).
    pub async fn set_current_async(
        &mut self,
        run_current: u8,
        hold_current: u8,
        hold_delay: u8,
    ) -> Result<(), Error<E>> {
        let mut reg = IholdIrun::new();
        reg.set_irun(run_current)
            .set_ihold(hold_current)
            .set_iholddelay(hold_delay);
        self.write_register_async(&reg).await
    }

    /// Set the microstep resolution (async).
    pub async fn set_microsteps_async(
        &mut self,
        resolution: MicrostepResolution,
    ) -> Result<(), Error<E>> {
        let mut chopconf = self.read_register_async::<Chopconf>().await?;
        chopconf.set_microstep_resolution(resolution);
        self.write_register_async(&chopconf).await
    }

    /// Set velocity for internal motion controller (async).
    pub async fn set_velocity_async(&mut self, velocity: i32) -> Result<(), Error<E>> {
        let mut reg = Vactual::new();
        reg.set_velocity(velocity);
        self.write_register_async(&reg).await
    }

    /// Stop the motor (async).
    pub async fn stop_async(&mut self) -> Result<(), Error<E>> {
        self.set_velocity_async(0).await
    }

    // ========================================================================
    // CoolStep and StallGuard methods (async)
    // ========================================================================

    /// Enable CoolStep adaptive current control (async).
    pub async fn enable_coolstep_async(&mut self, semin: u8, semax: u8) -> Result<(), Error<E>> {
        let mut coolconf = Coolconf::new();
        coolconf
            .set_semin(semin.min(15))
            .set_semax(semax.min(15))
            .set_seup(0)
            .set_sedn(0);
        self.write_register_async(&coolconf).await
    }

    /// Disable CoolStep (async).
    pub async fn disable_coolstep_async(&mut self) -> Result<(), Error<E>> {
        let coolconf = Coolconf::new();
        self.write_register_async(&coolconf).await
    }

    /// Set the CoolStep velocity threshold (async).
    pub async fn set_coolstep_threshold_async(&mut self, threshold: u32) -> Result<(), Error<E>> {
        let mut tcoolthrs = Tcoolthrs::new();
        tcoolthrs.set_threshold(threshold);
        self.write_register_async(&tcoolthrs).await
    }

    /// Set the StealthChop velocity threshold (async).
    pub async fn set_stealthchop_threshold_async(
        &mut self,
        threshold: u32,
    ) -> Result<(), Error<E>> {
        let mut tpwmthrs = Tpwmthrs::new();
        tpwmthrs.set_threshold(threshold);
        self.write_register_async(&tpwmthrs).await
    }

    // ========================================================================
    // Sensorless homing methods (async)
    // ========================================================================

    /// Configure stall detection for sensorless homing (async).
    pub async fn configure_stall_detection_async(&mut self, threshold: u8) -> Result<(), Error<E>> {
        // Set StallGuard threshold
        let mut sgthrs = Sgthrs::new();
        sgthrs.set_threshold(threshold);
        self.write_register_async(&sgthrs).await?;

        // Enable DIAG output for stall indication
        let mut gconf = self.read_register_async::<Gconf>().await?;
        gconf.set_diag0_stall(true);
        self.write_register_async(&gconf).await?;

        Ok(())
    }

    /// Check if the motor is currently stalled (async).
    pub async fn is_stalled_async(&mut self) -> Result<bool, Error<E>> {
        let sg = self.read_register_async::<SgResult>().await?;
        Ok(sg.result() == 0)
    }

    /// Get the current load indicator from StallGuard (async).
    pub async fn load_indicator_async(&mut self) -> Result<u16, Error<E>> {
        let sg = self.read_register_async::<SgResult>().await?;
        Ok(sg.result())
    }

    // ========================================================================
    // Mode selection (async)
    // ========================================================================

    /// Enable StealthChop mode (async).
    pub async fn enable_stealthchop_async(&mut self) -> Result<(), Error<E>> {
        let mut gconf = self.read_register_async::<Gconf>().await?;
        gconf.set_en_spreadcycle(false);
        self.write_register_async(&gconf).await
    }

    /// Enable SpreadCycle mode (async).
    pub async fn enable_spreadcycle_async(&mut self) -> Result<(), Error<E>> {
        let mut gconf = self.read_register_async::<Gconf>().await?;
        gconf.set_en_spreadcycle(true);
        self.write_register_async(&gconf).await
    }

    /// Enable or disable the driver (async).
    pub async fn set_enabled_async(&mut self, enabled: bool) -> Result<(), Error<E>> {
        let mut chopconf = self.read_register_async::<Chopconf>().await?;
        if enabled {
            if chopconf.toff() == 0 {
                chopconf.set_toff(3);
            }
        } else {
            chopconf.set_toff(0);
        }
        self.write_register_async(&chopconf).await
    }

    /// Check if motor is in standstill (async).
    pub async fn is_standstill_async(&mut self) -> Result<bool, Error<E>> {
        let status = self.drv_status_async().await?;
        Ok(status.stst())
    }
}
