use cpu::types::GameState;
use cpu::debug::Debug;

pub enum Condition {
    Zero,
    NotZero,
    NoCarry,
}

fn ubyte_to_sbyte(value: u8) -> i16 {
    -((value & 0x80) as i16) + ((value & 0x7f) as i16)
}

impl GameState {
pub fn jump_uncond_imm(&mut self, code_bytes: &[u8]) -> Debug {
    let new_addr = (code_bytes[1] as u16) << 8 | (code_bytes[0] as u16);
    self.pc = new_addr;
    self.ticks += 16;

    debug_format!("0x{:04x}", new_addr)
}

pub fn call_uncond_imm(&mut self, code_bytes: &[u8]) -> Debug {
    let new_addr = (code_bytes[1] as u16) << 8 | (code_bytes[0] as u16);
    let call_pc = self.pc + 3;

    self.push(call_pc);

    self.pc = new_addr;
    self.ticks += 24;

    debug_format!("CALL 0x{:04x}", new_addr)
}

pub fn jump_cond_imm(&mut self, code_bytes: &[u8],
                     condition: Condition) -> Debug {

    let (jump, _name) = match condition {
        Condition::Zero => (self.flags.zero, "Z"),
        Condition::NotZero => (!self.flags.zero, "NZ"),
        Condition::NoCarry => (!self.flags.carry, "NC"),
    };

    if jump {
        let offset = ubyte_to_sbyte(code_bytes[0]);
        /* Account for the size of this instruction in the offset */
        self.pc = (self.pc as i32 + offset as i32 + 2) as u16;
        self.ticks += 12;
        debug_format!("JP {} to 0x{:04x}", _name, self.pc)
    }
    else {
        self.pc += 2;
        self.ticks += 8;
        debug_format!("JP {}: not taken", _name)
    }
}

pub fn ret_cond(&mut self, condition: Condition) -> Debug {

    let (do_return, _name) = match condition {
        Condition::Zero => (self.flags.zero, "Z"),
        Condition::NotZero => (!self.flags.zero, "NZ"),
        Condition::NoCarry => (!self.flags.carry, "NC"),
    };

    if do_return {
        self.pc = self.pop();
        self.ticks += 20;
        debug_format!("RET {}: taken", _name)
    }
    else {
        self.pc += 1;
        self.ticks += 8;
        debug_format!("RET {}: not taken", _name)
    }
}
} /* impl GameState */
