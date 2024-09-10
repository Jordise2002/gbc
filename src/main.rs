use std::env;

use cartridge::Cartidge;
use cpu::Cpu;

mod bus;
mod cartridge;
mod code;
mod cpu;
mod memory_bank;

fn main() {
    let path = env::args().nth(1).expect("Usage gbc {path to rom}");
    let cart = Cartidge::new(path);
    let mut cpu = Cpu::new(cart);
    cpu.run();
}
