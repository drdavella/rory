use cpu::types;
use cpu::types::GameState;

use cpu::debug::Debug;


fn do_op_reg<F>(op: F, state: &mut GameState, index: u8, _label: &str) -> Debug
    where F: Fn(u8, u8) -> u8 {

    let source = &types::REGISTER_LIST[index as usize];
    let op0 = types::get_register(state, &types::Register::A);

    let op1;
    let ticks;

    match source {
        &types::Register::HL => {
            let addr = types::get_hl(state);
            op1 = state.read_mem(addr);

            ticks = 8;
        },
        _ => {
            op1 = types::get_register(state, source);
            ticks = 4;
        }
    }

    types::set_register(state, &types::Register::A, op(op0, op1));

    /* TODO: update flags, etc. */
    state.ticks += ticks;
    state.pc += 1;

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

pub fn or_reg(state: &mut GameState, index: u8) -> Debug {
    let or = |x, y| (x | y);
    do_op_reg(or, state, index, "OR")
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
