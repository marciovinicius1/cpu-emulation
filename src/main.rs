struct CPU {
    registers: [u8; 16],
    position_in_memory: usize,
    memory: [u8; 0x1000],
    // stack: [u16; 16],
    // stack_pointer: usize
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;
        // return 2 bytes that is the opcode size
        op_byte1 << 8 | op_byte2
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.position_in_memory += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            match (c, x, y, d) {
                (0, 0, 0, 0) => return,
                (0x8, _ , _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        // the last register 0xF indicates that an operation overflowed the u8 register size
        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }
}

fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
        // stack: [0; 16],
        // stack_pointer: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10;
    cpu.registers[3] = 10;

    let mem = &mut cpu.memory;
    // setting 2 bytes opcode/line
    // 0x8014 -> add the value in 0 (5) the value in register 1 (10)
    mem[0] = 0x80; mem[1] = 0x14;
    // 0x8024 -> add the value in 0 (15) the value in register 2 (10)
    mem[2] = 0x80; mem[3] = 0x24;
    // 0x8034 -> add the value in 0 (25) the value in register 3 (10)
    mem[4] = 0x80; mem[5] = 0x34;

    cpu.run();
    assert_eq!(cpu.registers[0], 35);
    println!("5 + 10 + 10 + 10 = {}", cpu.registers[0]);

    // //create add_twice 3 opcodes
    // let add_twice: [u8; 6] = [
    //     //0x8014 -> add the value in 0 (5) the value in register 1 (10)
    //     0x80, 0x14,
    //     //0x8014 -> add the value in 0 (15) the value in register 1 (10)
    //     0x80, 0x14,
    //     //return
    //     0x00, 0xEE,
    // ];
    // //load add_twice function in memory
    // mem[0x100..0x106].copy_from_slice(&add_twice);
}



