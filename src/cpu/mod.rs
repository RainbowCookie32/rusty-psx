use std::io::Read;
use std::fs::File;
use std::path::PathBuf;

use super::memory;

#[derive(PartialEq)]
pub enum CycleResult {
    None,
    Error,
    Success,
    Breakpoint,
}

#[derive(Clone, Copy)]
pub struct Instruction {
    value: u32
}

impl Instruction {
    pub fn new(value: u32) -> Instruction {
        Instruction {
            value: value,
        }
    }

    pub fn op(&self) -> u32 {
        (self.value >> 26) & 0x3F
    }

    pub fn rs(&self) -> u32 {
        (self.value >> 21) & 0x1F
    }

    pub fn rt(&self) -> u32 {
        (self.value >> 16) & 0x1F
    }

    pub fn rd(&self) -> u32 {
        (self.value >> 11) & 0x1F
    }

    pub fn shift(&self) -> u32 {
        (self.value >> 6) & 0x1F
    }

    pub fn function(&self) -> u32 {
        self.value & 0x3F
    }

    pub fn immediate(&self) -> u16 {
        (self.value & 0xFFFF) as u16
    }

    pub fn target(&self) -> u32 {
        self.value & 0x03FFFFFF
    }
}

pub struct Cpu {
    pub pc: u32,
    pub hi: u32,
    pub lo: u32,
    
    pub registers: Vec<u32>,
    pub cop0_registers: Vec<u32>,

    pub memory: memory::CpuMemory,

    pub next_instruction: Instruction,
    pub current_instruction: Instruction,

    pub branch_delay: bool,

    pub cpu_paused: bool,
    pub cpu_result: CycleResult,
    pub debugger_breakpoints: Vec<u32>,
}

impl Cpu {
    pub fn new() -> Cpu {
        
        let mut bios_data = Vec::with_capacity(512);
        let mut bios_file = File::open(PathBuf::from("SCPH1001.bin")).unwrap();

        bios_file.read_to_end(&mut bios_data).unwrap();

        let mut memory = memory::CpuMemory::new(bios_data);
        let first_op = memory.read_word(0xBFC00000);

        // The initial area of RAM has some values there that the BIOS seems to rely on.
        memory.write_word(0x00000000, 0x3C1A0000);
        memory.write_word(0x00000004, 0x275A0000);
        memory.write_word(0x00000008, 0x34000000);
        memory.write_word(0x0000000C, 0x00000000);
        
        Cpu {
            pc: 0xBFC00000,
            hi: 0,
            lo: 0,
            registers: vec![0; 32],

            cop0_registers: vec![0; 16],

            memory: memory,

            next_instruction: Instruction::new(first_op),
            current_instruction: Instruction::new(0),

            branch_delay: false,

            cpu_paused: true,
            cpu_result: CycleResult::None,
            debugger_breakpoints: Vec::new(),
        }
    }

    fn set_register(&mut self, idx: usize, value: u32) {
        self.registers[idx] = value;
        self.registers[0] = 0;
    }

    fn take_branch(&mut self) {
        let immediate = (self.current_instruction.immediate() << 2) as i16;
        let new_pc = (self.pc + 4).wrapping_add(immediate as u32);
        self.pc = new_pc;
        self.branch_delay = true;
    }

    fn fetch_instruction(&mut self) {
        self.current_instruction = Instruction::new(self.memory.read_word(self.pc));
        self.next_instruction = Instruction::new(self.memory.read_word(self.pc + 4));
    }

