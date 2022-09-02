
//http://www.6502.org/tutorials/6502opcodes.html#VFLAG
//https://skilldrick.github.io/easy6502/
//https://bugzmanov.github.io/nes_ebook/chapter_1.html

pub struct CPU {
    pub register_a: u8,
    pub status: u8,
    pub program_counter: u16, 
}

impl CPU {
    pub fn new() -> Self {
        CPU{
            register_a: 0,
            status: 0,
            program_counter: 0,
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        todo!("")
    }
}
fn main() {
    println!("Hello, world!");
}
