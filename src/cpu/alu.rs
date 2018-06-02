use cpu::types;
use cpu::types::GameState;

use cpu::debug::Debug;


fn do_op_reg<F>(op: F, state: &mut GameState, index: u8, _label: &str) -> Debug
    where F: Fn(u8, u8) -> u8 {

    let source = &types::REGISTER_LIST[index as usize];
    match source {
        &types::Register::HL => panic!("AND from HL is not implemented"),
        _ => {
            let op0 = types::get_register(state, &types::Register::A);
            let op1 = types::get_register(state, source);
            types::set_register(state, &types::Register::A, op(op0, op1));
            state.ticks += 4;
            state.pc += 1;
        }
    }
    /* TODO: update flags, etc. */

    debug_format!("{} {}", _label, types::reg_to_str(source))
}

pub fn add_reg(state: &mut GameState, index: u8) -> Debug {
    let add = |x: u8, y: u8| x.wrapping_add(y);
    do_op_reg(add, state, index, "ADD")
}

pub fn and_reg(state: &mut GameState, index: u8) -> Debug {
    let and = |x, y| x & y;
    do_op_reg(and, state, index, "AND")
}

pub fn xor_reg(state: &mut GameState, index: u8) -> Debug {
    let xor = |x, y| x ^ y;
    do_op_reg(xor, state, index, "XOR")
}

pub fn dec_reg(state: &mut GameState, opcode: u8) -> Debug {

    let index = ((opcode & 0xf) >> 3) + ((opcode >> 4) << 1);
    let reg = &types::REGISTER_LIST[index as usize];

    let new_val = match reg {
        &types::Register::HL => panic!("DEC HL is not implemented"),
        _ => {
            let new_val = types::get_register(state, reg).wrapping_sub(1);
            types::set_register(state, reg, new_val);
            state.ticks += 4;
            new_val
        }
    };

    state.flags.zero = new_val == 0x00;

    state.pc += 1;

    debug_format!("DEC {}", types::reg_to_str(&reg))
}
