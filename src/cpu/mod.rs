mod types;
mod memory;
mod jump;
mod alu;


pub fn emulate(rom_array: Vec<u8>) {

    /* initialize emulator state */
    let mut state = types::GameState{
        pc: 0x100,
        ticks: 0,
        regs: types::Registers{
            a: 0, b:0, c:0, d:0, e:0, h:0, l:0
        },
    };

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

    let msg = match opcode {
        /* NOP */
        0x00 => { state.pc += 1; state.ticks+= 4; String::from("NOP") },
        /* HALT (must be matched before LD REG) */
        0x76 => panic!("HALT"),
        /* UNCONDITIONAL JUMP IMM */
        0xc3 => jump::uncond_imm(state, code_bytes),
        /* DISABLE INTERRUPT */
        0xf3 => { state.pc +=1; state.ticks += 4; String::from("DI") },
        /* LD REG -> REG */
        0x40 ... 0x7f => memory::load_reg(state, opcode),
        /* XOR REG */
        0xa8 ... 0xaf => alu::xor_reg(state, (opcode & 0x0f) - 0x8),
        /* UNRECOGNIZED INSTRUCTION */
        _ => panic!("Unrecognized opcode 0x{:02x} at pc 0x{:04x}", opcode, pc)
    };

    println!("pc[0x{:04x}]=0x{:02x} {}", pc, opcode, msg);
}
