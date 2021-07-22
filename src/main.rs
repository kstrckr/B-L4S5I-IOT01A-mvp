#![deny(unsafe_code)]
#![no_main]
#![no_std]


use embedded_hal::spi::{Mode, Phase, Polarity};
use hal::pac::dbgmcu::{apb1_fzr1, apb2_fzr};
use hal::pac::gpioa::pupdr;
use hal::pac::gpiob::otyper;
use hal::pac::gpioc::afrl;
use hal::rcc::APB2;
use panic_halt as _;
use cortex_m_rt as rt;
use rt::entry;
use stm32l4xx_hal as hal;
use hal::prelude::*;
use hal::i2c::I2c;
use hal::spi::Spi;
use hal::rcc::{Rcc, Clocks};
struct I2cSensor {
    sub_addr: u8,
    write_addr: u8,
    read_addr: u8,
}

struct Sensors {
    temp_humidity: I2cSensor,
    magnetometer: I2cSensor,
    barometer: I2cSensor,
    accel_gyro: I2cSensor,
    time_of_flight: I2cSensor,
    // nfc: I2cSensor,
    stsafe: I2cSensor,
}
#[entry]
fn main() -> ! {

    let i2cSensors = Sensors {
        temp_humidity: I2cSensor {
            sub_addr: 0b101_1111,
            write_addr: 0xBE,
            read_addr: 0xBF,
        },
        magnetometer: I2cSensor {
            sub_addr: 0b0011110,
            write_addr: 0x3C,
            read_addr: 0x3D,
        },
        barometer: I2cSensor {
            sub_addr: 0b1011101,
            write_addr: 0xBA,
            read_addr: 0xBB,
        },
        accel_gyro: I2cSensor {
            sub_addr: 0b1101010,
            write_addr: 0xD4,
            read_addr: 0xD5,
        },
        time_of_flight: I2cSensor {
            sub_addr: 0b0101001,
            write_addr: 0x52,
            read_addr: 0x53,
        },
        // nfc: I2cSensor {
        //     sub_addr: 0b1010x11,
        //     write_addr: 0xBE,
        //     read_addr: 0xBF,
        // },
        stsafe: I2cSensor {
            sub_addr: 0b0100000,
            write_addr: 0x40,
            read_addr: 0x41,
        },
    };


    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = hal::stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain(); // .constrain();
    let mut rcc = dp.RCC.constrain();
    let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);

    let clocks = rcc.cfgr.hclk(8.mhz()).freeze(&mut flash.acr, &mut pwr);
    let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

    // sck pc10
    // miso pc11
    // mosi pc12

    // subg_csn pb5

    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb2);
    let mut cs = gpiob.pb5.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    let mut gpioc = dp.GPIOC.split(&mut rcc.ahb2);
    let mut sck = gpioc.pc10.into_af6(&mut gpioc.moder, &mut gpioc.afrh);
    let mut miso = gpioc.pc11.into_af6(&mut gpioc.moder, &mut gpioc.afrh);
    let mut mosi = gpioc.pc12.into_af6(&mut gpioc.moder, &mut gpioc.afrh);

    let mut spi = Spi::spi3(
        dp.SPI3,
        (sck, miso, mosi),
        Mode {
            polarity: Polarity::IdleLow,
            phase: Phase::CaptureOnFirstTransition,
        },
        hal::time::KiloHertz(2000),
        clocks,
        &mut rcc.apb1r1,
    );


    cs.set_high().unwrap();
    delay.delay_ms(1000_u32);
    cs.set_low().unwrap();

    // the radio responsds with 2 bytes of status info before responding to the read or command
    let words = &mut [0x01, 0xA3, 0];
    // 0xf1 res b"R\a\363\000\001"
    let val = spi.transfer(words).ok().unwrap();
    let word1 = words[0].to_be_bytes();
    let word2 = words[1].to_be_bytes();
    let word3 = words[2].to_be_bytes();
    loop {
        continue;
    }
}