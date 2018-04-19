use cpu::types;
use cpu::debug;

pub enum Condition {
    NotZero,
    NoCarry,
}

pub fn jump_uncond_imm(state: &mut types::GameState, code_bytes: &[u8]) -> debug::Debug {
    let new_addr = (code_bytes[1] as u16) << 8 | (code_bytes[0] as u16);
    state.pc = new_addr;
    state.ticks += 16;

    debug_format!("0x{:04x}", new_addr)
}

fn push_value(state: &mut types::GameState, value: u16) {
    /* Make sure the order of POP reverses this order */
    state.sp = state.sp.wrapping_sub(1);
    state.memory[state.sp as usize] = (value & 0xff) as u8;
    state.sp = state.sp.wrapping_sub(1);
    state.memory[state.sp as usize] = (value >> 8) as u8;
}

fn pop_value(state: &mut types::GameState) -> u16 {

    let mut new_val = (state.memory[state.sp as usize] as u16) << 8;
    state.sp = state.sp.wrapping_add(1);
    new_val |= state.memory[state.sp as usize] as u16;
    state.sp = state.sp.wrapping_add(1);

    new_val
}

pub fn call_uncond_imm(state: &mut types::GameState, code_bytes: &[u8]) -> debug::Debug {
    let new_addr = (code_bytes[1] as u16) << 8 | (code_bytes[0] as u16);
    let call_pc = state.pc + 3;

    push_value(state, call_pc);

    state.pc = new_addr;
    state.ticks += 24;

    debug_format!("CALL 0x{:04x}", new_addr)
}

fn ubyte_to_sbyte(value: u8) -> i16 {
    -((value & 0x80) as i16) + ((value & 0x7f) as i16)
}

pub fn jump_cond_imm(state: &mut types::GameState, code_bytes: &[u8],
                condition: Condition) -> debug::Debug {

    let (jump, name) = match condition {
        Condition::NotZero => (!state.flags.zero, "NZ"),
        Condition::NoCarry => (!state.flags.carry, "NC"),
    };

    if jump {
        let offset = ubyte_to_sbyte(code_bytes[0]);
        /* Account for the size of this instruction in the offset */
        state.pc = (state.pc as i32 + offset as i32 + 2) as u16;
        state.ticks += 12;
        debug_format!("JP {} to 0x{:04x}", name, state.pc)
    }
    else {
        state.pc += 2;
        state.ticks += 8;
        debug_format!("JP {}: not taken", name)
    }
}

pub fn ret_cond(state: &mut types::GameState, condition: Condition) -> debug::Debug {

    let (do_return, name) = match condition {
        Condition::NotZero => (!state.flags.zero, "NZ"),
        Condition::NoCarry => (!state.flags.carry, "NC"),
    };

    if do_return {
        state.pc = pop_value(state);
        state.ticks += 20;
        debug_format!("RET {}: taken", name)
    }
    else {
        state.pc += 1;
        state.ticks += 8;
        debug_format!("RET {}: not taken", name)
    }
}
