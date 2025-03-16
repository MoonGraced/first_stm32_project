#![no_std]
#![no_main]
use panic_halt as _;
use stm32f4xx_hal::{
    gpio::{gpioa::PA0, gpioc::PC13, Edge, Input, Output, Pull, PushPull, gpiod},
    pac::Peripherals,
    prelude::*,
    timer::Timer2,
};
use stm32f4xx_hal::dwt::Instant;
use stm32f4xx_hal::hal_02::digital::v2::OutputPin;

#[cortex_m_rt::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(84.MHz()).freeze();

    let mut timer = dp.TIM1.counter_ms(&clocks);
    let gpioa = dp.GPIOA.split();
    let gpioc = dp.GPIOC.split();
    let gpiod = dp.GPIOD.split();

    // Кнопка на PC13 (например, на STM32F4 Discovery)
    let button = gpioa.pa0.into_pull_up_input();

    // Светодиоды на PD12-PD15
    let mut led1 = gpiod.pd12.into_push_pull_output();
    let mut led2 = gpiod.pd13.into_push_pull_output();
    let mut led3 = gpiod.pd14.into_push_pull_output();
    let mut led4 = gpiod.pd15.into_push_pull_output();
    led1.set_high(); // Включить светодиод
    let zero = Instant::now;
    loop {
        if timer.now() == let Instant {
            now: 0_u32
        } {
            led1.toggle();
        }
    }
}