struct FlagsRegister {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool,
}

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

impl std::convert::From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION
            | (if flag.subtract { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION
            | (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION
            | (if flag.carry { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry,
        }
    }
}

struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: FlagsRegister,
    h: u8,
    l: u8,
}

impl Registers {
    fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xff00) >> 8) as u8;
        self.c = (value & 0xff) as u8;
    }

    fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xff00) >> 8) as u8;
        self.e = (value & 0xff) as u8;
    }

    fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xff00) >> 8) as u8;
        self.l = (value & 0xff) as u8;
    }
}

enum Instruction {
    ADD(ArithmeticTarget),
}

enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

struct CPU {
    registers: Registers,
}

impl CPU {
    fn execute(&mut self, instr: Instruction) {
        match instr {
            Instruction::ADD(target) => {
                match target {
                    ArithmeticTarget::C => {
                        // TODO: implement add on C
                    }
                    _ => { // TODO: more targets
                    }
                }
            }
            _ => { /* TODO: supp more instructions */ }
        }
    }

    fn add(&mut self, val: u8) -> u8 {
        let (new_val, did_overflow) = self.registers.a.overflowing_add(val);
        self.registers.f.zero = new_val == 0;
        self.registers.f.subtract = false; // TODO
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xf) + (val & 0xf) > 0xf;
        new_val
    }
}

fn main() {
    println!("Hello, world!");
}
