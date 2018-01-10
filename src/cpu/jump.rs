use cpu::types;


pub fn uncond_imm(state: &mut types::GameState, code_bytes: &[u8]) -> String {
    state.pc += 1;
    String::from(format!("0x{:02x}", code_bytes[0]))
}
