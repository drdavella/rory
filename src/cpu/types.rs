use std::ops::{Index,IndexMut};


pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8
}

pub struct Flags {
    pub zero: bool,
    pub carry: bool
}

pub struct GameState {
    pub pc: u16,
    pub sp: u16,
    pub ticks: i32,
    pub flags: Flags,
    pub regs: Registers,
    pub memory: [u8; 0x10000],
}

impl Index<u16> for GameState {
    type Output = u8;

    fn index<'a>(&'a self, index: u16) -> &Self::Output {
        &self.memory[index as usize]
    }
}

impl IndexMut<u16> for GameState {
    fn index_mut<'a>(&'a mut self, index: u16) -> &mut Self::Output {
        match index {
            0x0100 ... 0x3fff => panic!("Attempted write to cartridge ROM"),
            0x4000 ... 0x7fff => panic!("Attempted write to switchable ROM bank"),
            0xff00 ... 0xff7f => {
                println!("Write to hardware I/O register: 0x{:04x}", index);
                &mut self.memory[index as usize]
            }
            _ => &mut self.memory[index as usize]
        }
    }
}

pub enum Register {
    A, B, C, D, E, H, L, HL
}

pub const REGISTER_LIST: [Register; 8] = [
    Register::B, Register::C, Register::D, Register::E,
    Register::H, Register::L, Register::HL, Register::A
];

#[allow(dead_code)]
pub fn reg_to_str(reg: &Register) -> &str {
    match reg {
        &Register::A => "A",
        &Register::B => "B",
        &Register::C => "C",
        &Register::D => "D",
        &Register::E => "E",
        &Register::H => "H",
        &Register::L => "L",
        &Register::HL => "HL"
    }
}

pub fn set_register(state: &mut GameState, reg: &Register, val: u8) {
    match reg {
        &Register::A => state.regs.a = val,
        &Register::B => state.regs.b = val,
        &Register::C => state.regs.c = val,
        &Register::D => state.regs.d = val,
        &Register::E => state.regs.e = val,
        &Register::H => state.regs.h = val,
        &Register::L => state.regs.l = val,
        &Register::HL => panic!("Can't set HL as single register")
    };
}

pub fn get_register(state: &GameState, reg: &Register) -> u8 {
    match reg {
        &Register::A => state.regs.a,
        &Register::B => state.regs.b,
        &Register::C => state.regs.c,
        &Register::D => state.regs.d,
        &Register::E => state.regs.e,
        &Register::H => state.regs.h,
        &Register::L => state.regs.l,
        &Register::HL => panic!("Can't get HL as single register")
    }
}

pub fn set_hl(state: &mut GameState, val: u16) {
    state.regs.h = (val >> 8) as u8;
    state.regs.l = (val & 0xff) as u8;
}

pub fn get_hl(state: &GameState) -> u16 {
    ((state.regs.h as u16) << 8) | (state.regs.l as u16)
}
