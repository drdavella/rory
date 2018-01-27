use cpu::types;


pub fn xor_reg(state: &mut types::GameState, index: u8) -> String {
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

    format!("XOR {}", types::reg_to_str(source))
}
