#![deny(unsafe_code)]
#![no_main]
#![no_std]


use panic_halt as _;
use cortex_m_rt as rt;
use rt::entry;
use stm32l4xx_hal as hal;
use hal::prelude::*;
use hal::i2c::I2c;

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

    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb2);
    let mut scl = gpiob.pb10.into_open_drain_output(&mut gpiob.moder, &mut gpiob.otyper);
    let mut sda = gpiob.pb11.into_open_drain_output(&mut gpiob.moder, &mut gpiob.otyper);


    scl.internal_pull_up(&mut gpiob.pupdr, true);
    let scl = scl.into_af4(&mut gpiob.moder, &mut gpiob.afrh);
    sda.internal_pull_up(&mut gpiob.pupdr, true);
    let sda = sda.into_af4(&mut gpiob.moder, &mut gpiob.afrh);

    let mut i2c = I2c::i2c2(dp.I2C2, (scl, sda), 100u32.khz(), clocks, &mut rcc.apb1r1);

    // let mut timer = Delay::new(cp.SYST, clocks);

    let mut p_buffer = [0u8; 3];
    let mut t_buffer = [0u8; 2];
    i2c.write(i2cSensors.barometer.sub_addr, &[0x11, 0b0000_0001]).unwrap();
    i2c.write_read(i2cSensors.barometer.sub_addr, &[0x2A, 0x29, 0x28], &mut p_buffer).unwrap();
    i2c.write_read(i2cSensors.barometer.sub_addr, &[0x2C, 0x2B], &mut t_buffer).unwrap();
    loop {

    }
}