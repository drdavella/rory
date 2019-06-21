use cpu::types;
use cpu::types::GameState;

use cpu::debug::Debug;


impl GameState {
fn do_op_reg<F>(&mut self, op: F, index: u8, _label: &str) -> Debug
    where F: Fn(u8, u8) -> u8 {

    let source = &types::REGISTER_LIST[index as usize];
    let op0 = self.get_register(&types::Register::A);

    let op1;
    let ticks;

    match source {
        &types::Register::HL => {
            let addr = self.get_hl();
            op1 = self.read_mem(addr);

            ticks = 8;
        },
        _ => {
            op1 = self.get_register(source);
            ticks = 4;
        }
    }

    self.set_register(&types::Register::A, op(op0, op1));

    /* TODO: update flags, etc. */
    self.ticks += ticks;
    self.pc += 1;

    debug_format!("{} {}", _label, types::reg_to_str(source))
}

pub fn add_reg(&mut self, index: u8) -> Debug {
    let add = |x: u8, y: u8| x.wrapping_add(y);
    self.do_op_reg(add, index, "ADD")
}

pub fn and_reg(&mut self, index: u8) -> Debug {
    let and = |x, y| x & y;
    self.do_op_reg(and, index, "AND")
}

pub fn xor_reg(&mut self, index: u8) -> Debug {
    let xor = |x, y| x ^ y;
    self.do_op_reg(xor, index, "XOR")
}

pub fn or_reg(&mut self, index: u8) -> Debug {
    let or = |x, y| (x | y);
    self.do_op_reg(or, index, "OR")
}

pub fn dec_reg(&mut self, opcode: u8) -> Debug {

    let index = ((opcode & 0xf) >> 3) + ((opcode >> 4) << 1);
    let reg = &types::REGISTER_LIST[index as usize];

    let new_val = match reg {
        &types::Register::HL => panic!("DEC HL is not implemented"),
        _ => {
            let new_val = self.get_register(reg).wrapping_sub(1);
            self.set_register(reg, new_val);
            self.ticks += 4;
            new_val
        }
    };

    self.flags.zero = new_val == 0x00;

    self.pc += 1;

    debug_format!("DEC {}", types::reg_to_str(&reg))
}
} /* impl GameState */
