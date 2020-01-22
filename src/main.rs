mod cpu;
mod memory;

fn main() {
    
    let mut cpu = cpu::Cpu::new();
    cpu.start_cpu();
}
