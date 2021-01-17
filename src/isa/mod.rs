use crate::cpu::State;

/// Represents the implementation of a related group of instructions, e.g. "all 8-bit reg->reg loads"
/// Stateless, all instances should be statically constructed here. Decoding returns static references to the instances here.
pub struct Insn {
    /// The stages of the instruction.
    /// Because every memory read takes 4 clock ticks, each stage is separated by 4 cycles
    /// Each stage function will be executed on the 1st tick then a gap of 3 ticks will occur before the next stage
    pub stages: &'static [fn(insn: u16, state: &mut State)],

    sealed: (),
}

pub static NOP: Insn = Insn {
    stages: &[
        |_, state| {
            state.reg_pc += 1;
        }
    ],
    sealed: (),
};

/// CB prefix, tells the CPU that the immediately following instruction decode stage
/// should use CB-prefixed decoding
pub static CB_DUMMY: Insn = Insn {
    stages: &[
        |_, state| {
            state.cb_prefix = true;
        }
    ],
    sealed: (),
};
