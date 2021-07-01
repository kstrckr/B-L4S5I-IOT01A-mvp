#![deny(unsafe_code)]
#![no_main]
#![no_std]


use panic_halt as _;
use cortex_m_rt::entry;
use stm32l4xx_hal as hal;
use hal::prelude::*;
use hal::delay::Delay;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = hal::stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain(); // .constrain();
    let mut rcc = dp.RCC.constrain();
    let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);

    let clocks = rcc.cfgr.hclk(8.mhz()).freeze(&mut flash.acr, &mut pwr);

    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb2);
    let mut led1 = gpiob
        .pb14
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);
    let mut led2 = gpioa.pa5.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    let mut timer = Delay::new(cp.SYST, clocks);

    loop {
        // block!(timer.wait()).unwrap();
        timer.delay_ms(1000_u32);
        led2.set_low().ok();
        led1.set_high().ok();
        // block!(timer.wait()).unwrap();
        timer.delay_ms(1000_u32);
        led1.set_low().ok();
        led2.set_high().ok();
    }
}
