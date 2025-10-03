use core::ops::Add;

use crate::cpu::isa::interface::io::{IReg8Ifce, OReg8Ifce};

pub struct Reg8(*mut u8);

impl IReg8Ifce for Reg8 {
    fn read(&self) -> u8 {
        unsafe { core::ptr::read_volatile(self.0) }
    }
}

impl OReg8Ifce for Reg8 {
    fn write(&self, value: u8) {
        unsafe { core::ptr::write_volatile(self.0, value) }
    }
}

impl Add<usize> for Reg8 {
    type Output = Reg8;

    fn add(self, rhs: usize) -> Self::Output {
        Reg8(unsafe { self.0.add(rhs) })
    }
}

impl Add<isize> for Reg8 {
    type Output = Reg8;

    fn add(self, rhs: isize) -> Self::Output {
        Reg8(unsafe { self.0.offset(rhs) })
    }
}
