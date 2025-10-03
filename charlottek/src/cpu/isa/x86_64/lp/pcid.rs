use core::hint::likely;

/// The x86-64 Process Context Identifier
#[derive(Clone, Copy)]
pub struct Pcid(u16);

impl Default for Pcid {
    fn default() -> Pcid {
        //! Returns the shared PCID which clears all its entries on context switch
        Pcid(0)
    }
}

impl From<u16> for Pcid {
    fn from(raw: u16) -> Pcid {
        //! Constructs a Pcid from the raw value if it is in range or returns
        //! the default if it isn't.
        if likely(raw == raw & 0xfff) {
            Pcid(raw)
        } else {
            Pcid::default()
        }
    }
}

impl Into<u16> for Pcid {
    fn into(self) -> u16 {
        self.0
    }
}
