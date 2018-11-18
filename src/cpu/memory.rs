use cpu::types;
use cpu::types::Register;
use cpu::types::GameState;

use cpu::debug::Debug;


pub enum Operation {
    Increment, Decrement
}


pub fn load_reg(state: &mut GameState, opcode: u8) -> Debug {
    let high = opcode >> 4;
    let low = opcode & 0xf;
    let dest_idx = (low / 8) + (high - 0x4) * 2;

    let source = &types::REGISTER_LIST[(low % 8) as usize];
    let dest = &types::REGISTER_LIST[dest_idx as usize];
    match (source, dest) {
        (&Register::HL, _) => panic!("Load to/from HL not implemented"),
        (_, &Register::HL) => panic!("Load to/from HL not implemented"),
        (_, _) => {
            let reg_val = state.get_register(source);
            state.set_register(dest, reg_val);
            state.ticks += 4;
            state.pc += 1;
        }
    }

    debug_format!("LD {} => {}",
            types::reg_to_str(source),
            types::reg_to_str(dest))
}

pub fn load_word_imm(state: &mut GameState, opcode: u8,
                     code_bytes: &[u8]) -> Debug {
    let high = opcode >> 4;
    let low = opcode & 0xf;
    let dest_idx = (low >> 3) + (high * 2);

    let reg = &types::REGISTER_LIST[dest_idx as usize];
    match reg {
        &Register::HL => panic!("Load to HL not implemented"),
        _ => {
            state.set_register(reg, code_bytes[0]);
            state.ticks += 8;
        }
    }

    state.pc += 2;

    debug_format!("LD 0x{:02x} => {}", code_bytes[0], types::reg_to_str(reg))
}

fn load_compound_register(state: &mut GameState, opcode: u8,
                          code_bytes: &[u8]) -> Debug {

    let (high, low) = match opcode {
        0x01 => (Register::B, Register::C),
        0x11 => (Register::D, Register::E),
        0x21 => (Register::H, Register::L),
        _ => panic!("Unrecognized opcode: 0x{:02x}", opcode)
    };

    state.set_register(&high, code_bytes[1]);
    state.set_register(&low, code_bytes[0]);

    debug_format!("LD 0x{:02x}{:02x} => {}{}",
        code_bytes[1], code_bytes[0],
        types::reg_to_str(&high), types::reg_to_str(&low))
}

pub fn load_dword_imm(state: &mut GameState, opcode: u8,
                      code_bytes: &[u8]) -> Debug {

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

pub fn store_and_update(state: &mut GameState, operation: Operation) -> Debug {

    let addr = types::get_hl(state);
    let value = state.get_register(&Register::A);
    state.write_mem(addr, value);

    let new_addr = match operation {
        Operation::Decrement => addr.wrapping_sub(1),
        Operation::Increment => addr.wrapping_add(1),
    };

    types::set_hl(state, new_addr);

    state.ticks += 8;
    state.pc += 1;

    debug_format!("LD (HL +/-): A => mem[0x{:04x}]", addr)
}

pub fn load_and_update(state: &mut GameState, operation: Operation) -> Debug {

    let addr = types::get_hl(state);
    let value = state.read_mem(addr);
    state.set_register(&Register::A, value);

    let new_addr = match operation {
        Operation::Decrement => addr.wrapping_sub(1),
        Operation::Increment => addr.wrapping_add(1),
    };

    types::set_hl(state, new_addr);

    state.ticks += 8;
    state.pc += 1;

    debug_format!("LD (HL +/-): mem[0x{:04x}] => A", addr)
}

pub fn store_imm_addr(state: &mut GameState, code_bytes: &[u8]) -> Debug {
    let addr = ((code_bytes[1] as u16) << 8) | code_bytes[0] as u16;
    let value = state.get_register(&Register::A);
    state.write_mem(addr, value);

    state.ticks += 16;
    state.pc += 3;

    debug_format!("LD A => mem[0x{:04x}]", addr)
}

pub fn load_a_mem(state: &mut GameState, code_bytes: &[u8]) -> Debug {
    let addr = (0xff00 as u16).wrapping_add(code_bytes[0] as u16);
    let value = state.read_mem(addr);
    state.set_register(&Register::A, value);

    state.ticks += 12;
    state.pc += 2;

    debug_format!("LD mem[0x{:02x}] => A", addr)
}

pub fn store_a_mem(state: &mut GameState, code_bytes: &[u8]) -> Debug {
    let addr = (0xff00 as u16).wrapping_add(code_bytes[0] as u16);
    let value = state.get_register(&Register::A);
    state.write_mem(addr, value);

    state.ticks += 12;
    state.pc += 2;

    debug_format!("LD A => mem[0x{:02x}]", addr)
}

pub fn store_a_indirect_c(state: &mut GameState) -> Debug {

    let regc = state.get_register(&Register::C);
    let addr = (0xff00 as u16).wrapping_add(regc as u16);
    let value = state.get_register(&Register::A);
    state.write_mem(addr, value);

    state.ticks += 8;
    state.pc += 2;

    debug_format!("LD A => mem[C]")
}

pub fn load_a_indirect_c(state: &mut GameState) -> Debug {

    let regc = state.get_register(&Register::C);
    let addr = (0xff00 as u16).wrapping_add(regc as u16);
    let value = state.read_mem(addr);
    state.set_register(&Register::A, value);

    state.ticks += 8;
    state.pc += 2;

    debug_format!("LD mem[C] => A")
}
