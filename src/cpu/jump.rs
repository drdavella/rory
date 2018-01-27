use cpu::types;


pub fn uncond_imm(state: &mut types::GameState, code_bytes: &[u8]) -> String {
    let new_addr = (code_bytes[1] as u16) << 8 | (code_bytes[0] as u16);
    state.pc = new_addr;
    state.ticks += 16;
    String::from(format!("0x{:04x}", new_addr))
}
