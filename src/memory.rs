use byteorder::{ByteOrder, LittleEndian};

pub const RAM: MemoryRegion = MemoryRegion(0x00000000, 2048 * 1024);
pub const EXPANSION_1: MemoryRegion = MemoryRegion(0x1F000000, 8192 * 1024);
pub const SCRATCH: MemoryRegion = MemoryRegion(0x1F800000, 1024);
pub const IO_PORTS: MemoryRegion = MemoryRegion(0x1F801000, 8192);
pub const EXPANSION_2: MemoryRegion = MemoryRegion(0x1F802000, 8192);
pub const EXPANSION_3: MemoryRegion = MemoryRegion(0x1FA00000, 2048 * 1024);
pub const BIOS: MemoryRegion = MemoryRegion(0x1FC00000, 512 * 1024);
pub const CACHE_CONTROL: MemoryRegion = MemoryRegion(0x1FFE0000, 512);

pub struct MemoryRegion (pub u32, pub u32);

impl MemoryRegion {

    pub fn contains(self, address: u32) -> Option<u32> {
        let MemoryRegion(start, length) = self;
        
        if address >= start && address < (start + length) {
            Some(address - start)
        }
        else {
            None
        }
    }
}


pub struct CpuMemory {
    pub ram: Vec<u8>,
    pub expansion_1: Vec<u8>,
    pub scratchpad: Vec<u8>,
    pub io_ports: Vec<u8>,
    pub expansion_2: Vec<u8>,
    pub expansion_3: Vec<u8>,
    pub bios: Vec<u8>,
    pub cache_control: Vec<u8>,
}

impl CpuMemory {

    pub fn new(bios_data: Vec<u8>) -> CpuMemory {
        CpuMemory{
            ram: vec![0; 2048*1024],
            expansion_1: vec![0; 8192*1024],
            scratchpad: vec![0; 1024],
            io_ports: vec![0; 8192],
            expansion_2: vec![0; 8192],
            expansion_3: vec![0; 2048*1024],
            bios: bios_data,
            cache_control: vec![0; 512],
        }
    }

    pub fn read_byte(&self, address: u32) -> u8 {
        self.read(address) as u8
    }

    pub fn read_halfword(&self, address: u32) -> u16 {
        if address % 2 != 0 {
            panic!("Unaligned memory read at address {:08X}", address);
        }
        self.read(address) as u16
    }

    pub fn read_word(&self, address: u32) -> u32 {
        if address % 4 != 0 {
            panic!("Unaligned memory read at address {:08X}", address);
        }
        self.read(address)
    }

    fn read(&self, address: u32) -> u32 {

        let address = address & 0x1FFFFFFF;
        let mut bytes = vec![0; 4];

        if let Some(offset) = RAM.contains(address) {
            bytes[0] = self.ram[offset as usize];
            bytes[1] = self.ram[offset as usize + 1];
            bytes[2] = self.ram[offset as usize + 2];
            bytes[3] = self.ram[offset as usize + 3];
            LittleEndian::read_u32(&bytes)
        }
        else if let Some(offset) = EXPANSION_1.contains(address) {
            bytes[0] = self.expansion_1[offset as usize];
            bytes[1] = self.expansion_1[offset as usize + 1];
            bytes[2] = self.expansion_1[offset as usize + 2];
            bytes[3] = self.expansion_1[offset as usize + 3];
            LittleEndian::read_u32(&bytes)
        }
        else if let Some(offset) = SCRATCH.contains(address) {
            bytes[0] = self.scratchpad[offset as usize];
            bytes[1] = self.scratchpad[offset as usize + 1];
            bytes[2] = self.scratchpad[offset as usize + 2];
            bytes[3] = self.scratchpad[offset as usize + 3];
            LittleEndian::read_u32(&bytes)
        }
        else if let Some(offset) = IO_PORTS.contains(address) {
            bytes[0] = self.io_ports[offset as usize];
            bytes[1] = self.io_ports[offset as usize + 1];
            bytes[2] = self.io_ports[offset as usize + 2];
            bytes[3] = self.io_ports[offset as usize + 3];
            LittleEndian::read_u32(&bytes)
        }
        else if let Some(offset) = EXPANSION_2.contains(address) {
            bytes[0] = self.expansion_2[offset as usize];
            bytes[1] = self.expansion_2[offset as usize + 1];
            bytes[2] = self.expansion_2[offset as usize + 2];
            bytes[3] = self.expansion_2[offset as usize + 3];
            LittleEndian::read_u32(&bytes)
        }
        else if let Some(offset) = EXPANSION_3.contains(address) {
            bytes[0] = self.expansion_3[offset as usize];
            bytes[1] = self.expansion_3[offset as usize + 1];
            bytes[2] = self.expansion_3[offset as usize + 2];
            bytes[3] = self.expansion_3[offset as usize + 3];
            LittleEndian::read_u32(&bytes)
        }
        else if let Some(offset) = BIOS.contains(address) {
            bytes[0] = self.bios[offset as usize];
            bytes[1] = self.bios[offset as usize + 1];
            bytes[2] = self.bios[offset as usize + 2];
            bytes[3] = self.bios[offset as usize + 3];
            LittleEndian::read_u32(&bytes)
        }
        else if let Some(offset) = CACHE_CONTROL.contains(address) {
            bytes[0] = self.cache_control[offset as usize];
            bytes[1] = self.cache_control[offset as usize + 1];
            bytes[2] = self.cache_control[offset as usize + 2];
            bytes[3] = self.cache_control[offset as usize + 3];
            LittleEndian::read_u32(&bytes)
        }
        else {
            println!("Read to unknown location {:08X}", address);
            0
        }
    }

    pub fn write_byte(&mut self, address: u32, value: u8) {
        self.write(address, value);
    }

    pub fn write_halfword(&mut self, address: u32, value: u16) {
        if address % 2 != 0 {
            panic!("Unaligned memory write at address {:08X}", address);
        }
        self.write(address, value as u8);
        self.write(address + 1, (value >> 8) as u8);
    }

    pub fn write_word(&mut self, address: u32, value: u32) {
        if address % 4 != 0 {
            panic!("Unaligned memory write at address {:08X}", address);
        }
        self.write(address, value as u8);
        self.write(address + 1, (value >> 8) as u8);
        self.write(address + 2, (value >> 16) as u8);
        self.write(address + 3, (value >> 24) as u8);
    }

    fn write(&mut self, address: u32, value: u8) {

        let address = address & 0x1FFFFFFF;
        
        if let Some(offset) = RAM.contains(address) {
            self.ram[offset as usize] = value;
        }
        else if let Some(offset) = EXPANSION_1.contains(address) {
            self.expansion_1[offset as usize] = value;
        }
        else if let Some(offset) = SCRATCH.contains(address) {
            self.scratchpad[offset as usize] = value;
        }
        else if let Some(offset) = IO_PORTS.contains(address) {
            self.io_ports[offset as usize] = value;
        }
        else if let Some(offset) = EXPANSION_2.contains(address) {
            self.expansion_2[offset as usize] = value;
        }
        else if let Some(offset) = EXPANSION_3.contains(address) {
            self.expansion_3[offset as usize] = value;
        }
        else if let Some(_offset) = BIOS.contains(address) {
            println!("Tried to write {:X} to address {:X} in BIOS", value, address);
        }
        else if let Some(offset) = CACHE_CONTROL.contains(address) {
            self.cache_control[offset as usize] = value;
        }
        else {
            println!("Tried to write {:X} to unhandled address {:X}", value, address);
        }
    }
}