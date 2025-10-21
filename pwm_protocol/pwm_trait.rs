#[derive(Debug)]
pub enum PwmError {
    EINVAL,
    ENOTIMPLEMENTED,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PwmPolarity {
    PwmPolarityNormal = 0,
    PwmPolarityInversed = 1,
}

type ClockFrequency = u32;

#[derive(Debug)]
pub enum PwmState {
    Enable(PwmRawState),
    Disable,
}

#[derive(Debug)]
pub struct PwmRawState {
    pub polarity: PwmPolarity,
    pub period_cycle: u32,
    pub duty_cycle: u32,
}

/// @period_ns: PWM period (in nanoseconds)
/// @duty_ns: PWM duty cycle (in nanoseconds)
/// @polarity: PWM polarity
/// @enabled: PWM enabled status
#[derive(Debug)]
pub struct PwmRequest {
    pub period_ns: u32,
    pub duty_ns: u32,
    pub polarity: PwmPolarity,
}

pub trait PwmHal {
    /// Return the state of Pwm and the frequency of the clock applied to the pwm
    fn pwm_get_state(&mut self) -> Result<(PwmState, ClockFrequency), PwmError> {
        Err(PwmError::ENOTIMPLEMENTED)
    }

    // Send request will enable the disabled pwm automatically
    fn pwm_send_request(&mut self, request: PwmRequest) -> Result<(), PwmError> {
        Err(PwmError::ENOTIMPLEMENTED)
    }

    fn pwm_apply_state(&mut self, state: PwmState) -> Result<(), PwmError> {
        Err(PwmError::ENOTIMPLEMENTED)
    }
}
