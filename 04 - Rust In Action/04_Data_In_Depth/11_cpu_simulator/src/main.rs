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

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.prog_cntr += 2;

            /* Decoding the op code. */
            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            /* Short circuit the function to terminate execution
             *  when 0x0000 is encountered. */
            match (c, x, y, d) {
                (0, 0, 0, 0) => {
                    return;
                }
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }
}

fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        prog_cntr: 0,
    };

    /* Initialise the registers. */
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10;
    cpu.registers[3] = 10;

    let mem = &mut cpu.memory;

    /* Add register 1 to register 0, via opcode 0x8014 */
    mem[0] = 0x80;
    mem[1] = 0x14;

    /* Add register 2 to register 0, via opcode 0x8024 */
    mem[2] = 0x80;
    mem[3] = 0x24;

    /* Add register 3 to register 0, via opcode 0x8034 */
    mem[4] = 0x80;
    mem[5] = 0x34;

    cpu.run();

    assert_eq!(cpu.registers[0], 35);
    println!("5 + 10 + 10 + 10 = {}", cpu.registers[0]);
}
