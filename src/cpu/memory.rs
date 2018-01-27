use cpu::types;


pub fn load_reg(state: &mut types::GameState, opcode: u8) -> String {
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

    format!("LD {} => {}",
            types::reg_to_str(source),
            types::reg_to_str(dest))
}
