use std::fmt::Display;

use super::values::{Immediate, Register, RegisterWithOffset};

#[derive(Debug)]
pub enum Instruction {
    Comment(String),
    Label(String),
    Symbol(String),
    Push(Register),
    Pop(Register),

    Sw(Register, RegisterWithOffset),
    Sd(Register, RegisterWithOffset),
    Li(Register, Immediate),
    Lw(Register, RegisterWithOffset),
    Ld(Register, RegisterWithOffset),
    Jal(Register, Immediate),
    Jalr(Register, RegisterWithOffset),

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
    Not(Register, Register),
    Or(Register, Register, Register),
    Xor(Register, Register, Register),
    Xori(Register, Register, Immediate),

    // shift
    Sll(Register, Register, Register),
    Srl(Register, Register, Register),

    // jump
    J(Immediate),
    Call(Immediate),
    Ret,

    // branch
    Beq(Register, Register, Immediate),
    Beqz(Register, Immediate),
    Bne(Register, Register, Immediate),
    Bnez(Register, Immediate),

    // set
    Sltu(Register, Register, Register),
    Sltiu(Register, Register, Immediate),
    Seqz(Register, Register),
    Snez(Register, Register),
    Seq(Register, Register, Register),
}

// pseudoinstructions list: https://riscv.org/wp-content/uploads/2019/12/riscv-spec-20191213.pdf (page 139, Table 25.2)
// todo: worry about immediates longer than 12 bits (using additional instructions)
impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Comment(comment) => write!(f, "# {}", comment),
            Instruction::Label(label) => write!(f, "{}:", label),
            Instruction::Symbol(label) => write!(f, ".{}", label),
            Instruction::Push(reg) => {
                // extend the stack by 16 bytes (the stack must be 16-byte aligned)
                Instruction::Addi(Register::Sp, Register::Sp, (-16).into()).fmt(f)?;

                "\n".fmt(f)?;

                // store the register value at the top of the stack
                Instruction::Sd(reg.clone(), RegisterWithOffset(0.into(), Register::Sp)).fmt(f)?;
                Ok(())
            }
            Instruction::Pop(reg) => {
                // load the register value from the top of the stack
                Instruction::Ld(reg.clone(), RegisterWithOffset(0.into(), Register::Sp)).fmt(f)?;

                "\n".fmt(f)?;

                // shrink the stack by 16 bytes
                Instruction::Addi(Register::Sp, Register::Sp, 16.into()).fmt(f)?;
                Ok(())
            }

            Instruction::J(imm) => write!(f, "j {}", imm), // todo: remove this pseudo-instruction
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
            Instruction::Li(rd, imm) => write!(f, "li {}, {}", rd, imm),
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

            // todo: far calls (https://projectf.io/posts/riscv-jump-function/#far-calls)
            Instruction::Call(symbol) => Instruction::Jal(Register::Ra, (*symbol).clone()).fmt(f),

            Instruction::Ret => {
                Instruction::Jalr(Register::Zero, RegisterWithOffset(0.into(), Register::Ra)).fmt(f)
            }
            Instruction::Not(rd, rs1) => {
                Instruction::Xori(rd.clone(), rs1.clone(), (-1).into()).fmt(f)
            }
            Instruction::Beqz(rs1, imm) => {
                Instruction::Beq(rs1.clone(), Register::Zero, imm.clone()).fmt(f)
            }
            Instruction::Bnez(rs1, imm) => {
                Instruction::Bne(rs1.clone(), Register::Zero, imm.clone()).fmt(f)
            }
            Instruction::Seqz(rd, rs1) => {
                Instruction::Sltiu(rd.clone(), rs1.clone(), 1.into()).fmt(f)
            }
            Instruction::Snez(rd, rs1) => {
                Instruction::Sltu(rd.clone(), Register::Zero, rs1.clone()).fmt(f)
            }
            Instruction::Seq(rd, rs1, rs2) => {
                Instruction::Xor(rd.clone(), rs1.clone(), rs2.clone()).fmt(f)?;

                "\n".fmt(f)?;

                Instruction::Seqz(rd.clone(), rd.clone()).fmt(f);

                Ok(())
            }
        }
    }
}
