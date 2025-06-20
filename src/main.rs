struct CPU {
    registers: [u8; 16],
    position_in_memory: usize,
    memory: [u8; 0x1000],
    stack: [u16; 16],
    stack_pointer: usize
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

            let nnn = opcode & 0x0FFF;
            // let kk = (opcode & 0x00FF) as u8
            
            match (c, x, y, d) {
                (0, 0, 0, 0) => return,
                (0, 0, 0xE, 0xE) => self.ret(),
                (0x2, _, _, _) => self.call(nnn),
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

    fn call(&mut self, addr:u16) {
        let sp = self.stack_pointer; //current index of stack
        let stack = &mut self.stack; //stack 
        
        if sp > stack.len() {
            panic!("Stack overflow")
        }
        
        stack[sp] = self.position_in_memory as u16; //current stack now is current memory position
        self.stack_pointer += 1; //stack pointer increase
        self.position_in_memory = addr as usize; //read code from a new place
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow")
        }

        self.stack_pointer -= 1; //decrement stack
        let call_addr = self.stack[self.stack_pointer]; // call addr
        self.position_in_memory = call_addr as usize;
    }
}

fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
        stack: [0; 16],
        stack_pointer: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    

    let mem = &mut cpu.memory;
    // setting 2 bytes opcode/line
    // 0x2100 -> call the function at 0x001 addr memory
    mem[0x000] = 0x21; mem[0x001] = 0x00;
    // 0x2100 -> call the function at 0x001 addr memory
    mem[0x002] = 0x21; mem[0x003] = 0x00;
    // 0x0000 -> set return addr with 0
    mem[0x004] = 0x00; mem[0x005] = 0x00;

    //fn add_twice()
    // 0x8014 -> add register y to register x
    mem[0x100] = 0x80; mem[0x101] = 0x14;
    // 0x8014 -> add register y to register x
    mem[0x102] = 0x80; mem[0x103] = 0x14;
    // return 
    mem[0x104] = 0x00; mem[0x105] = 0xEE;
    
    cpu.run();
    
    assert_eq!(cpu.registers[0], 45);
    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);
}