    pub fn run_instruction(&mut self) -> CycleResult {

        if self.cpu_result == CycleResult::Breakpoint {
            self.cpu_result = CycleResult::Success;
        }

        if self.branch_delay {
            self.current_instruction = self.next_instruction;
            self.branch_delay = false;
        }
        else {
            self.fetch_instruction();
        }

        match self.current_instruction.op() {

            0x00 => match self.current_instruction.function() {
                0x00 => self.sll(),
                0x01 => {},
                0x02 => self.srl(),
                0x03 => self.sra(),
                0x04 => self.sllv(),
                0x05 => {},
                0x06 => self.srlv(),
                0x07 => self.srav(),

                0x08 => self.jr(),
                0x09 => self.jalr(),
                0x0A => {},
                0x0B => {},
                0x0C => self.syscall(),
                0x0D => self.break_op(),
                0x0E => {},
                0x0F => {},

                0x10 => self.mfhi(),
                0x11 => self.mthi(),
                0x12 => self.mflo(),
                0x13 => self.mtlo(),
                0x14 => {},
                0x15 => {},
                0x16 => {},
                0x17 => {},

                0x18 => self.mult(),
                0x19 => self.multu(),
                0x1A => self.div(),
                0x1B => self.divu(),
                0x1C => {},
                0x1D => {},
                0x1E => {},
                0x1F => {},

                0x20 => self.add(),
                0x21 => self.addu(),
                0x22 => self.sub(),
                0x23 => self.subu(),
                0x24 => self.and(),
                0x25 => self.or(),
                0x26 => self.xor(),
                0x27 => self.nor(),

                0x28 => {},
                0x29 => {},
                0x2A => self.slt(),
                0x2B => self.sltu(),
                0x2C => {},
                0x2D => {},
                0x2E => {},
                0x2F => {},

                0x30 => {},
                0x31 => {},
                0x32 => {},
                0x33 => {},
                0x34 => {},
                0x35 => {},
                0x36 => {},
                0x37 => {},

                0x38 => {},
                0x39 => {},
                0x3A => {},
                0x3B => {},
                0x3C => {},
                0x3D => {},
                0x3E => {},
                0x3F => {},

                _=> {}
            }
            0x01 => match self.current_instruction.rt() {
                0x00 => self.bltz(),
                0x01 => self.bgez(),
                0x10 => self.bltzal(),
                0x11 => self.bgezal(),
                _ => panic!("Invalid argument in branch instruction"),
            }
            0x02 => self.j(),
            0x03 => self.jal(),
            0x04 => self.beq(),
            0x05 => self.bne(),
            0x06 => self.blez(),
            0x07 => self.bgtz(),

            0x08 => self.addi(),
            0x09 => self.addiu(),
            0x0A => self.slti(),
            0x0B => self.sltiu(),
            0x0C => self.andi(),
            0x0D => self.ori(),
            0x0E => self.xori(),
            0x0F => self.lui(),

            0x10 => self.cop0(),
            0x11 => {},
            0x12 => {},
            0x13 => {},
            0x14 => {},
            0x15 => {},
            0x16 => {},
            0x17 => {},

            0x18 => {},
            0x19 => {},
            0x1A => {},
            0x1B => {},
            0x1C => {},
            0x1D => {},
            0x1E => {},
            0x1F => {},

            0x20 => self.lb(),
            0x21 => self.lh(),
            0x22 => self.lwl(),
            0x23 => self.lw(),
            0x24 => self.lbu(),
            0x25 => self.lhu(),
            0x26 => self.lwr(),
            0x27 => {},

            0x28 => self.sb(),
            0x29 => self.sh(),
            0x2A => self.swl(),
            0x2B => self.sw(),
            0x2C => {},
            0x2D => {},
            0x2E => self.swr(),
            0x2F => {},

            0x30 => {},
            0x31 => {},
            0x32 => {},
            0x33 => {},
            0x34 => {},
            0x35 => {},
            0x36 => {},
            0x37 => {},

            0x38 => {},
            0x39 => {},
            0x3A => {},
            0x3B => {},
            0x3C => {},
            0x3D => {},
            0x3E => {},
            0x3F => {},

            _=> {}
        }

        for index in 0..self.debugger_breakpoints.len() {
            if self.pc == self.debugger_breakpoints[index] {
                self.cpu_result = CycleResult::Breakpoint;
                self.debugger_breakpoints.remove(index);
                return CycleResult::Breakpoint;
            }
        }

        self.pc = self.pc.wrapping_add(4);

        if self.cpu_result == CycleResult::Error {
            CycleResult::Error
        }
        else {
            CycleResult::Success
        }
    }



    // Load/Store instructions

    fn lb(&mut self) {
        let immediate = self.current_instruction.immediate() as i32;
        let address = immediate as u32 + self.registers[self.current_instruction.rs() as usize];
        let value = self.memory.read_byte(address) as i32;
        self.set_register(self.current_instruction.rt() as usize, value as u32);
    }

    fn lbu(&mut self) {
        let immediate = self.current_instruction.immediate() as i32;
        let address = immediate as u32 + self.registers[self.current_instruction.rs() as usize];
        let value = self.memory.read_byte(address) as u32;
        self.set_register(self.current_instruction.rt() as usize, value);
    }

