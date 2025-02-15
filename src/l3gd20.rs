use arduino_hal::{prelude::*, I2c};

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

enum L3GD20Bus {
    I2C(I2c),
    // SPI(Spi),
}

pub struct L3GD20 {
    bus: L3GD20Bus,
}

impl L3GD20 {
    pub fn from_i2c(i2c: I2c) -> Self {
        L3GD20 {
            bus: L3GD20Bus::I2C(i2c)
            
        }
    }

    // fn from_spi(spi: Spi) -> Self {
    //     L3GD20 {
    //         bus: L3GD20Bus::SPI(spi)
    //     }
    // }

    pub fn enable(&mut self) -> Result<(), arduino_hal::i2c::Error> {
        match &mut self.bus {
            L3GD20Bus::I2C(i2c) => i2c.write(ADDR, &[CTRL_REG1, 0x0F]),
            // L3GD20Bus::SPI(spi) => ()
        }
    }

    fn read_reg(&mut self, reg_addr: u8) -> u8 {
        match &mut self.bus {
            L3GD20Bus::I2C(i2c) => {
                let mut buffer = [0];
                i2c.write_read(ADDR, &[reg_addr], &mut buffer);
                buffer[0]
            },
            // L3GD20Bus::SPI(spi) => ()
        }
    }

	pub fn is_data_ready(&mut self) -> bool {
		self.read_reg(STATUS_REG) & 0x08 == 0x08
	}

    pub fn read_x_h(&mut self) -> u8 {
        match &mut self.bus {
            L3GD20Bus::I2C(i2c) => {
				let mut buffer: [u8; 1] = [0x00];
				i2c.write_read(ADDR, &[OUT_X_H], &mut buffer);
				buffer[0]
            },
            // L3GD20Bus::SPI(spi) => ()
        }
    }

    pub fn read_x_l(&mut self) -> u8 {
        match &mut self.bus {
            L3GD20Bus::I2C(i2c) => {
				let mut buffer: [u8; 1] = [0x00];
				i2c.write_read(ADDR, &[OUT_X_L], &mut buffer);
				buffer[0]
            },
            // L3GD20Bus::SPI(spi) => ()
        }
	}

    pub fn get_x(&mut self) -> i16 {
		// (((self.read_x_h() as u16) << 8) | (self.read_x_l() as u16)) as u16
		i16::from_be_bytes([self.read_x_h(), self.read_x_l()])
    }

    pub fn read_y_h(&mut self) -> u8 {
        match &mut self.bus {
            L3GD20Bus::I2C(i2c) => {
				let mut buffer: [u8; 1] = [0x00];
				i2c.write_read(ADDR, &[OUT_Y_H], &mut buffer);
				buffer[0]
            },
            // L3GD20Bus::SPI(spi) => ()
        }
    }

    pub fn read_y_l(&mut self) -> u8 {
        match &mut self.bus {
            L3GD20Bus::I2C(i2c) => {
				let mut buffer: [u8; 1] = [0x00];
				i2c.write_read(ADDR, &[OUT_Y_L], &mut buffer);
				buffer[0]
            },
            // L3GD20Bus::SPI(spi) => ()
        }
	}

    pub fn get_y(&mut self) -> i16 {
		// (((self.read_y_h() as u16) << 8) | (self.read_y_l() as u16)) as u16
		i16::from_be_bytes([self.read_y_h(), self.read_y_l()])
    }

    pub fn read_z_h(&mut self) -> u8 {
        match &mut self.bus {
            L3GD20Bus::I2C(i2c) => {
				let mut buffer: [u8; 1] = [0x00];
				i2c.write_read(ADDR, &[OUT_Z_H], &mut buffer);
				buffer[0]
            },
            // L3GD20Bus::SPI(spi) => ()
        }
    }

    pub fn read_z_l(&mut self) -> u8 {
        match &mut self.bus {
            L3GD20Bus::I2C(i2c) => {
				let mut buffer: [u8; 1] = [0x00];
				i2c.write_read(ADDR, &[OUT_Z_L], &mut buffer);
				buffer[0]
            },
            // L3GD20Bus::SPI(spi) => ()
        }
	}

    pub fn get_z(&mut self) -> i16 {
		// (((self.read_z_h() as u16) << 8) | (self.read_z_l() as u16)) as u16
		i16::from_be_bytes([self.read_z_h(), self.read_z_l()])
    }

    // for filters
    // i2c.write(ADDR, &[CTRL_REG2, 0x00]);
    // i2c.write(ADDR, &[CTRL_REG5, 0x00]);
	// i2c.write_read(ADDR, &[0x0F], &mut buf);
	// i2c.write_read(ADDR, &[0x20, 0x08], &mut buf);
}
