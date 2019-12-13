//! This module defines `Arm64Inst` and friends, which implement `MachInst`.

use crate::ir::Type;
use crate::isa::arm64::registers::*;
use crate::isa::registers::RegClass;
use crate::machinst::*;
use crate::{mach_args, mach_ops};

mach_ops!(Arm64Op, {
    Add,
    AddS,
    Sub,
    SubS,
    Cmp,
    Cmn,
    Neg,
    NegS,
    Mov,
    MovI,
    And,
    AndS,
    Orr,
    Orn,
    Eor,
    Eon,
    Bic,
    BicS,
    Tst,
    Asr,
    Lsl,
    Lsr,
    Ror,
    Asrv,
    Lslv,
    Lsrv,
    Rorv,
    Cls,
    Clz,
    Adc,
    AdcS,
    Csel
});

mach_args!(Arm64Arg, Arm64ArgKind, {
    Imm(Arm64ShiftedImm),
    Reg(),
    ShiftedReg(Arm64ShiftOp, usize),
    ExtendedReg(Arm64ExtendOp, usize),
    Mem(Arm64MemArg)
});

impl MachInstArg for Arm64Arg {
    fn num_regs(&self) -> usize {
        match self {
            &Arm64Arg::Imm(..) => 0,
            &Arm64Arg::Reg(..) | &Arm64Arg::ShiftedReg(..) | &Arm64Arg::ExtendedReg(..) => 1,
            &Arm64Arg::Mem(ref m) => m.num_regs(),
        }
    }

    fn regclass_for_type(ty: Type) -> RegClass {
        if ty.is_int() || ty.is_bool() {
            GPR
        } else {
            FPR
        }
    }
}

/// A shifted immediate value.
#[derive(Clone, Debug)]
pub struct Arm64ShiftedImm {
    bits: usize,
    shift: usize,
}

/// A shift operator for a register or immediate.
#[derive(Clone, Debug)]
pub enum Arm64ShiftOp {
    ASR,
    LSR,
    LSL,
    ROR,
}

/// An extend operator for a register.
#[derive(Clone, Debug)]
pub enum Arm64ExtendOp {
    SXTB,
    SXTH,
    SXTW,
    XSTX,
    UXTB,
    UXTH,
    UXTW,
    UXTX,
}

/// A memory argument to load/store, encapsulating the possible addressing modes.
#[derive(Clone, Debug)]
pub enum Arm64MemArg {
    Base,
    BaseImm(usize),
    BaseOffsetShifted(usize),
    BaseImmPreIndexed(usize),
    BaseImmPostIndexed(usize),
    PCRel(usize), // TODO: what is the right type for a label reference?
}

impl Arm64MemArg {
    fn num_regs(&self) -> usize {
        match self {
            &Arm64MemArg::Base => 1,
            &Arm64MemArg::BaseImm(..) => 1,
            &Arm64MemArg::BaseOffsetShifted(..) => 2,
            &Arm64MemArg::BaseImmPreIndexed(..) => 1,
            &Arm64MemArg::BaseImmPostIndexed(..) => 1,
            &Arm64MemArg::PCRel(..) => 0,
        }
    }
}

#[derive(Clone, Debug)]
enum Arm64Cond {
    Eq,
    Ne,
    Hs,
    Lo,
    Mi,
    Pl,
    Vs,
    Vc,
    Hi,
    Ls,
    Ge,
    Lt,
    Gt,
    Le,
    Al,
    Nv,
}
