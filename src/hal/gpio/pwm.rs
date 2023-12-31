

/// Checks if the specified port is available for digital use.
/// Has a sensible default implementation but can be overriden if needed.
#[allow(clippy::missing_errors_doc)]
pub fn inside_pwm_range(port: u8, value: f64) -> Result<(), GPIOError> {
    if (0.0..=1.0).contains(&value) {
        Ok(())
    } else {
        Err(GPIOError::ValueOutOfRange(value, GPIOPortType::PWM, port))
    }
}