# B-L4S5I-IOT01A Iot Node Minimum Viable Project

A base starting point for projects using the STM B-L4S5I-IOT01A Iot Node dev board: https://www.st.com/en/evaluation-tools/b-l4s5i-iot01a.html

# useful links
https://github.com/stm32-rs/stm32-rs


## I2C2 Addresses
I2C2_SCL : PB10
I2C2_SDA : PB11
|Modules |Description |SAD[0:6]+R/W |I2C write address |I2C read address
--- | --- | --- | --- | ---
|HTS221 |Capacitive digital sensor for relativehumidity and temperatur |1011111x |0xBE |0xBF
|LIS3MDL |3-axis magnetometer |0011110x |0x3C |0x3D
|LPS22HB |MEMS nano pressure sensor |1011101x |0xBA |0xBB
|LSM6DSL |3D accelerometer and3D gyroscope |1101010x |0xD4 |0xD5
|VL53L0X |Time-of-Flight ranging and gesturedetection sensor |0101001x |0x52 |0x53
|ST25DV04K |Dynamic NFC/RFID Tag IC |1010x11x |0xAE for systemarea0xA6 for usermemory |0xAF for systemarea0xA7 for usermemory
|STSAFE-A110 |Highly secure solution |0100000x |0x40 |0x41


## SPI CSN Assignments
|Module |Description |Pin Name
--- | --- | ---
|ISM43362-SPI3_CSN |WiFi |PE0
|SPSGRF-915-SPI3_CSN |Sub Ghz Module |PB5
|SPBTLE-RF-SPI3_CSN |Bluetooth LE |PD13
|SPI3_SCK |MCU SCK |PC10
|SPI3_MISO |MCU MISO |PC11
|SPI3_MOSI |MCU MOSI |PC12

## Module Reset Pins
|Module |Pin Name
--- | ---
|SPBTLE-RF-RST |PA8
|ISM43362-RST |PE8
|STSAFE-A110-RESET |PD7

## QSPI NOR Flash Memory Pins
|Module Pin |Pin Name
--- | ---
|QUADSPI_CLK |PE10
|QUADSPI_NCS |PE11
|QUADSPI_BK1_IO0 |PE12
|QUADSPI_BK1_IO1 |PE13
|QUADSPI_BK1_IO2 |PE14
|QUADSPI_BK1_IO3 |PE15

## MEMS Microphone
|Signal/Label |Pin Name
-- | --
|DFSDM1_DATIN2 |PE7
|DFSDM1_CKOUT |PE9