    fn lh(&mut self) {
        let immediate = self.current_instruction.immediate() as i32;
        let address = immediate as u32 + self.registers[self.current_instruction.rs() as usize];
        let value = self.memory.read_halfword(address) as i32;
        self.set_register(self.current_instruction.rt() as usize, value as u32);
    }

    fn lhu(&mut self) {
        let immediate = self.current_instruction.immediate() as i32;
        let address = immediate as u32 + self.registers[self.current_instruction.rs() as usize];
        let value = self.memory.read_halfword(address) as u32;
        self.set_register(self.current_instruction.rt() as usize, value);
    }

    fn lw(&mut self) {
        let immediate = self.current_instruction.immediate() as i32;
        let address = immediate as u32 + self.registers[self.current_instruction.rs() as usize];
        let value = self.memory.read_word(address);

        self.set_register(self.current_instruction.rt() as usize, value)
    }

    fn lwl(&mut self) {
        self.cpu_result = CycleResult::Error;
        println!("Unimplemented instruction: LWL at PC {:08X}", self.pc);
    }

    fn lwr(&mut self) {
        self.cpu_result = CycleResult::Error;
        println!("Unimplemented instruction: LWR at PC {:08X}", self.pc);
    }

    fn sb(&mut self) {
        let immediate = self.current_instruction.immediate() as i32;
        let address = self.registers[self.current_instruction.rs() as usize].wrapping_add(immediate as u32);
        let value = (self.registers[self.current_instruction.rt() as usize] & 0xFF) as u8;
        self.memory.write_byte(address, value);
    }

    fn sh(&mut self) {
        let immediate = self.current_instruction.immediate() as i32;
        let address = self.registers[self.current_instruction.rs() as usize].wrapping_add(immediate as u32);
        let value = (self.registers[self.current_instruction.rt() as usize] & 0xFFFF) as u16;
        self.memory.write_halfword(address, value);
    }

    fn sw(&mut self) {
        let immediate = self.current_instruction.immediate() as i32;
        let address = self.registers[self.current_instruction.rs() as usize].wrapping_add(immediate as u32);
        let value = self.registers[self.current_instruction.rt() as usize];
        self.memory.write_word(address, value);
    }

    fn swl(&mut self) {
        self.cpu_result = CycleResult::Error;
        println!("Unimplemented instruction: SWL at PC {:08X}", self.pc);
    }

    fn swr(&mut self) {
        self.cpu_result = CycleResult::Error;
        println!("Unimplemented instruction: SWR at PC {:08X}", self.pc);
    }



    // ALU instructions

    fn addi(&mut self) {
        let value = self.current_instruction.immediate() as i32;
        let result = self.registers[self.current_instruction.rs() as usize].overflowing_add(value as u32);

        if result.1 {
            self.cpu_result = CycleResult::Error;
            println!("ADDI overflowed and traps are not implemented!");
        }

        self.set_register(self.current_instruction.rt() as usize, result.0);
    }

    fn addiu(&mut self) {
        let value = self.current_instruction.immediate() as i32;
        let result = self.registers[self.current_instruction.rs() as usize].wrapping_add(value as u32);
        self.set_register(self.current_instruction.rt() as usize, result);
    }

    fn slti(&mut self) {
        if (self.registers[self.current_instruction.rs() as usize] as i32) < (self.current_instruction.immediate() as i32) {
            self.set_register(self.current_instruction.rt() as usize, 1)
        }
        else {
            self.set_register(self.current_instruction.rt() as usize, 0)
        }
    }

    fn sltiu(&mut self) {
        if (self.registers[self.current_instruction.rs() as usize]) < (self.current_instruction.immediate() as i32) as u32 {
            self.set_register(self.current_instruction.rt() as usize, 1)
        }
        else {
            self.set_register(self.current_instruction.rt() as usize, 0)
        }
    }

    fn andi(&mut self) {
        let result = self.registers[self.current_instruction.rs() as usize] & self.current_instruction.immediate() as u32;
        self.set_register(self.current_instruction.rt() as usize, result)
    }

    fn ori(&mut self) {
        let result = self.registers[self.current_instruction.rs() as usize] | self.current_instruction.immediate() as u32;
        self.set_register(self.current_instruction.rt() as usize, result)
    }

    fn xori(&mut self) {
        let result = self.registers[self.current_instruction.rs() as usize] ^ self.current_instruction.immediate() as u32;
        self.set_register(self.current_instruction.rt() as usize, result)
    }

