use cpu::types;
use cpu::types::GameState;

use cpu::debug::Debug;


type OpResult = (u8, u8, u8, Debug);


impl GameState {

fn do_op_reg<F>(&mut self, op: F, index: u8, _label: &str) -> OpResult
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

    let result = op(op0, op1);
    self.set_register(&types::Register::A, result);

    self.ticks += ticks;
    self.pc += 1;

    let dbg = debug_format!("{} {:?}", _label, source);
    (op0, op1, result, dbg)
}

pub fn add_reg(&mut self, index: u8) -> Debug {
    let add = |x: u8, y: u8| x.wrapping_add(y);
    let (op0, op1, result, output) = self.do_op_reg(add, index, "ADD");

    self.flags.zero = result == 0;
    self.flags.subtract = false;
    self.flags.halfcarry = (op0 & 0xf) + (op1 & 0xf) > 0xf;
    self.flags.carry = result < op0 || result < op1;

    output
}

pub fn and_reg(&mut self, index: u8) -> Debug {
    let and = |x, y| x & y;
    let (_op0, _op1, result, output) = self.do_op_reg(and, index, "AND");

    self.flags.zero = result == 0;
    self.flags.subtract = false;
    self.flags.halfcarry = true;
    self.flags.carry = false;

    output
}

pub fn xor_reg(&mut self, index: u8) -> Debug {
    let xor = |x, y| x ^ y;
    let (_op0, _op1, result, output) = self.do_op_reg(xor, index, "XOR");

    self.flags.zero = result == 0;
    self.flags.subtract = false;
    self.flags.halfcarry = false;
    self.flags.carry = false;

    output
}

pub fn or_reg(&mut self, index: u8) -> Debug {
    let or = |x, y| (x | y);
    let (_op0, _op1, result, output) = self.do_op_reg(or, index, "OR");

    self.flags.zero = result == 0;
    self.flags.subtract = false;
    self.flags.halfcarry = false;
    self.flags.carry = false;

    output
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

    self.flags.zero = new_val == 0;
    self.flags.subtract = true;
    /* Set if no borrow from bit 4 */
    self.flags.halfcarry = new_val != 0x0f;
    /* DOES NOT UPDATE CARRY FLAG */

    self.pc += 1;

    debug_format!("DEC {:?}", reg)
}
} /* impl GameState */
