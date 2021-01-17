const MEM_SZ: usize = u16::MAX as usize;

/// `State` holds the complete CPU-side global state, such as the register values
pub struct State {
    // Registers
    pub reg_a: u8,
    pub reg_b: u8,
    pub reg_c: u8,
    pub reg_d: u8,
    pub reg_e: u8,
    pub reg_f: u8,
    pub reg_h: u8,
    pub reg_l: u8,
    pub reg_sp: u16,
    pub reg_pc: u16,
    pub reg_flags: u8,

    // Memory
    mem: [u8; MEM_SZ],

    // Implementation details
    current_insn: &'static crate::isa::Insn,
    current_insn_stage: u8,
    tick_count: u8,
    pub cb_prefix: bool,
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
            mem: [0u8; MEM_SZ],
            current_insn: &crate::isa::NOP,
            current_insn_stage: 0,
            tick_count: 0,
            cb_prefix: false,
        }
    }

    /// Attempt to retrieve the current value of `addr` in memory
    #[inline]
    pub fn peek(&self, addr: u16) -> u8 {
        todo!();
    }

    /// Attempt to update the current value of `addr` in memory to `v`
    /// Various memory-mapped registers reside at certain memory addresses, so this may not result in a "real"
    /// memory state change.
    #[inline]
    pub fn poke(&mut self, addr: u16, v: u8) {
        todo!();
    }

    /// Executes one CPU clock tick
    pub fn tick(&mut self) {
        // execute next instruction stage if counter == 0
        if self.tick_count == 0 {
            let f = self.current_insn.stages[self.current_insn_stage as usize];
            f(todo!(), self);
        }

        // tick counter
        self.tick_count += 1;

        // fetch next instruction if we're on the last stage of the current instruction
    }
}
