#![no_std]
#![no_main]

use core::mem::size_of;

use arduino_hal::{
    default_serial, delay_ms, pins, prelude::*, I2c, Peripherals
};
use l3gd20::L3GD20;
use panic_halt as _;
use pseudo_adcs_protocol::message::{Message, MessagePayload, PushState, SAT};

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

    let mut target_attitude_achieved = true;

    l3gd20.enable();

    loop {
        // if let Ok(b) = serial.read() {
        //     ufmt::uwriteln!(serial, "{}", b);
        // }
        if let Ok(command_number) = serial.read() {
            let mut sat_message = Message::new();
            sat_message.push_byte(command_number);
            while let PushState::Continue = sat_message.push_byte(
                serial.read_byte()
            ) {}

            if let Some(MessagePayload::SAT(sat_payload)) = sat_message.payload {
                target_x = sat_payload.get_x() as i32;
                target_y = sat_payload.get_y() as i32;
                target_z = sat_payload.get_z() as i32;
                target_attitude_achieved = false;
            }
        }


        if l3gd20.is_data_ready() {

            x += (l3gd20.get_x() as i32)/500;
            y += (l3gd20.get_y() as i32)/500;
            z += (l3gd20.get_z() as i32)/500;

            serial.write_byte(0x01);
            for byte in [
                l3gd20.read_x_h(), l3gd20.read_x_l(),
                l3gd20.read_y_h(), l3gd20.read_y_l(),
                l3gd20.read_z_h(), l3gd20.read_z_l()
            ] {
                serial.write_byte(byte);
            }

            // for byte in [
            //     0x00, 0x0a,
            //     0x00, 0x0a,
            //     0x00, 0x0a
            // ] {
            //     serial.write_byte(byte);
            // }

            // ufmt::uwriteln!(serial, "{} {} {} ({} {} {})",
            //                 x/20,
            //                 y/20,
            //                 z/20,
            //                 l3gd20.get_x(), l3gd20.get_y(), l3gd20.get_z());
        }

        let old_state = target_attitude_achieved;
        target_attitude_achieved =
            x/20 == target_x && y/20 == target_y && z/20 == target_z;      
        if target_attitude_achieved != old_state {
            serial.write_byte(0x03);
            led.set_high();
            delay_ms(100);
            led.set_low();
        }
    }
}
