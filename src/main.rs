use std::env;
use std::fs;

pub mod cpu;
pub use cpu::CPU;

pub mod cpm;
pub use cpm::CPM;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::read(args[1].clone()).unwrap();
    let mut m = [0xfd;0x10000];
    m[0x100..file.len()+0x100].copy_from_slice(&file);
    m[5] = 8;
    let mut c = CPU::new();
    c.index();
    loop{
        c.next(&mut m);
    }
}
