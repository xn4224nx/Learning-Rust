struct CPU {
    registers: [u8; 16],
    prog_cntr: usize,
    memory: [u8; 0x1000],
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        let p = self.prog_cntr;
        let op_byte0 = self.memory[p] as u16;
        let op_byte1 = self.memory[p + 1] as u16;

        /* Combine the two values with a bitwise OR. */
        return op_byte0 << 8 | op_byte1;
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg0 = self.registers[x as usize];
        let arg1 = self.registers[y as usize];

        /* Overflowing add return a u8 and a bool if there is an overflow. */
        let (val, overflow) = arg0.overflowing_add(arg1);
        self.registers[x as usize] = val;

        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }
}

fn main() {}
