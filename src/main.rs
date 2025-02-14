#![no_std]
#![no_main]

use core::mem::size_of;

use arduino_hal::{
    default_serial, delay_ms, pins, prelude::*, I2c, Peripherals
};
use l3gd20::L3GD20;
use panic_halt as _;
use pseudo_adcs_protocol::{commands::Command, data_format::MyFrame};

mod l3gd20;


#[arduino_hal::entry]
fn main() -> ! {

    let dp = Peripherals::take().unwrap();
    let pins = pins!(dp);

    let mut serial = default_serial!(dp, pins, 115200);
    let mut i2c = I2c::new(
        dp.TWI,
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        50000,
    );

    let mut l3gd20 = L3GD20::from_i2c(i2c);
    let mut led = pins.d13.into_output();

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut z: i32 = 0;

    let mut target_x: i32 = 0;
    let mut target_y: i32 = 0;
    let mut target_z: i32 = 0;

    // let mut out_frame = MyFrame::new();

    l3gd20.enable();

    loop {

        if let Ok(command_number) = serial.read() {
            if let Ok(command) = Command::from(command_number) {
                match command {
                    Command::SetAttitude => {
                        led.set_high();
                        delay_ms(100);
                        led.set_low();
                        // let mut buffer = [0x00; size_of::<MyFrame>()];
                        // let mut tail: usize = 0;
                        // while let Ok(byte) = serial.read() {
                        //     if tail == buffer.len() {
                        //         break;
                        //     }
                        //     buffer[tail] = byte;
                        //     tail += 1;
                        // }
                        // let in_frame = MyFrame::from_fixed(&buffer);
                        // target_x = in_frame.get_x() as i32;
                        // target_y = in_frame.get_y() as i32;
                        // target_z = in_frame.get_z() as i32;
                    },
                }
            }
        }


        if l3gd20.is_data_ready() {

            x += (l3gd20.get_x() as i32)/200;
            y += (l3gd20.get_y() as i32)/200;
            z += (l3gd20.get_z() as i32)/200;

            for byte in [
                l3gd20.read_x_h(), l3gd20.read_x_l(),
                l3gd20.read_y_h(), l3gd20.read_y_l(),
                l3gd20.read_z_h(), l3gd20.read_z_l()
            ] {
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
