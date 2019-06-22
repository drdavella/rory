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
    hardware_regs: [u8; 0x80],
}

impl GameState {

/* Constructor for new state starting with given pc and sp */
pub fn initialize(pc: u16, sp: u16) -> GameState {
    GameState {
        pc: pc,
        sp: sp,
        ticks: 0,
        flags: Flags {
            zero: false,
            carry: false,
        },
        regs: Registers{
            a: 0, b:0, c:0, d:0, e:0, h:0, l:0
        },
        memory: [0; 0x10000],
        hardware_regs: [0; 0x80],
    }
}

pub fn read_mem(&self, index: u16) -> u8 {
    match index {
        0xff00 ..= 0xff7f => {
            debug_println!(
                "Read from hardware I/O register: mem[0x{:04x}]", index);
            self.hardware_regs[(index - 0xff00) as usize]
        }
        _ => self.memory[index as usize]
    }
}

pub fn write_mem(&mut self, index: u16, value: u8) {
    match index {
        0x0100 ..= 0x3fff => panic!("Attempted write to cartridge ROM"),
        0x4000 ..= 0x7fff => panic!("Attempted write to switchable ROM bank"),
        0xff00 ..= 0xff7f => {
            debug_println!(
                "Write to hardware I/O register: mem[0x{:04x}] = 0x{:02}",
                index, value);
            self.hardware_regs[(index - 0xff00) as usize] = value;
        }
        _ => self.memory[index as usize] = value
    }
}

pub fn push(&mut self, value: u16) {
    let mut sp = self.sp;
    sp = sp.wrapping_sub(1);
    self.write_mem(sp, (value & 0xff) as u8);
    sp = sp.wrapping_sub(1);
    self.write_mem(sp, (value >> 8) as u8);
    self.sp = sp;
}

pub fn pop(&mut self) -> u16 {
    let mut new_val = (self.read_mem(self.sp) as u16) << 8;
    self.sp = self.sp.wrapping_add(1);
    new_val |= self.read_mem(self.sp) as u16;
    self.sp = self.sp.wrapping_add(1);

    new_val
}

pub fn set_register(&mut self, reg: &Register, val: u8) {
    match reg {
        &Register::A => self.regs.a = val,
        &Register::B => self.regs.b = val,
        &Register::C => self.regs.c = val,
        &Register::D => self.regs.d = val,
        &Register::E => self.regs.e = val,
        &Register::H => self.regs.h = val,
        &Register::L => self.regs.l = val,
        &Register::HL => panic!("Can't set HL as single register")
    };
}

pub fn get_register(&self, reg: &Register) -> u8 {
    match reg {
        &Register::A => self.regs.a,
        &Register::B => self.regs.b,
        &Register::C => self.regs.c,
        &Register::D => self.regs.d,
        &Register::E => self.regs.e,
        &Register::H => self.regs.h,
        &Register::L => self.regs.l,
        &Register::HL => panic!("Can't get HL as single register")
    }
}

pub fn set_hl(&mut self, val: u16) {
    self.regs.h = (val >> 8) as u8;
    self.regs.l = (val & 0xff) as u8;
}

pub fn get_hl(&self) -> u16 {
    ((self.regs.h as u16) << 8) | (self.regs.l as u16)
}
} /* impl GameState */

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
