struct CPU {
    /* All CHIP-8 opcodes are u16 */
    current_operation: u16,
    registers: [u8; 2],
}

fn main() {
    /* Initialise the CPU with zeros. */
    let mut cpu = CPU {
        current_operation: 0,
        registers: [0; 2],
    };

    /* 8 means two registers.
     * 0 maps to the the first register
     * 1 maps to the second register
     * 4 means addition.
     */
    cpu.current_operation = 0x8014;

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
}
