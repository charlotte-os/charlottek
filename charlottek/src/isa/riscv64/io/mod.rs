use core::ops::Add;

use crate::isa::interface::io::{IReg8Ifce, OReg8Ifce};

pub struct IoReg8(*mut u8);

impl IReg8Ifce for IoReg8 {
    fn read(&self) -> u8 {
        unsafe { core::ptr::read_volatile(self.0) }
    }
}

impl OReg8Ifce for IoReg8 {
    fn write(&self, value: u8) {
        unsafe { core::ptr::write_volatile(self.0, value) }
    }
}

impl Add<usize> for IoReg8 {
    type Output = IoReg8;

    fn add(self, rhs: usize) -> Self::Output {
        IoReg8(unsafe { self.0.add(rhs) })
    }
}

impl Add<isize> for IoReg8 {
    type Output = IoReg8;

    fn add(self, rhs: isize) -> Self::Output {
        IoReg8(unsafe { self.0.offset(rhs) })
    }
}
