#![no_std] // Don't link the standard library
#![no_main] // Don't use the default entry point

use core::{convert::Infallible, hint};

use pwm_protocol::pwm_fan::{Fan, FanSpeed};
use pwm_rockchip_hal::pwm_rockchip::RockchipPwmHardware;
use sel4_microkit::{debug_println, protection_domain, Handler};

const FAN_PWM_ADDR: u32 = 0xff420010;

#[inline]
pub fn process_wait_unreliable(time_ns: u64) {
    for _ in 0..time_ns {
        hint::spin_loop(); // Use spin loop hint to reduce contention during the wait
    }
}

#[protection_domain(heap_size = 0x10000)]
fn init() -> HandlerImpl {
    let pwm: RockchipPwmHardware = RockchipPwmHardware::new(FAN_PWM_ADDR);

    let mut fan: Fan<RockchipPwmHardware> = Fan::new(pwm);

    debug_println!("Wait for a bit");

    match fan.config_fan_speed(FanSpeed::Full) {
        Ok(()) => {}
        Err(_) => core::panic!("Config fan speed failed"),
    }

    debug_println!("Test end");

    HandlerImpl {}
}

struct HandlerImpl {}

impl Handler for HandlerImpl {
    type Error = Infallible;
    fn notified(&mut self, channel: sel4_microkit::Channel) -> Result<(), Self::Error> {
        core::panic!("unexpected notification from channel {channel:?}")
    }

    fn protected(
        &mut self,
        channel: sel4_microkit::Channel,
        msg_info: sel4_microkit::MessageInfo,
    ) -> Result<sel4_microkit::MessageInfo, Self::Error> {
        core::panic!("unexpected protected procedure call from channel {channel:?} with msg_info={msg_info:?}")
    }

    fn fault(
        &mut self,
        child: sel4_microkit::Child,
        msg_info: sel4_microkit::MessageInfo,
    ) -> Result<Option<sel4_microkit::MessageInfo>, Self::Error> {
        core::panic!("unexpected fault from protection domain {child:?} with msg_info={msg_info:?}")
    }

    fn take_deferred_action(&mut self) -> Option<sel4_microkit::DeferredAction> {
        None
    }
}
