pub struct  CPU {
    pub program_counter: u16,
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    memory: [u8; 0xFFFF]
}


#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
   Accumulator,
   Immediate,
   ZeroPage,
   ZeroPage_X,
   ZeroPage_Y,
   Relative,
   Absolute,
   Absolute_X,
   Absolute_Y,
   Indirect,
   Indirect_X,
   Indirect_Y,
   Implicit
}

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0,
            program_counter: 0,
            memory: [0; 0xFFFF],
        }
    }
        
    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.status = 0;
        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    fn set_flags(&mut self, operation_result: u8) {
        match operation_result {
            0 => { self.status |= 0b0000_0010; },
            _ => { self.status &= 0b1111_1101; }
        }

        if operation_result & 0b1000_0000  != 0 {
            self.status |= 0b1000_0000;
        }
        else {
            self.status &= 0b0111_1111;
        }
    }

    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    fn mem_read_u16(&self, addr: u16) -> u16 {
        let lo = self.mem_read(addr);
        let hi = self.mem_read(addr+1);
        u16::from_le_bytes([lo, hi])
    }

    fn mem_write_u16(&mut self, addr: u16, data: u16) {
        let data = data.to_le_bytes();
        self.mem_write(addr, data[0]);
        self.mem_write(addr+1, data[1]);
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000 .. (0x8000 + program.len())]
            .copy_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, 0x8000);
    }

    fn get_address(&mut self, addressing_mode: &AddressingMode) -> Option<u16> {
        let address: Option<u16>;
        match addressing_mode {
            AddressingMode::Implicit => address = None,
            AddressingMode::Immediate => address = Some(self.program_counter),
            AddressingMode::ZeroPage => {
                address = Some(self.mem_read(self.program_counter) as u16);
                self.program_counter += 1;
            },
            AddressingMode::ZeroPage_X => {
                address = Some(self.mem_read(self.program_counter)
                    .wrapping_add(self.register_x) as u16);
                self.program_counter += 1;
            },
            AddressingMode::ZeroPage_Y => {
                address = Some(self.mem_read(self.program_counter)
                    .wrapping_add(self.register_y) as u16);
                self.program_counter += 1;
            },
            AddressingMode::Relative => {
                address = Some(self.program_counter.wrapping_add(
                    self.mem_read(self.program_counter) as u16
                ));
                self.program_counter += 1;
            },
            AddressingMode::Absolute => {
                address = Some(self.mem_read_u16(self.program_counter));
                self.program_counter += 2;
            },
            AddressingMode::Absolute_X => {
                address = Some(self.mem_read_u16(self.program_counter)
                    .wrapping_add(self.register_x as u16));
                self.program_counter += 2;
            },
            AddressingMode::Absolute_Y => {
                address = Some(self.mem_read_u16(self.program_counter)
                    .wrapping_add(self.register_y as u16));
                self.program_counter += 2;
            },
            AddressingMode::Indirect => {
                let addr = self.mem_read_u16(self.program_counter);
                address = Some(self.mem_read_u16(addr));
                self.program_counter += 2;
            },
            AddressingMode::Indirect_X => {
                let addr = self.mem_read(self.program_counter)
                    .wrapping_add(self.register_x);
                address = Some(self.mem_read_u16(addr as u16));
                self.program_counter += 1;
            },
            AddressingMode::Indirect_Y => {
                let addr = self.mem_read(self.program_counter)
                    .wrapping_add(self.register_y);
                address = Some(self.mem_read_u16(addr as u16));
                self.program_counter += 1;
            },
            _ => todo!("finish"),
        }
        address
    }
    
    pub fn run(&mut self) {
        loop {
            let opscode = self.mem_read(self.program_counter);
            self.program_counter += 1;
            let result: u8;

            match opscode {
                0xA9 => {
                    let value = self.mem_read(self.program_counter);
                    self.register_a = value;
                    self.program_counter += 1;
                    result = self.register_a;
                }
                0xAA => {
                    self.register_x = self.register_a;
                    result = self.register_x;
                }
                0xE8 => { 
                    self.register_x = self.register_x.wrapping_add(1);
                    result = self.register_x;
                }
                0x00 => { return; },
                _ => {todo!("Implement more opcodes!");}
            }
            self.set_flags(result);

        }
    }

    pub fn load_and_run(&mut self, program:Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
use super::*;
 
    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xa9_lda_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xff, 0x00]);
        assert!(cpu.status & 0b1000_0000 != 0);
    }

    #[test]
    fn test_0xaa_tax_copy_a_to_x() {
        let mut cpu: CPU = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x50, 0xaa, 0x00]);
        assert_eq!(cpu.register_x, 0x50);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xaa_tax_zero_flag() {
        let mut cpu: CPU = CPU::new();
        cpu.register_a = 0x00;
        cpu.load_and_run(vec![0xaa, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xaa_tax_negative_flag() {
        let mut cpu: CPU = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xff, 0xaa, 0x00]);
        assert!(cpu.status & 0b1000_0000 != 0);
    }

    #[test]
    fn test_0xe8_inx_increment_x() {
        let mut cpu: CPU = CPU::new();
        cpu.load_and_run(vec![0xe8, 0x00]);
        assert_eq!(cpu.register_x, 0x01);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xe8_inx_zero_flag() {
        let mut cpu: CPU = CPU::new();

        /* LDA 0xff -> TAX -> INX */
        cpu.load_and_run(vec![0xa9, 0xff, 0xaa, 0xe8, 0x00]);
        assert_eq!(cpu.register_x, 0x00);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xe8_inx_wrap_around() {
        let mut cpu: CPU = CPU::new();

        /* LDA 0xff -> TAX -> INX -> INX */
        cpu.load_and_run(vec![0xa9, 0xff, 0xaa, 0xe8, 0xe8, 0x00]);
        assert_eq!(cpu.register_x, 0x01);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xe8_inx_negative_flag() {
        let mut cpu: CPU = CPU::new();

        /* LDA 0xfe -> TAX -> INX */
        cpu.load_and_run(vec![0xa9, 0xfe, 0xaa, 0xe8, 0x00]);
        assert_eq!(cpu.register_x, 0xff);
        assert!(cpu.status & 0b1000_0000 != 0);
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
        assert_eq!(cpu.register_x, 0xc1)
    }


    #[test]
    fn test_mem_write_read() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x5050, 0xEA);
        assert_eq!(0xEA, cpu.mem_read(0x5050));
    }

    #[test]
    fn test_load() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xa9, 0xa9, 0x50, 0x00]);
        assert_eq!(0xa9, cpu.mem_read(0x8000));
        assert_eq!(0xa9, cpu.mem_read(0x8001));
        assert_eq!(0x50, cpu.mem_read(0x8002));
        assert_eq!(0x00, cpu.mem_read(0x8003));
   }

    #[test]
    fn test_u16_read_write() {
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0xdead, 0xbeef);
        assert_eq!(0xbeef, cpu.mem_read_u16(0xdead));
    }

}
