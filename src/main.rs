#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    // let mut led = pins.d13.into_output();
    // loop {
    //     led.toggle();
    //     arduino_hal::delay_ms(1000);
    // }

    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut i2c = arduino_hal::I2c::new(
        dp.TWI,
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        50000,
    );

    // ufmt::uwriteln!(&mut serial, "Write direction test:\r").unwrap_infallible();
    // i2c.i2cdetect(&mut serial, arduino_hal::i2c::Direction::Write)
    //     .unwrap_infallible();
    // ufmt::uwriteln!(&mut serial, "\r\nRead direction test:\r").unwrap_infallible();
    // i2c.i2cdetect(&mut serial, arduino_hal::i2c::Direction::Read)
    //     .unwrap_infallible();

    const ADDR: u8 = 0x69;
    const CTRL_REG1: u8 = 0x20;
    const CTRL_REG2: u8 = 0x21;
    const CTRL_REG5: u8 = 0x24;
    const STATUS_REG: u8 = 0x27;
    const OUT_X_L: u8 = 0x28;
    const OUT_X_H: u8 = 0x29;
    const OUT_Y_L: u8 = 0x2A;
    const OUT_Y_H: u8 = 0x2B;
    const OUT_Z_L: u8 = 0x2C;
    const OUT_Z_H: u8 = 0x2D;

    let mut x: i32 = 0;
    let mut x_l_buf: [u8; 1] = [0x00];
    let mut x_h_buf: [u8; 1] = [0x00];

    let mut y: i32 = 0;
    let mut y_l_buf: [u8; 1] = [0x00];
    let mut y_h_buf: [u8; 1] = [0x00];

    let mut z: i32 = 0;
    let mut z_l_buf: [u8; 1] = [0x00];
    let mut z_h_buf: [u8; 1] = [0x00];

    let mut status_reg: [u8; 1] = [0];


    // i2c.write(ADDR, &[CTRL_REG2, 0x00]);
    // i2c.write(ADDR, &[CTRL_REG5, 0x00]);
    i2c.write(ADDR, &[CTRL_REG1, 0x0F]);

    loop {
        // match i2c.read(69, &mut buf) {
        //     Ok(_) => serial.write_byte(buf[0]),
        //     Err(_) => {serial.write_str("err"); ()}
        // }

        // i2c.write_read(0x69, &[0x0F], &mut buf);
        // i2c.write_read(0x69, &[0x20, 0x08], &mut buf);
        i2c.write_read(ADDR, &[STATUS_REG], &mut status_reg);

        if status_reg[0] & 0x08 == 0x08 { 
            i2c.write_read(ADDR, &[OUT_X_L], &mut x_l_buf);
            i2c.write_read(ADDR, &[OUT_X_H], &mut x_h_buf);
            let x_raw: i16 = (((x_h_buf[0] as u16) << 8) | ((x_l_buf[0]) as u16)) as i16;
            x += (x_raw as i32)/100;

            i2c.write_read(ADDR, &[OUT_Y_L], &mut y_l_buf);
            i2c.write_read(ADDR, &[OUT_Y_H], &mut y_h_buf);
            let y_raw: i16 = (((y_h_buf[0] as u16) << 8) | ((y_l_buf[0]) as u16)) as i16;
            y += (y_raw as i32)/100;

            i2c.write_read(ADDR, &[OUT_Z_L], &mut z_l_buf);
            i2c.write_read(ADDR, &[OUT_Z_H], &mut z_h_buf);
            let z_raw: i16 = (((z_h_buf[0] as u16) << 8) | ((z_l_buf[0]) as u16)) as i16;
            z += (z_raw as i32)/100;

            ufmt::uwriteln!(serial, "{} {} {} ({} {} {})", x/10, y/10, z/10, x_raw, y_raw, z_raw);
        }

        // serial.write_byte(buf[0]);
        // if buf[0] == 0b11010011 {
        //     serial.write_str("yes");
        // }

        arduino_hal::delay_ms(100);
    }
}
