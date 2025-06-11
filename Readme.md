# PWM Fan Controller Driver

## 1. Overview

This document describes the driver for controlling a system fan's speed using Pulse-Width Modulation (PWM). The driver abstracts the low-level hardware details and provides a simple, high-level interface for setting the fan to several predefined speed levels.

### What is PWM?

Pulse-Width Modulation (PWM) is a technique for getting analog-like results with digital means. Instead of outputting a continuous analog voltage, a PWM signal switches between fully on (e.g., 5V) and fully off (0V) at a very high frequency.

The key concept in PWM is the **duty cycle**, which represents the percentage of time the signal is "on" during one cycle.

*   A **0% duty cycle** means the signal is always off.
*   A **100% duty cycle** means the signal is always on.
*   A **50% duty cycle** means the signal is on for half the time and off for the other half.

By varying the duty cycle, we can control the average power delivered to a device. For a fan, a higher duty cycle results in a faster spin speed, and a lower duty cycle results in a slower speed.



## 2. API Reference

The driver exposes a single function to control the fan.

---

### `config_fan_speed()`

Sets the fan speed by configuring the underlying PWM duty cycle.

#### Signature
```rust
fn config_fan_speed(speed: FanSpeed) -> Result<(), PwmError>
```

#### Description
This function takes a `FanSpeed` enum variant as an argument and attempts to set the hardware PWM controller's duty cycle to the corresponding value. It provides a safe and simple way to manage fan thermals without needing to calculate raw PWM values.

#### Parameters

| Parameter | Type       | Description                                                  |
|-----------|------------|--------------------------------------------------------------|
| `speed`   | `FanSpeed` | The desired, predefined speed level for the fan. See [FanSpeed Enum](#fanspeed-enum) for details. |

#### Returns
*   `Ok(())`: The fan speed was successfully configured.
*   `Err(PwmError)`: An error occurred while trying to set the fan speed.

---

## 3. Data Types

### `FanSpeed` Enum

The `FanSpeed` enum represents the five supported speed levels for the fan. It is defined as a `u8` integer, where the raw value corresponds directly to the 8-bit value written to the PWM duty cycle register (0-255).

#### Definition
```rust
#[repr(u8)]
pub enum FanSpeed {
    Stopped = 0,
    Low     = 64,
    Medium  = 128,
    High    = 192,
    Full    = 255,
}
```

#### Variants

| Variant | Raw Value (`u8`) | Duty Cycle | Description                                                |
|---------|------------------|------------|------------------------------------------------------------|
| `Stopped`| `0`              | 0%         | The fan is completely stopped (no power).                  |
| `Low`    | `64`             | ~25.1%     | A low, quiet speed for minimal cooling under light load.   |
| `Medium` | `128`            | ~50.2%     | A moderate speed suitable for typical thermal loads.       |
| `High`   | `192`            | ~75.3%     | A high speed for demanding tasks or warm environments.     |
| `Full`   | `255`            | 100%       | The fan runs at its maximum possible speed for critical cooling. |