// Define the register offsets as a struct
#[derive(Clone, Copy)]
struct PwmRegisters {
    cntr: usize,   // Counter register offset
    period: usize, // Period register offset
    duty: usize,   // Duty cycle register offset
    ctrl: usize,   // Control register offset
}

// Define the PWM data structure
struct RockchipPwmData {
    regs: PwmRegisters,
    prescaler: u32,
    supports_polarity: bool,
    supports_lock: bool,
    enable_conf: u32, // Bitmask for control register
}

// From pwm-rockchip.c (likely in a header or inline)
// Control register bit definitions
const PWM_CTRL_TIMER_EN: u32 = 1 << 0;    // Bit 0: Enable the PWM timer
const PWM_CTRL_OUTPUT_EN: u32 = 1 << 3;   // Bit 3: Enable PWM output

// PWM-specific flags (used in pwm_data_v3.enable_conf)
const PWM_ENABLE: u32 = 1 << 0;           // Bit 0: Enable PWM (same as PWM_CTRL_TIMER_EN)
const PWM_CONTINUOUS: u32 = 1 << 1;       // Bit 1: Continuous mode (vs one-shot)
const PWM_DUTY_POSITIVE: u32 = 1 << 3;    // Bit 3: Duty cycle is positive (active-high)
const PWM_DUTY_NEGATIVE: u32 = 0 << 3;    // Bit 3: Duty cycle is negative (active-low, no shift needed)
const PWM_INACTIVE_NEGATIVE: u32 = 0 << 4;// Bit 4: Inactive state is negative (low)
const PWM_INACTIVE_POSITIVE: u32 = 1 << 4;// Bit 4: Inactive state is positive (high)
const PWM_POLARITY_MASK: u32 = PWM_DUTY_POSITIVE | PWM_INACTIVE_POSITIVE; // Mask for polarity bits (3 and 4)
const PWM_OUTPUT_LEFT: u32 = 0 << 5;      // Bit 5: Output left-aligned (default, no shift)
const PWM_LOCK_EN: u32 = 1 << 6;          // Bit 6: Lock PWM configuration
const PWM_LP_DISABLE: u32 = 0 << 8;       // Bit 8: Disable low-power mode (0 = disabled)

// The value I am guessing the clock rate to be
// If the div is touched, the driver may not work
// As the clock rate could be different from this value
const RK3399_PWM_CLOCKRATE: u32 = 24_000_000;

// The pwm_data_v3 equivalent
const PWM_DATA: RockchipPwmData = RockchipPwmData {
    regs: PwmRegisters {
        cntr: 0x00,
        period: 0x04,
        duty: 0x08,
        ctrl: 0x0c,
    },
    prescaler: 1,
    supports_polarity: true,
    supports_lock: true,
    enable_conf: PWM_OUTPUT_LEFT | PWM_LP_DISABLE | PWM_ENABLE | PWM_CONTINUOUS,
};

