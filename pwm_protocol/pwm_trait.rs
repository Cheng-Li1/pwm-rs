// Copyright 2025, UNSW
// SPDX-License-Identifier: BSD-2-Clause

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
#[derive(Debug)]
pub struct PwmRequest {
    pub period_ns: u32,
    pub duty_ns: u32,
    pub polarity: PwmPolarity,
}

const NS_IN_S: u32 = 1000_000_000;

pub trait PwmHal {
    /// Return the state of Pwm and the frequency of the clock applied to the pwm
    fn pwm_get_state(&mut self) -> Result<(PwmState, ClockFrequency), PwmError> {
        Err(PwmError::ENOTIMPLEMENTED)
    }

    fn pwm_apply_state(&mut self, state: PwmState) -> Result<(), PwmError> {
        Err(PwmError::ENOTIMPLEMENTED)
    }

    // Send request will enable the disabled pwm automatically
    fn pwm_send_request(&mut self, request: PwmRequest) -> Result<(), PwmError> {
        let (_, clock) = self.pwm_get_state().unwrap();

        let duty_cycle = (request.duty_ns * clock) / NS_IN_S;

        let period_cycle = (request.period_ns * clock) / NS_IN_S;

        self.pwm_apply_state(PwmState::Enable(PwmRawState {
            polarity: request.polarity,
            period_cycle,
            duty_cycle,
        }))
    }
}
