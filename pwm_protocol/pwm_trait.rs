#[derive(Debug)]
pub enum PwmError {
    EINVAL,
    ENOTIMPLEMENTED,
}

#[derive(Debug)]
pub struct PwmState {
    pub enabled: bool,
    // The frequency that pwm operating under
    // Not the frequency pwm output
    pub clock_frequency: u32,
    pub period: u32,
    pub duty: u32,
}

#[derive(Debug)]
pub struct PwmRequest {
    pub period: u32,
    pub duty: u32,
}

pub trait PwmHal {
    fn pwm_get_state(&self) -> Result<PwmState, PwmError> {
        Err(PwmError::ENOTIMPLEMENTED)
    }

    // Send request will enable the disabled pwm automatically
    fn pwm_send_request(&mut self, request: PwmRequest) -> Result<(), PwmError> {
        Err(PwmError::ENOTIMPLEMENTED)
    }

    fn pwm_disable(&mut self) -> Result<(), PwmError> {
        Err(PwmError::ENOTIMPLEMENTED)
    }
}
