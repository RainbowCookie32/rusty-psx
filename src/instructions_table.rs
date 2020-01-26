use std::collections::HashMap;

pub fn make_primary_opcodes_hashmap() -> HashMap<u32, String> {
    let mut map = HashMap::new();

    map.insert(0x00, String::from("Special"));
    map.insert(0x01, String::from("BcondZ"));
    map.insert(0x02, String::from("J"));
    map.insert(0x03, String::from("JAL"));
    map.insert(0x04, String::from("BEQ"));
    map.insert(0x05, String::from("BNE"));
    map.insert(0x06, String::from("BLEZ"));
    map.insert(0x07, String::from("BGTZ"));

    map.insert(0x08, String::from("ADDI"));
    map.insert(0x09, String::from("ADDIU"));
    map.insert(0x0A, String::from("SLTI"));
    map.insert(0x0B, String::from("SLTIU"));
    map.insert(0x0C, String::from("ANDI"));
    map.insert(0x0D, String::from("ORI"));
    map.insert(0x0E, String::from("XORI"));
    map.insert(0x0F, String::from("LUI"));

    map.insert(0x10, String::from("COP0"));
    map.insert(0x11, String::from("COP1"));
    map.insert(0x12, String::from("COP2"));
    map.insert(0x13, String::from("COP3"));
    map.insert(0x14, String::from("Invalid"));
    map.insert(0x15, String::from("Invalid"));
    map.insert(0x16, String::from("Invalid"));
    map.insert(0x17, String::from("Invalid"));

    map.insert(0x18, String::from("Invalid"));
    map.insert(0x19, String::from("Invalid"));
    map.insert(0x1A, String::from("Invalid"));
    map.insert(0x1B, String::from("Invalid"));
    map.insert(0x1C, String::from("Invalid"));
    map.insert(0x1D, String::from("Invalid"));
    map.insert(0x1E, String::from("Invalid"));
    map.insert(0x1F, String::from("Invalid"));

    map.insert(0x20, String::from("LB"));
    map.insert(0x21, String::from("LH"));
    map.insert(0x22, String::from("LWL"));
    map.insert(0x23, String::from("LW"));
    map.insert(0x24, String::from("LBU"));
    map.insert(0x25, String::from("LHU"));
    map.insert(0x26, String::from("LWR"));
    map.insert(0x27, String::from("Invalid"));

    map.insert(0x28, String::from("SB"));
    map.insert(0x29, String::from("SH"));
    map.insert(0x2A, String::from("SWL"));
    map.insert(0x2B, String::from("SW"));
    map.insert(0x2C, String::from("Invalid"));
    map.insert(0x2D, String::from("Invalid"));
    map.insert(0x2E, String::from("SWR"));
    map.insert(0x2F, String::from("Invalid"));

    map.insert(0x30, String::from("LWC0"));
    map.insert(0x31, String::from("LWC1"));
    map.insert(0x32, String::from("LWC2"));
    map.insert(0x33, String::from("LWC3"));
    map.insert(0x34, String::from("Invalid"));
    map.insert(0x35, String::from("Invalid"));
    map.insert(0x36, String::from("Invalid"));
    map.insert(0x37, String::from("Invalid"));

    map.insert(0x38, String::from("SWC0"));
    map.insert(0x39, String::from("SWC1"));
    map.insert(0x3A, String::from("SWC2"));
    map.insert(0x3B, String::from("SWC3"));
    map.insert(0x3C, String::from("Invalid"));
    map.insert(0x3D, String::from("Invalid"));
    map.insert(0x3E, String::from("Invalid"));
    map.insert(0x3F, String::from("Invalid"));

    map
}

pub fn make_secondary_opcodes_hashmap() -> HashMap<u32, String> {
    let mut map = HashMap::new();

    map.insert(0x00, String::from("SLL"));
    map.insert(0x01, String::from("Invalid"));
    map.insert(0x02, String::from("SRL"));
    map.insert(0x03, String::from("SRA"));
    map.insert(0x04, String::from("SLLV"));
    map.insert(0x05, String::from("Invalid"));
    map.insert(0x06, String::from("SRLV"));
    map.insert(0x07, String::from("SRAV"));

    map.insert(0x08, String::from("JR"));
    map.insert(0x09, String::from("JALR"));
    map.insert(0x0A, String::from("Invalid"));
    map.insert(0x0B, String::from("Invalid"));
    map.insert(0x0C, String::from("SYSCALL"));
    map.insert(0x0D, String::from("BREAK"));
    map.insert(0x0E, String::from("Invalid"));
    map.insert(0x0F, String::from("Invalid"));

    map.insert(0x10, String::from("MFHI"));
    map.insert(0x11, String::from("MTHI"));
    map.insert(0x12, String::from("MFLO"));
    map.insert(0x13, String::from("MTLO"));
    map.insert(0x14, String::from("Invalid"));
    map.insert(0x15, String::from("Invalid"));
    map.insert(0x16, String::from("Invalid"));
    map.insert(0x17, String::from("Invalid"));

    map.insert(0x18, String::from("MULT"));
    map.insert(0x19, String::from("MULTU"));
    map.insert(0x1A, String::from("DIV"));
    map.insert(0x1B, String::from("DIVU"));
    map.insert(0x1C, String::from("Invalid"));
    map.insert(0x1D, String::from("Invalid"));
    map.insert(0x1E, String::from("Invalid"));
    map.insert(0x1F, String::from("Invalid"));

    map.insert(0x20, String::from("ADD"));
    map.insert(0x21, String::from("ADDU"));
    map.insert(0x22, String::from("SUB"));
    map.insert(0x23, String::from("SUBU"));
    map.insert(0x24, String::from("AND"));
    map.insert(0x25, String::from("OR"));
    map.insert(0x26, String::from("XOR"));
    map.insert(0x27, String::from("NOR"));

    map.insert(0x28, String::from("Invalid"));
    map.insert(0x29, String::from("Invalid"));
    map.insert(0x2A, String::from("SLT"));
    map.insert(0x2B, String::from("SLTU"));
    map.insert(0x2C, String::from("Invalid"));
    map.insert(0x2D, String::from("Invalid"));
    map.insert(0x2E, String::from("Invalid"));
    map.insert(0x2F, String::from("Invalid"));

    map.insert(0x30, String::from("Invalid"));
    map.insert(0x31, String::from("Invalid"));
    map.insert(0x32, String::from("Invalid"));
    map.insert(0x33, String::from("Invalid"));
    map.insert(0x34, String::from("Invalid"));
    map.insert(0x35, String::from("Invalid"));
    map.insert(0x36, String::from("Invalid"));
    map.insert(0x37, String::from("Invalid"));

    map.insert(0x38, String::from("Invalid"));
    map.insert(0x39, String::from("Invalid"));
    map.insert(0x3A, String::from("Invalid"));
    map.insert(0x3B, String::from("Invalid"));
    map.insert(0x3C, String::from("Invalid"));
    map.insert(0x3D, String::from("Invalid"));
    map.insert(0x3E, String::from("Invalid"));
    map.insert(0x3F, String::from("Invalid"));

    map
}