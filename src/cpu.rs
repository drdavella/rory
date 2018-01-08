struct GameState {
    pc: u16
}

pub fn emulate(rom_array: Vec<u8>) {

    /* initialize emulator state */
    let mut state = GameState{ pc: 0 };

    loop {
        decode(&rom_array, &mut state);
    }
}

fn decode(rom_array: &Vec<u8>, state: &mut GameState) {
    println!("0x{:02x}", rom_array[state.pc as usize]);
    state.pc += 1;
}
