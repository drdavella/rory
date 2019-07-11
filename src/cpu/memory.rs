use cpu::types;
use cpu::types::Register;
use cpu::types::GameState;

use cpu::debug::Debug;


pub enum Operation {
    Increment, Decrement
}

impl GameState {
pub fn load_reg(&mut self, opcode: u8) -> Debug {
    let high = opcode >> 4;
    let low = opcode & 0xf;
    let dest_idx = (low / 8) + (high - 0x4) * 2;

    let source = &types::REGISTER_LIST[(low % 8) as usize];
    let dest = &types::REGISTER_LIST[dest_idx as usize];
    match (source, dest) {
        (&Register::HL, _) => panic!("Load to/from HL not implemented"),
        (_, &Register::HL) => {
            let addr = self.get_hl();
            let reg_val = self.get_register(source);
            self.write_mem(addr, reg_val);
            self.ticks += 8;
            self.pc += 1;

            debug_format!("LD {:?} => 0x{:04x}", source, addr)
        },
        (_, _) => {
            let reg_val = self.get_register(source);
            self.set_register(dest, reg_val);
            self.ticks += 4;
            self.pc += 1;

            debug_format!("LD {:?} => {:?}", source, dest)
        }
    }

}

pub fn load_word_imm(&mut self, opcode: u8, code_bytes: &[u8]) -> Debug {
    let high = opcode >> 4;
    let low = opcode & 0xf;
    let dest_idx = (low >> 3) + (high * 2);

    let reg = &types::REGISTER_LIST[dest_idx as usize];
    match reg {
        &Register::HL => panic!("Load to HL not implemented"),
        _ => {
            self.set_register(reg, code_bytes[0]);
            self.ticks += 8;
        }
    }

    self.pc += 2;

    debug_format!("LD 0x{:02x} => {:?}", code_bytes[0], reg)
}

fn load_compound_register(&mut self, opcode: u8, code_bytes: &[u8]) -> Debug {
    let (high, low) = match opcode {
        0x01 => (Register::B, Register::C),
        0x11 => (Register::D, Register::E),
        0x21 => (Register::H, Register::L),
        _ => panic!("Unrecognized opcode: 0x{:02x}", opcode)
    };

    self.set_register(&high, code_bytes[1]);
    self.set_register(&low, code_bytes[0]);

    debug_format!("LD 0x{:02x}{:02x} => {:?}{:?}",
        code_bytes[1], code_bytes[0], high, low)
}

pub fn load_dword_imm(&mut self, opcode: u8, code_bytes: &[u8]) -> Debug {
    let msg = match opcode {
        0x31 => {
            /* Load stack pointer */
            self.sp = ((code_bytes[1] as u16) << 8) | (code_bytes[0] as u16);
            debug_format!("LD 0x{:04x} => SP", self.sp)
        }
        _ => {
            self.load_compound_register(opcode, code_bytes)
        }
    };

    self.ticks += 12;
    self.pc += 3;

    msg
}

pub fn store_and_update(&mut self, operation: Operation) -> Debug {
    let addr = self.get_hl();
    let value = self.get_register(&Register::A);
    self.write_mem(addr, value);

    let new_addr = match operation {
        Operation::Decrement => addr.wrapping_sub(1),
        Operation::Increment => addr.wrapping_add(1),
    };

    self.set_hl(new_addr);

    self.ticks += 8;
    self.pc += 1;

    debug_format!("LD (HL +/-): A => mem[0x{:04x}]", addr)
}

pub fn load_and_update(&mut self, operation: Operation) -> Debug {

    let addr = self.get_hl();
    let value = self.read_mem(addr);
    self.set_register(&Register::A, value);

    let new_addr = match operation {
        Operation::Decrement => addr.wrapping_sub(1),
        Operation::Increment => addr.wrapping_add(1),
    };

    self.set_hl(new_addr);

    self.ticks += 8;
    self.pc += 1;

    debug_format!("LD (HL +/-): mem[0x{:04x}] => A", addr)
}

pub fn store_imm_addr(&mut self, code_bytes: &[u8]) -> Debug {
    let addr = ((code_bytes[1] as u16) << 8) | code_bytes[0] as u16;
    let value = self.get_register(&Register::A);
    self.write_mem(addr, value);

    self.ticks += 16;
    self.pc += 3;

    debug_format!("LD A => mem[0x{:04x}]", addr)
}

pub fn load_a_mem(&mut self, code_bytes: &[u8]) -> Debug {
    let addr = (0xff00 as u16).wrapping_add(code_bytes[0] as u16);
    let value = self.read_mem(addr);
    self.set_register(&Register::A, value);

    self.ticks += 12;
    self.pc += 2;

    debug_format!("LD mem[0x{:02x}] => A", addr)
}

pub fn store_a_mem(&mut self, code_bytes: &[u8]) -> Debug {
    let addr = (0xff00 as u16).wrapping_add(code_bytes[0] as u16);
    let value = self.get_register(&Register::A);
    self.write_mem(addr, value);

    self.ticks += 12;
    self.pc += 2;

    debug_format!("LD A => mem[0x{:02x}]", addr)
}

pub fn store_a_indirect_c(&mut self) -> Debug {
    let regc = self.get_register(&Register::C);
    let addr = (0xff00 as u16).wrapping_add(regc as u16);
    let value = self.get_register(&Register::A);
    self.write_mem(addr, value);

    self.ticks += 8;
    self.pc += 2;

    debug_format!("LD A => mem[C]")
}

pub fn load_a_indirect_c(&mut self) -> Debug {
    let regc = self.get_register(&Register::C);
    let addr = (0xff00 as u16).wrapping_add(regc as u16);
    let value = self.read_mem(addr);
    self.set_register(&Register::A, value);

    self.ticks += 8;
    self.pc += 2;

    debug_format!("LD mem[C] => A")
}
} /* impl GameState */
