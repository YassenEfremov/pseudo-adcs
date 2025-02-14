#![no_std]
#![no_main]

use core::mem::size_of;

use arduino_hal::prelude::*;
use panic_halt as _;
use pseudo_adcs_protocol::{commands::Command, data_format::MyFrame};


#[arduino_hal::entry]
fn main() -> ! {

    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led = pins.d13.into_output();
    let mut serial = arduino_hal::default_serial!(dp, pins, 115200);
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
    let mut y: i32 = 0;
    let mut z: i32 = 0;

    let mut target_x: i32 = 0;
    let mut target_y: i32 = 0;
    let mut target_z: i32 = 0;

    let mut status_reg_buf: [u8; 1] = [0];

    let mut out_frame = MyFrame::new();
    let mut buf: [u8; 1] = [0x00];


    // i2c.write(ADDR, &[CTRL_REG2, 0x00]);
    // i2c.write(ADDR, &[CTRL_REG5, 0x00]);
    i2c.write(ADDR, &[CTRL_REG1, 0x0F]);

    loop {

        // if let Ok(command_number) = serial.read() {
        //     if let Ok(command) = Command::from(command_number) {
        //         match command {
        //             Command::SetAttitude => {
        //                 led.set_high();
        //                 arduino_hal::delay_ms(100);
        //                 led.set_low();
        //                 // let mut buffer = [0x00; size_of::<MyFrame>()];
        //                 // let mut tail: usize = 0;
        //                 // while let Ok(byte) = serial.read() {
        //                 //     if tail == buffer.len() {
        //                 //         break;
        //                 //     }
        //                 //     buffer[tail] = byte;
        //                 //     tail += 1;
        //                 // }
        //                 // let in_frame = MyFrame::from_fixed(&buffer);
        //                 // target_x = in_frame.get_x() as i32;
        //                 // target_y = in_frame.get_y() as i32;
        //                 // target_z = in_frame.get_z() as i32;
        //             },
        //         }
        //     }
        // }

        // // i2c.write_read(0x69, &[0x0F], &mut buf);
        // // i2c.write_read(0x69, &[0x20, 0x08], &mut buf);
        i2c.write_read(ADDR, &[STATUS_REG], &mut status_reg_buf);

        if status_reg_buf[0] & 0x08 == 0x08 {

            i2c.write_read(ADDR, &[OUT_X_L], &mut buf);
            // serial.write(buf[0]);
            out_frame.x_l = buf[0];
            i2c.write_read(ADDR, &[OUT_X_H], &mut buf);
            // serial.write(buf[0]);
            out_frame.x_h = buf[0];
            // let x_raw: i16 = (((x_h_buf[0] as u16) << 8) | ((x_l_buf[0]) as u16)) as i16;
            x += (out_frame.get_x() as i32)/200;

            i2c.write_read(ADDR, &[OUT_Y_L], &mut buf);
            // serial.write(buf[0]);
            out_frame.y_l = buf[0];
            i2c.write_read(ADDR, &[OUT_Y_H], &mut buf);
            // serial.write(buf[0]);
            out_frame.y_h = buf[0];
            // let y_raw: i16 = (((y_h_buf[0] as u16) << 8) | ((y_l_buf[0]) as u16)) as i16;
            y += (out_frame.get_y() as i32)/200;

            i2c.write_read(ADDR, &[OUT_Z_L], &mut buf);
            // serial.write(buf[0]);
            out_frame.z_l = buf[0];
            i2c.write_read(ADDR, &[OUT_Z_H], &mut buf);
            // serial.write(buf[0]);
            out_frame.z_h = buf[0];
            // let z_raw: i16 = (((z_h_buf[0] as u16) << 8) | ((z_l_buf[0]) as u16)) as i16;
            z += (out_frame.get_z() as i32)/200;

            for byte in out_frame.as_bytes() {
                serial.write_byte(byte);
            }

            // ufmt::uwriteln!(serial, "{} {} {} ({} {} {})",
            //                 x/50,
            //                 y/50,
            //                 z/50,
            //                 my_frame.get_x(), my_frame.get_y(), my_frame.get_z());
        }

        // arduino_hal::delay_ms(100);
    }
}
