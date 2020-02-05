use super::cpu::Instruction;


pub fn get_instruction_info(instruction: &Instruction) -> String {

    let mut result = String::from("");
    if instruction.op() != 0 {
        match instruction.op() {
            0x01 => {
                match instruction.rt() {
                    0x00 => result = String::from(format!("bltz, r{:02X}, {:08X}", instruction.rs(), instruction.target())),
                    0x01 => result = String::from(format!("bgtz, r{:02X}, {:08X}", instruction.rs(), instruction.target())),
                    0x10 => result = String::from(format!("bltzal, r{:02X}, {:08X}", instruction.rs(), instruction.target())),
                    0x11 => result = String::from(format!("bgezal, r{:02X}, {:08X}", instruction.rs(), instruction.target())),
                    _ => unreachable!(),
                }
            },
            0x02 => {
                result = String::from(format!("j {:08X}", instruction.target()));
            },
            0x03 => {
                result = String::from(format!("jal {:08X}", instruction.target()));
            },
            0x04 => {
                result = String::from(format!("beq, r{:02X}, r{:02X}, {:08X}", instruction.rs(), instruction.rt(), instruction.target()));
            },
            0x05 => {
                result = String::from(format!("bne, r{:02X}, r{:02X}, {:08X}", instruction.rs(), instruction.rt(), instruction.target()));
            },
            0x06 => {
                result = String::from(format!("blez, r{:02X}, {:08X}", instruction.rs(), instruction.target()));
            },
            0x07 => {
                result = String::from(format!("bgtz, r{:02X}, {:08X}", instruction.rs(), instruction.target()));
            },
            0x08 => {
                result = String::from(format!("addi, r{:02X}, r{:02X}, {:08X}", instruction.rt(), instruction.rs(), instruction.immediate()));
            },
            0x09 => {
                result = String::from(format!("addiu, r{:02X}, r{:02X}, {:08X}", instruction.rt(), instruction.rs(), instruction.immediate()));
            },
            0x0A => {
                result = String::from(format!("slti, r{:02X}, r{:02X}, {:08X}", instruction.rt(), instruction.rs(), instruction.immediate()));
            },
            0x0B => {
                result = String::from(format!("sltiu, r{:02X}, r{:02X}, {:08X}", instruction.rt(), instruction.rs(), instruction.immediate()));
            },
            0x0C => {
                result = String::from(format!("andi, r{:02X}, r{:02X}, {:08X}", instruction.rt(), instruction.rs(), instruction.immediate()));
            },
            0x0D => {
                result = String::from(format!("ori, r{:02X}, r{:02X}, {:08X}", instruction.rt(), instruction.rs(), instruction.immediate()));
            },
            0x0E => {
                result = String::from(format!("xori, r{:02X}, r{:02X}, {:08X}", instruction.rt(), instruction.rs(), instruction.immediate()));
            },
            0x0F => {
                result = String::from(format!("lui, r{:02X}, {:08X}", instruction.rt(), instruction.immediate()));
            },
            0x10 => {
                // COP0
            },
            0x11 => {

            },
            0x12 => {

            },
            0x13 => {

            },
            0x20 => {
                result = String::from(format!("lb, r{:02X}, {:08X}(r{:02X})", instruction.rt(), instruction.immediate(), instruction.rs()));
            },
            0x21 => {
                result = String::from(format!("lh, r{:02X}, {:08X}(r{:02X})", instruction.rt(), instruction.immediate(), instruction.rs()));
            },
            0x22 => {
                result = String::from(format!("lwl, r{:02X}, {:08X}(r{:02X})", instruction.rt(), instruction.immediate(), instruction.rs()));
            },
            0x23 => {
                result = String::from(format!("lw, r{:02X}, {:08X}(r{:02X})", instruction.rt(), instruction.immediate(), instruction.rs()));
            },
            0x24 => {
                result = String::from(format!("lbu, r{:02X}, {:08X}(r{:02X})", instruction.rt(), instruction.immediate(), instruction.rs()));
            },
            0x25 => {
                result = String::from(format!("lhu, r{:02X}, {:08X}(r{:02X})", instruction.rt(), instruction.immediate(), instruction.rs()));
            },
            0x26 => {
                result = String::from(format!("lwr, r{:02X}, {:08X}(r{:02X})", instruction.rt(), instruction.immediate(), instruction.rs()));
            },
            0x28 => {
                result = String::from(format!("sb, r{:02X}, {:08X}(r{:02X})", instruction.rt(), instruction.immediate(), instruction.rs()));
            },
            0x29 => {
                result = String::from(format!("sh, r{:02X}, {:08X}(r{:02X})", instruction.rt(), instruction.immediate(), instruction.rs()));
            },
            0x2A => {
                result = String::from(format!("swl, r{:02X}, {:08X}(r{:02X})", instruction.rt(), instruction.immediate(), instruction.rs()));
            },
            0x2B => {
                result = String::from(format!("sw, r{:02X}, {:08X}(r{:02X})", instruction.rt(), instruction.immediate(), instruction.rs()));
            },
            0x2E => {
                result = String::from(format!("swr, r{:02X}, {:08X}(r{:02X})", instruction.rt(), instruction.immediate(), instruction.rs()));
            },
            0x30 => {

            },
            0x31 => {

            },
            0x32 => {

            },
            0x33 => {

            },
            0x38 => {

            },
            0x39 => {

            },
            0x3A => {

            },
            0x3B => {

            },
            _ => {
                result = String::from("Illegal instruction");
            }
        }
    }
    else {
        match instruction.op() {
            0x00 => {
                result = String::from(format!("sll r{:02X}, r{:02X}, {:08X}", instruction.rd(), instruction.rt(), instruction.immediate()));
            },
            0x02 => {
                result = String::from(format!("srl r{:02X}, r{:02X}, {:08X}", instruction.rd(), instruction.rt(), instruction.immediate()));
            },
            0x03 => {
                result = String::from(format!("sra r{:02X}, r{:02X}, {:08X}", instruction.rd(), instruction.rt(), instruction.immediate()));
            },
            0x04 => {
                result = String::from(format!("sllv r{:02X}, r{:02X}, r{:02X}", instruction.rd(), instruction.rt(), instruction.rs()));
            },
            0x06 => {
                result = String::from(format!("srlv r{:02X}, r{:02X}, r{:02X}", instruction.rd(), instruction.rt(), instruction.rs()));
            },
            0x07 => {
                result = String::from(format!("srav r{:02X}, r{:02X}, r{:02X}", instruction.rd(), instruction.rt(), instruction.rs()));
            },
            0x08 => {
                result = String::from(format!("jr r{:02X}", instruction.rs()));
            },
            0x09 => {
                result = String::from(format!("jalr r{:02X}, r{:02X}", instruction.rs(), instruction.rd()));
            },
            0x0C => {
                result = String::from("syscall");
            },
            0x0D => {
                result = String::from("break");
            },
            0x10 => {
                result = String::from(format!("mfhi r{:02X}", instruction.rd()));
            },
            0x11 => {
                result = String::from(format!("mthi r{:02X}", instruction.rs()));
            },
            0x12 => {
                result = String::from(format!("mflo r{:02X}", instruction.rd()));
            },
            0x13 => {
                result = String::from(format!("mtlo r{:02X}", instruction.rs()));
            },
            0x18 => {
                result = String::from(format!("mult r{:02X}, r{:02X}", instruction.rs(), instruction.rt()));
            },
            0x19 => {
                result = String::from(format!("multu r{:02X}, r{:02X}", instruction.rs(), instruction.rt()));
            },
            0x1A => {
                result = String::from(format!("div r{:02X}, r{:02X}", instruction.rs(), instruction.rt()));
            },
            0x1B => {
                result = String::from(format!("divu r{:02X}, r{:02X}", instruction.rs(), instruction.rt()));
            },
            0x20 => {
                result = String::from(format!("add r{:02X}, r{:02X}, r{:02X}", instruction.rd(), instruction.rs(), instruction.rt()));
            },
            0x21 => {
                result = String::from(format!("addu r{:02X}, r{:02X}, r{:02X}", instruction.rd(), instruction.rs(), instruction.rt()));
            },
            0x22 => {
                result = String::from(format!("sub r{:02X}, r{:02X}, r{:02X}", instruction.rd(), instruction.rs(), instruction.rt()));
            },
            0x23 => {
                result = String::from(format!("subu r{:02X}, r{:02X}, r{:02X}", instruction.rd(), instruction.rs(), instruction.rt()));
            },
            0x24 => {
                result = String::from(format!("and r{:02X}, r{:02X}, r{:02X}", instruction.rd(), instruction.rs(), instruction.rt()));
            },
            0x25 => {
                result = String::from(format!("or r{:02X}, r{:02X}, r{:02X}", instruction.rd(), instruction.rs(), instruction.rt()));
            },
            0x26 => {
                result = String::from(format!("xor r{:02X}, r{:02X}, r{:02X}", instruction.rd(), instruction.rs(), instruction.rt()));
            },
            0x27 => {
                result = String::from(format!("nor r{:02X}, r{:02X}, r{:02X}", instruction.rd(), instruction.rs(), instruction.rt()));
            },
            0x2A => {
                result = String::from(format!("slt r{:02X}, r{:02X}, r{:02X}", instruction.rd(), instruction.rs(), instruction.rt()));
            },
            0x2B => {
                result = String::from(format!("sltu r{:02X}, r{:02X}, r{:02X}", instruction.rd(), instruction.rs(), instruction.rt()));
            },
            _ => {
                result = String::from("Illegal instruction");
            }
        }
    }

    result
}