use cpu::types;
use cpu::debug;


pub fn xor_reg(state: &mut types::GameState, index: u8) -> debug::Debug {
    let source = &types::REGISTER_LIST[index as usize];
    match source {
        &types::Register::HL => panic!("Can't XOR from HL"),
        _ => {
            let op0 = types::get_register(state, &types::Register::A);
            let op1 = types::get_register(state, source);
            types::set_register(state, &types::Register::A, op0 ^ op1);
            state.ticks += 4;
            state.pc += 1;
        }
    }

    debug_format!("XOR {}", types::reg_to_str(source))
}

pub fn dec_reg(state: &mut types::GameState, opcode: u8) -> debug::Debug {

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
