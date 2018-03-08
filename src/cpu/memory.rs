use cpu::types;
use cpu::debug;


pub enum Operation {
    Increment, Decrement
}


pub fn load_reg(state: &mut types::GameState, opcode: u8) -> debug::Debug {
    let high = opcode >> 4;
    let low = opcode & 0xf;
    let dest_idx = (high / low) * (high - 0x4);

    let source = &types::REGISTER_LIST[low as usize];
    let dest = &types::REGISTER_LIST[dest_idx as usize];
    match (source, dest) {
        (&types::Register::HL, _) => panic!("Load to/from HL not implemented"),
        (_, &types::Register::HL) => panic!("Load to/from HL not implemented"),
        (_, _) => {
            let reg_val = types::get_register(state, source);
            types::set_register(state, dest, reg_val);
            state.ticks += 4;
            state.pc += 1;
        }
    }

    debug_format!("LD {} => {}",
            types::reg_to_str(source),
            types::reg_to_str(dest))
}

pub fn load_word_imm(state: &mut types::GameState, opcode: u8,
                     code_bytes: &[u8]) -> debug::Debug {
    let high = opcode >> 4;
    let low = opcode & 0xf;
    let dest_idx = (low >> 3) + (high * 2);

    let reg = &types::REGISTER_LIST[dest_idx as usize];
    match reg {
        &types::Register::HL => panic!("Load to HL not implemented"),
        _ => {
            types::set_register(state, reg, code_bytes[0]);
            state.ticks += 8;
        }
    }

    state.pc += 2;

    debug_format!("LD 0x{:02x} => {}", code_bytes[0], types::reg_to_str(reg))
}

fn load_compound_register(state: &mut types::GameState, opcode: u8,
                          code_bytes: &[u8]) -> debug::Debug {

    let (high, low) = match opcode {
        0x01 => (types::Register::B, types::Register::C),
        0x11 => (types::Register::D, types::Register::E),
        0x21 => (types::Register::H, types::Register::L),
        _ => panic!("Unrecognized opcode: 0x{:02x}", opcode)
    };

    types::set_register(state, &high, code_bytes[1]);
    types::set_register(state, &low, code_bytes[0]);

    debug_format!("LD 0x{:02x}{:02x} => {}{}",
        code_bytes[1], code_bytes[0],
        types::reg_to_str(&high), types::reg_to_str(&low))
}

pub fn load_dword_imm(state: &mut types::GameState, opcode: u8,
                      code_bytes: &[u8]) -> debug::Debug {

    let msg = match opcode {
        0x31 => {
            /* Load stack pointer */
            state.sp = ((code_bytes[1] as u16) << 8) | (code_bytes[0] as u16);
            debug_format!("LD 0x{:04x} => SP", state.sp)
        }
        _ => {
            load_compound_register(state, opcode, code_bytes)
        }
    };

    state.ticks += 12;
    state.pc += 3;

    msg
}

pub fn store_and_update(state: &mut types::GameState, operation: Operation) -> debug::Debug {

    let addr = types::get_hl(state);
    state.memory[addr as usize] = types::get_register(state, &types::Register::A);

    let new_addr = match operation {
        Operation::Decrement => addr.wrapping_sub(1),
        Operation::Increment => addr.wrapping_add(1),
    };

    types::set_hl(state, new_addr);

    state.ticks += 8;
    state.pc += 1;

    debug_format!("LD (HL +/-): A => mem[0x{:04x}]", addr)
}
