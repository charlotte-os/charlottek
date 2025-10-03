pub trait IoIfce {
    type Error: core::fmt::Debug;
    type Reg8: OReg8Ifce + IReg8Ifce;
}

pub trait IReg8Ifce {
    fn read(&self) -> u8;
}

pub trait OReg8Ifce {
    fn write(&self, value: u8);
}
