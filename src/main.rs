#![no_std]
#![no_main]

use core::mem::size_of;

use arduino_hal::prelude::*;
use panic_halt as _;

// my frame format:
//
//    0                   1
//    0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5
//   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//   |  X high bits  |  X low bits   |
//   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//   |  Y high bits  |  Y low bits   |
//   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//   |  Y high bits  |  Z low bits   |
//   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

struct MyFrame {
    x_h: u8,
    x_l: u8,
    y_h: u8,
    y_l: u8,
    z_h: u8,
    z_l: u8,
}

impl MyFrame {
    fn new() -> Self {
        Self {
            x_h: 0, x_l: 0,
            y_h: 0, y_l: 0,
            z_h: 0, z_l: 0
        }
    }

    fn from(buf: [u8; size_of::<Self>()]) -> Self {
        Self {
            x_h: buf[0], x_l: buf[1],
            y_h: buf[2], y_l: buf[3],
            z_h: buf[4], z_l: buf[5]
        }
    }

    fn get_x(&self) -> i16 {
        (((self.x_h as u16) << 8) | (self.x_l as u16)) as i16
    }

    fn get_y(&self) -> i16 {
        (((self.y_h as u16) << 8) | (self.y_l as u16)) as i16
    }

    fn get_z(&self) -> i16 {
        (((self.z_h as u16) << 8) | (self.z_l as u16)) as i16
    }

    fn as_bytes(&self) -> [u8; size_of::<Self>()] {
        return [
            self.x_h, self.x_l,
            self.y_h, self.y_l,
            self.z_h, self.z_l,
        ];
    }
}

#[arduino_hal::entry]
fn main() -> ! {

    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 115200);
    let mut led = pins.d13.into_output();

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

    let mut status_reg_buf: [u8; 1] = [0];

    let mut my_frame = MyFrame::new();
    let mut buf: [u8; 1] = [0x00];


    // i2c.write(ADDR, &[CTRL_REG2, 0x00]);
    // i2c.write(ADDR, &[CTRL_REG5, 0x00]);
    i2c.write(ADDR, &[CTRL_REG1, 0x0F]);

    loop {
        // // i2c.write_read(0x69, &[0x0F], &mut buf);
        // // i2c.write_read(0x69, &[0x20, 0x08], &mut buf);
        i2c.write_read(ADDR, &[STATUS_REG], &mut status_reg_buf);

        if status_reg_buf[0] & 0x08 == 0x08 {

            i2c.write_read(ADDR, &[OUT_X_L], &mut buf);
            // serial.write(buf[0]);
            my_frame.x_l = buf[0];
            i2c.write_read(ADDR, &[OUT_X_H], &mut buf);
            // serial.write(buf[0]);
            my_frame.x_h = buf[0];
            // let x_raw: i16 = (((x_h_buf[0] as u16) << 8) | ((x_l_buf[0]) as u16)) as i16;
            // x += (my_frame.get_x() as i32)/200;

            i2c.write_read(ADDR, &[OUT_Y_L], &mut buf);
            // serial.write(buf[0]);
            my_frame.y_l = buf[0];
            i2c.write_read(ADDR, &[OUT_Y_H], &mut buf);
            // serial.write(buf[0]);
            my_frame.y_h = buf[0];
            // let y_raw: i16 = (((y_h_buf[0] as u16) << 8) | ((y_l_buf[0]) as u16)) as i16;
            // y += (my_frame.get_y() as i32)/200;

            i2c.write_read(ADDR, &[OUT_Z_L], &mut buf);
            // serial.write(buf[0]);
            my_frame.z_l = buf[0];
            i2c.write_read(ADDR, &[OUT_Z_H], &mut buf);
            // serial.write(buf[0]);
            my_frame.z_h = buf[0];
            // let z_raw: i16 = (((z_h_buf[0] as u16) << 8) | ((z_l_buf[0]) as u16)) as i16;
            // z += (my_frame.get_z() as i32)/200;

            for byte in my_frame.as_bytes() {
                serial.write_byte(byte);
            }
            // ufmt::uwriteln!(serial, "{} {} {} ({} {} {})",
            //                 x/50,
            //                 y/50,
            //                 z/50,
            //                 my_frame.get_x(), my_frame.get_y(), my_frame.get_z());
        }

        // let res = serial.read();
        // if let Ok(c) = res {
        //     if c as char == 'Z' {
        //         led.toggle();
        //     }
        // }
        // arduino_hal::delay_ms(100);
    }
}