    fn lui(&mut self) {
        let value = (self.current_instruction.immediate() as u32) << 16;
        self.set_register(self.current_instruction.rt() as usize, value)
    }


    fn add(&mut self) {
        let value = self.registers[self.current_instruction.rt() as usize];
        let result = self.registers[self.current_instruction.rs() as usize].overflowing_add(value);

        if result.1 {
            self.cpu_result = CycleResult::Error;
            println!("ADD overflowed and traps are not implemented!");
        }

        self.set_register(self.current_instruction.rd() as usize, result.0);
    }

    fn addu(&mut self) {
        let result = self.registers[self.current_instruction.rs() as usize].wrapping_add(self.registers[self.current_instruction.rt() as usize]);
        self.set_register(self.current_instruction.rd() as usize, result)
    }

    fn sub(&mut self) {
        let value = self.registers[self.current_instruction.rt() as usize];
        let result = self.registers[self.current_instruction.rs() as usize].overflowing_sub(value);

        if result.1 {
            self.cpu_result = CycleResult::Error;
            println!("SUB overflowed and traps are not implemented!");
        }

        self.set_register(self.current_instruction.rd() as usize, result.0);
    }

    fn subu(&mut self) {
        let result = self.registers[self.current_instruction.rs() as usize].wrapping_sub(self.registers[self.current_instruction.rt() as usize]);
        self.set_register(self.current_instruction.rd() as usize, result)
    }

    fn slt(&mut self) {
        if (self.registers[self.current_instruction.rs() as usize] as i32) < (self.registers[self.current_instruction.rt() as usize] as i32) {
            self.set_register(self.current_instruction.rd() as usize, 1)
        }
        else {
            self.set_register(self.current_instruction.rd() as usize, 0)
        }
    }

    fn sltu(&mut self) {
        if self.registers[self.current_instruction.rs() as usize] < self.registers[self.current_instruction.rt() as usize] {
            self.set_register(self.current_instruction.rd() as usize, 1)
        }
        else {
            self.set_register(self.current_instruction.rd() as usize, 0)
        }
    }

    fn and(&mut self) {
        let result = self.registers[self.current_instruction.rs() as usize] & self.registers[self.current_instruction.rt() as usize];
        self.set_register(self.current_instruction.rd() as usize, result)
    }

    fn or(&mut self) {
        let result = self.registers[self.current_instruction.rs() as usize] | self.registers[self.current_instruction.rt() as usize];
        self.set_register(self.current_instruction.rd() as usize, result)
    }

    fn xor(&mut self) {
        let result = self.registers[self.current_instruction.rs() as usize] ^ self.registers[self.current_instruction.rt() as usize];
        self.set_register(self.current_instruction.rd() as usize, result)
    }

    fn nor(&mut self) {
        self.cpu_result = CycleResult::Error;
        println!("Unimplemented instruction: NOR at PC {:08X}", self.pc);
    }


    fn sll(&mut self) {
        let result = self.registers[self.current_instruction.rt() as usize] << self.current_instruction.shift();
        self.set_register(self.current_instruction.rd() as usize, result)
    }

    fn srl(&mut self) {
        let result = self.registers[self.current_instruction.rt() as usize] >> self.current_instruction.shift();
        self.set_register(self.current_instruction.rd() as usize, result)
    }

    fn sra(&mut self) {
        self.cpu_result = CycleResult::Error;
        println!("Unimplemented instruction: SRA at PC {:08X}", self.pc);
    }

    fn sllv(&mut self) {
        let result = self.registers[self.current_instruction.rt() as usize] << (self.registers[self.current_instruction.rs() as usize] & 0x1F);
        self.set_register(self.current_instruction.rd() as usize, result)
    }

    fn srlv(&mut self) {
        let result = self.registers[self.current_instruction.rt() as usize] >> (self.registers[self.current_instruction.rs() as usize] & 0x1F);
        self.set_register(self.current_instruction.rd() as usize, result)
    }

    fn srav(&mut self) {
        self.cpu_result = CycleResult::Error;
        println!("Unimplemented instruction: SRAV at PC {:08X}", self.pc);
    }


    fn mult(&mut self) {
        self.cpu_result = CycleResult::Error;
        println!("Unimplemented instruction: MULT at PC {:08X}", self.pc);
    }

    fn multu(&mut self) {
        self.cpu_result = CycleResult::Error;
        println!("Unimplemented instruction: MULTU at PC {:08X}", self.pc);
    }

