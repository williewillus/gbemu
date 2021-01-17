/// `State` holds the complete CPU-side global state, such as the register values
pub struct State {
    // Registers
    reg_a: u8,
    reg_b: u8,
    reg_c: u8,
    reg_d: u8,
    reg_e: u8,
    reg_f: u8,
    reg_h: u8,
    reg_l: u8,
    reg_sp: u16,
    reg_pc: u16,
    reg_flags: u8,
}

impl State {
    pub fn new() -> Self {
        Self {
            reg_a: 0,
            reg_b: 0,
            reg_c: 0,
            reg_d: 0,
            reg_e: 0,
            reg_f: 0,
            reg_h: 0,
            reg_l: 0,
            reg_sp: 0xFFFE,
            reg_pc: 0x100,
            reg_flags: 0,
        }
    }

}
