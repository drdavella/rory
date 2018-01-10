mod types;
mod jump;


pub fn emulate(rom_array: Vec<u8>) {

    /* initialize emulator state */
    let mut state = types::GameState{ pc: 0x100 };

    loop {
        decode(&rom_array, &mut state);
    }
}

fn decode(rom_array: &Vec<u8>, state: &mut types::GameState) {
    let pc = state.pc as usize;
    let opcode = rom_array[pc as usize];
    let code_bytes = match rom_array.get(pc+1..) {
        Some(result) => result,
        _ => panic!("Bad code bytes at pc 0x{:04}", pc)
    };

    let msg = match opcode {
        /* NOP */
        0x00 => { state.pc += 1; String::from("NOP") }
        /* UNCONDITIONAL JUMP IMM */
        0xc3 => jump::uncond_imm(state, code_bytes),
        /* UNRECOGNIZED INSTRUCTION */
        _ => panic!("Unrecognized opcode 0x{:02x} at pc 0x{:04x}", opcode, pc)
    };

    println!("pc[0x{:04x}]=0x{:02x} {}", pc, opcode, msg);
}
