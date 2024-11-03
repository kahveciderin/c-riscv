use std::fmt::Display;

const USE_ABI_REGISTER_NAMES: bool = true;

#[derive(Debug, Clone)]
pub enum Register {
    Zero = 0,
    Ra = 1,
    Sp = 2,
    Gp = 3,
    Tp = 4,
    T0 = 5,
    T1 = 6,
    T2 = 7,
    Fp = 8,
    S1 = 9,
    A0 = 10,
    A1 = 11,
    A2 = 12,
    A3 = 13,
    A4 = 14,
    A5 = 15,
    A6 = 16,
    A7 = 17,
    S2 = 18,
    S3 = 19,
    S4 = 20,
    S5 = 21,
    S6 = 22,
    S7 = 23,
    S8 = 24,
    S9 = 25,
    S10 = 26,
    S11 = 27,
    T3 = 28,
    T4 = 29,
    T5 = 30,
    T6 = 31,
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if USE_ABI_REGISTER_NAMES {
            let s = match self {
                Register::Zero => "zero",
                Register::Ra => "ra",
                Register::Sp => "sp",
                Register::Gp => "gp",
                Register::Tp => "tp",
                Register::T0 => "t0",
                Register::T1 => "t1",
                Register::T2 => "t2",
                Register::Fp => "s0",
                Register::S1 => "s1",
                Register::A0 => "a0",
                Register::A1 => "a1",
                Register::A2 => "a2",
                Register::A3 => "a3",
                Register::A4 => "a4",
                Register::A5 => "a5",
                Register::A6 => "a6",
                Register::A7 => "a7",
                Register::S2 => "s2",
                Register::S3 => "s3",
                Register::S4 => "s4",
                Register::S5 => "s5",
                Register::S6 => "s6",
                Register::S7 => "s7",
                Register::S8 => "s8",
                Register::S9 => "s9",
                Register::S10 => "s10",
                Register::S11 => "s11",
                Register::T3 => "t3",
                Register::T4 => "t4",
                Register::T5 => "t5",
                Register::T6 => "t6",
            };

            write!(f, "{}", s)
        } else {
            let register_number = (*self).clone() as u8;
            write!(f, "x{register_number}")
        }
    }
}

#[derive(Debug)]
pub struct RegisterWithOffset(pub Immediate, pub Register);

impl Display for RegisterWithOffset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.0, self.1)
    }
}
#[derive(Debug, Clone)]
pub enum Immediate {
    Label(String),
    Number(i32),
}

impl Display for Immediate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Immediate::Label(label) => write!(f, "{}", label),
            Immediate::Number(number) => write!(f, "{}", number),
        }
    }
}

impl From<i32> for Immediate {
    fn from(number: i32) -> Self {
        Immediate::Number(number)
    }
}
