#[macro_use]
mod debug;
mod types;
mod memory;
mod jump;
mod alu;


pub fn emulate(rom_array: Vec<u8>) {

    debug_println!("Cartridge type: 0x{:02x}", rom_array[0x147]);
    debug_println!("ROM size:       0x{:02x}", rom_array[0x148]);
    debug_println!("RAM size:       0x{:02x}", rom_array[0x149]);

    /* initialize emulator state */
    let mut state = types::GameState::initialize(0x100, 0xfff3);

    loop {
        decode(&rom_array, &mut state);
    }
}

fn decode(rom_array: &Vec<u8>, state: &mut types::GameState) {
    let pc = state.pc as usize;
    let opcode = rom_array[pc];
    let code_bytes = match rom_array.get(pc+1..) {
        Some(result) => result,
        _ => panic!("Bad code bytes at pc 0x{:04}", pc)
    };

    let _msg = match opcode {
        /* NOP */
        0x00 => { state.pc += 1; state.ticks+= 4; debug_format!("NOP") },
        /* HALT (must be matched before LD REG) */
        0x76 => panic!("HALT"),
        /* UNCONDITIONAL JUMP IMM */
        0xc3 => jump::jump_uncond_imm(state, code_bytes),
        0xcd => jump::call_uncond_imm(state, code_bytes),
        0x20 => jump::jump_cond_imm(state, code_bytes, jump::Condition::NotZero),
        /* DISABLE INTERRUPT */
        0xf3 => { state.pc +=1; state.ticks += 4; debug_format!("DI") },
        /* LD REG -> REG */
        0x40 ... 0x7f => memory::load_reg(state, opcode),
        /* ADD REG */
        0x80 ... 0x87 => alu::add_reg(state, (opcode & 0x0f)),
        /* AND REG */
        0xa0 ... 0xa7 => alu::and_reg(state, (opcode & 0x0f)),
        /* XOR REG */
        0xa8 ... 0xaf => alu::xor_reg(state, (opcode & 0x0f) - 0x8),
        /* OR REG */
        0xb0 ... 0xb7 => alu::or_reg(state, (opcode & 0xf)),
        /* DEC REG */
        0x05 | 0x15 | 0x25 | 0x35 | 0x0d | 0x1d | 0x2d | 0x3d =>
            alu::dec_reg(state, opcode),
        /* STORE AND INC/DEC */
        0x22 => memory::store_and_update(state, memory::Operation::Increment),
        0x32 => memory::store_and_update(state, memory::Operation::Decrement),
        /* LOAD AND INC/DEC */
        0x2a => memory::load_and_update(state, memory::Operation::Increment),
        0x3a => memory::load_and_update(state, memory::Operation::Decrement),
        /* LD SINGLE WORD */
        0x06 | 0x16 | 0x26 | 0x36 | 0x0e | 0x1e | 0x2e | 0x3e =>
            memory::load_word_imm(state, opcode, code_bytes),
        /* LD DOUBLE WORD */
        0x01 | 0x11 | 0x21 | 0x31 =>
            memory::load_dword_imm(state, opcode, code_bytes),
        /* LD IMM ADDR */
        0xd0 => jump::ret_cond(state, jump::Condition::NoCarry),
        0xea => memory::store_imm_addr(state, code_bytes),
        /* LD INDIRECT ADDR */
        0xe2 => memory::store_a_indirect_c(state),
        0xf2 => memory::load_a_indirect_c(state),
        /* LDH (a8), A */
        0xe0 => memory::store_a_mem(state, code_bytes),
        /* LDH A, (a8) */
        0xf0 => memory::load_a_mem(state, code_bytes),
        /* UNRECOGNIZED INSTRUCTION */
        _ => panic!("Unrecognized opcode 0x{:02x} at pc 0x{:04x}", opcode, pc)
    };

    debug_println!("pc[0x{:04x}]=0x{:02x} {}", pc, opcode, _msg);
}