    fn div(&mut self) {
        self.cpu_result = CycleResult::Error;
        println!("Unimplemented instruction: DIV at PC {:08X}", self.pc);
    }

    fn divu(&mut self) {
        self.cpu_result = CycleResult::Error;
        println!("Unimplemented instruction: DIVU at PC {:08X}", self.pc);
    }

    fn mfhi(&mut self) {
        self.cpu_result = CycleResult::Error;
        println!("Unimplemented instruction: MFHI at PC {:08X}", self.pc);
    }

    fn mflo(&mut self) {
        self.cpu_result = CycleResult::Error;
        println!("Unimplemented instruction: MFLO at PC {:08X}", self.pc);
    }

    fn mthi(&mut self) {
        self.cpu_result = CycleResult::Error;
        println!("Unimplemented instruction: MTHI at PC {:08X}", self.pc);
    }

    fn mtlo(&mut self) {
        self.cpu_result = CycleResult::Error;
        println!("Unimplemented instruction: MTLO at PC {:08X}", self.pc);
    }



    // Jump and Branch instructions

    fn j(&mut self) {
        let target_addr = (self.pc & 0xF0000000) + (self.current_instruction.target() << 2);
        self.pc = target_addr;
        self.branch_delay = true;
    }

    fn jal(&mut self) {
        let target_addr = (self.pc & 0xF0000000) + (self.current_instruction.target() << 2);
        self.set_register(31, self.pc + 8);
        self.pc = target_addr;
        self.branch_delay = true;
    }

    fn jr(&mut self) {
        self.pc = self.registers[self.current_instruction.rs() as usize];
        self.branch_delay = true;
    }

    fn jalr(&mut self) {
        self.set_register(self.current_instruction.rd() as usize, self.pc + 4);
        self.pc = self.registers[self.current_instruction.rs() as usize];
        self.branch_delay = true;
    }


    fn beq(&mut self) {
        if self.registers[self.current_instruction.rs() as usize] == self.registers[self.current_instruction.rt() as usize] {
            self.take_branch();
        }
    }

    fn bne(&mut self) {
        if self.registers[self.current_instruction.rs() as usize] != self.registers[self.current_instruction.rt() as usize] {
            self.take_branch();
        }
    }

    fn blez(&mut self) {
        if self.registers[self.current_instruction.rs() as usize] as i32 <= 0 {
            self.take_branch();
        }
    }

    fn bgtz(&mut self) {
        if self.registers[self.current_instruction.rs() as usize] as i32 >= 0 {
            self.take_branch();
        }
    }

    fn bltz(&mut self) {
        if (self.registers[self.current_instruction.rs() as usize] as i32) < 0 {
            self.take_branch();
        }
    }

    fn bgez(&mut self) {
        if self.registers[self.current_instruction.rs() as usize] as i32 >= 0 {
            self.take_branch();
        }
    }

    fn bltzal(&mut self) {
        // Place the address after the delay slot on the link register.
        self.registers[31] = self.pc + 4;
        if (self.registers[self.current_instruction.rs() as usize] as i32) < 0 {
            self.take_branch();
        }
    }

    fn bgezal(&mut self) {
        // Place the address after the delay slot on the link register.
        self.registers[31] = self.pc + 4;
        if self.registers[self.current_instruction.rs() as usize] as i32 >= 0 {
            self.take_branch();
        }
    }

    

    // Special instructions

    fn syscall(&mut self) {
        self.cpu_result = CycleResult::Error;
        println!("Unimplemented instruction: SYSCALL at PC {:08X}", self.pc);
    }

    fn break_op(&mut self) {
        self.cpu_result = CycleResult::Error;
        println!("Unimplemented instruction: BREAK at PC {:08X}", self.pc);
    }



    // COP0 Instructions

    fn cop0(&mut self) {
        match self.current_instruction.rs() {
            0x00 => self.mfc0(),
            0x04 => self.mtc0(),
            _ => {
                self.cpu_result = CycleResult::Error;
                println!("Unimplemented COP0 instruction: {:X}({:b})", self.current_instruction.rs(), self.current_instruction.rs());
            },
        }
    }

    fn mfc0(&mut self) {
        self.set_register(self.current_instruction.rt() as usize, self.cop0_registers[self.current_instruction.rd() as usize]);
    }

    fn mtc0(&mut self) {
        self.cop0_registers[self.current_instruction.rd() as usize] = self.registers[self.current_instruction.rt() as usize];
    }
}