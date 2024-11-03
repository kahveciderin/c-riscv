use std::fmt::Display;

use super::values::{Immediate, Register, RegisterWithOffset};

#[derive(Debug)]
pub enum Instruction {
    Comment(String),
    Label(String),
    Symbol(String),

    // arithmetic
    Add(Register, Register, Register),
    Addi(Register, Register, Immediate),
    Neg(Register, Register),
    Sub(Register, Register, Register),
    Mul(Register, Register, Register),
    Div(Register, Register, Register),
    Rem(Register, Register, Register),

    // bitwise
    And(Register, Register, Register),
    NotP(Register, Register),
    Or(Register, Register, Register),
    Xor(Register, Register, Register),
    Xori(Register, Register, Immediate),

    // shift
    Sll(Register, Register, Register),
    Srl(Register, Register, Register),

    // load immediate
    LiP(Register, Immediate),

    // load and store
    Lw(Register, RegisterWithOffset),
    Ld(Register, RegisterWithOffset),
    LaP(Register, Immediate),
    Sw(Register, RegisterWithOffset),
    Sd(Register, RegisterWithOffset),

    // jump
    JP(Immediate),
    Jal(Register, Immediate),
    Jalr(Register, RegisterWithOffset),
    CallP(Immediate),
    RetP,

    // branch
    Beq(Register, Register, Immediate),
    BeqzP(Register, Immediate),
    Bne(Register, Register, Immediate),
    BnezP(Register, Immediate),

    // set
    Sltu(Register, Register, Register),
    Sltiu(Register, Register, Immediate),
    SeqzP(Register, Register),
    SnezP(Register, Register),
    SeqP(Register, Register, Register),
}

// pseudoinstructions list: https://riscv.org/wp-content/uploads/2019/12/riscv-spec-20191213.pdf (page 139, Table 25.2)
impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Comment(comment) => write!(f, "# {}", comment),
            Instruction::Label(label) => write!(f, "{}:", label),
            Instruction::Symbol(label) => write!(f, ".{}", label),
            Instruction::JP(imm) => write!(f, "j {}", imm),
            Instruction::Addi(rd, rs1, imm) => write!(f, "addi {}, {}, {}", rd, rs1, imm),
            Instruction::Add(rd, rs1, rs2) => write!(f, "add {}, {}, {}", rd, rs1, rs2),
            Instruction::Sll(rd, rs1, rs2) => write!(f, "sll {}, {}, {}", rd, rs1, rs2),
            Instruction::Srl(rd, rs1, rs2) => write!(f, "srl {}, {}, {}", rd, rs1, rs2),
            Instruction::And(rd, rs1, rs2) => write!(f, "and {}, {}, {}", rd, rs1, rs2),
            Instruction::Or(rd, rs1, rs2) => write!(f, "or {}, {}, {}", rd, rs1, rs2),
            Instruction::Sub(rd, rs1, rs2) => write!(f, "sub {}, {}, {}", rd, rs1, rs2),
            Instruction::Mul(rd, rs1, rs2) => write!(f, "mul {}, {}, {}", rd, rs1, rs2),
            Instruction::Div(rd, rs1, rs2) => write!(f, "div {}, {}, {}", rd, rs1, rs2),
            Instruction::Rem(rd, rs1, rs2) => write!(f, "rem {}, {}, {}", rd, rs1, rs2),
            Instruction::Sw(rs1, rs2) => write!(f, "sw {}, {}", rs1, rs2),
            Instruction::Sd(rs1, rs2) => write!(f, "sd {}, {}", rs1, rs2),
            Instruction::LiP(rd, imm) => write!(f, "li {}, {}", rd, imm),
            Instruction::Lw(rd, rs1) => write!(f, "lw {}, {}", rd, rs1),
            Instruction::Ld(rd, rs1) => write!(f, "ld {}, {}", rd, rs1),
            Instruction::Neg(rd, rs1) => write!(f, "neg {}, {}", rd, rs1),
            Instruction::Jal(rd, symbol) => write!(f, "jal {}, {}", rd, symbol),
            Instruction::Jalr(rd, rs1) => write!(f, "jalr {}, {}", rd, rs1),
            Instruction::Xor(rd, rs1, rs2) => write!(f, "xor {}, {}, {}", rd, rs1, rs2),
            Instruction::Xori(rd, rs1, imm) => write!(f, "xori {}, {}, {}", rd, rs1, imm),
            Instruction::Beq(rs1, rs2, imm) => write!(f, "beq {}, {}, {}", rs1, rs2, imm),
            Instruction::Bne(rs1, rs2, imm) => write!(f, "bne {}, {}, {}", rs1, rs2, imm),
            Instruction::Sltu(rd, rs1, imm) => write!(f, "sltu {}, {}, {}", rd, rs1, imm),
            Instruction::Sltiu(rd, rs1, imm) => write!(f, "sltiu {}, {}, {}", rd, rs1, imm),
            Instruction::CallP(symbol) => write!(f, "call {}", symbol),
            Instruction::RetP => write!(f, "ret"),
            Instruction::NotP(rd, rs1) => write!(f, "not {}, {}", rd, rs1),
            Instruction::BeqzP(rs1, imm) => write!(f, "beqz {}, {}", rs1, imm),
            Instruction::BnezP(rs1, imm) => write!(f, "bnez {}, {}", rs1, imm),
            Instruction::SeqzP(rd, rs1) => write!(f, "seqz {}, {}", rd, rs1),
            Instruction::SnezP(rd, rs1) => write!(f, "snez {}, {}", rd, rs1),
            Instruction::LaP(rd, imm) => write!(f, "la {}, {}", rd, imm),
            Instruction::SeqP(rd, rs1, rs2) => {
                Instruction::Xor(rd.clone(), rs1.clone(), rs2.clone()).fmt(f)?;

                "\n".fmt(f)?;

                Instruction::SeqzP(rd.clone(), rd.clone()).fmt(f)?;

                Ok(())
            }
        }
    }
}
