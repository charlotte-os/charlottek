use core::arch::asm;
use core::ops::Add;

use crate::cpu::isa::interface::io::{IReg8Ifce, OReg8Ifce};

pub struct Io;
impl crate::cpu::isa::interface::io::IoIfce for Io {
    type Error = core::convert::Infallible;
    type Reg8 = Reg8;
}

#[derive(Copy, Clone, Debug)]
pub enum Reg8 {
    IoPort(u16),
    Mmio(*mut u8),
}

impl IReg8Ifce for Reg8 {
    fn read(&self) -> u8 {
        match self {
            Reg8::IoPort(port) => {
                let value: u8;
                unsafe {
                    asm!(
                        "in al, dx",
                        in("dx") *port,
                        out("al") value,
                    );
                }
                value
            }
            Reg8::Mmio(address) => unsafe { core::ptr::read_volatile(*address) },
        }
    }
}

impl OReg8Ifce for Reg8 {
    fn write(&self, value: u8) {
        match self {
            Reg8::IoPort(port) => unsafe {
                asm!(
                    "out dx, al",
                    in("dx") *port,
                    in("al") value,
                );
            },
            Reg8::Mmio(address) => unsafe { core::ptr::write_volatile(*address, value) },
        }
    }
}

impl Add<u16> for Reg8 {
    type Output = Reg8;

    fn add(self, rhs: u16) -> Self::Output {
        match self {
            Reg8::IoPort(port) => Reg8::IoPort(port + rhs),
            Reg8::Mmio(address) => {
                Reg8::Mmio(unsafe { (address as *mut u8).add(rhs as usize) as *mut u8 })
            }
        }
    }
}
