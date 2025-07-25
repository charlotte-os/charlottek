use core::arch::asm;
use core::mem::MaybeUninit;

static mut IDTR: MaybeUninit<Idtr> = MaybeUninit::uninit();

const N_INTERRUPT_VECTORS: usize = 256;

#[derive(Debug)]
#[repr(C, align(16))]
pub struct Idt {
    pub gates: [InterruptGate; 256],
}

impl Idt {
    pub const fn new() -> Self {
        Idt {
            gates: [InterruptGate::new(); 256],
        }
    }

    pub fn set_gate(
        &mut self,
        index: usize,
        isr_ptr: unsafe extern "C" fn(),
        segment_selector: u16,
        is_trap: bool,
        is_present: bool,
    ) {
        let gate = &mut self.gates[index];
        let isr_addr = isr_ptr as u64;

        gate.addr0 = u16::try_from(isr_addr & 0xffff).unwrap();
        gate.segment_selector = segment_selector;
        gate.reserved_ist_index = 0u8; // the IST is not used
        gate.flags = if is_trap { 0b1111u8 } else { 0b1110u8 }; //gate type
        gate.flags &= !(0b1u8 << 4); //reserved bit
        gate.flags &= !(0b11u8 << 5); //privilege ring required to use gate
        if is_present {
            gate.flags |= 0b1u8 << 7;
        } else {
            gate.flags &= !(0b1u8 << 7);
        }
        gate.addr1 = ((isr_addr & (0xffff << 16)) >> 16) as u16;
        gate.addr2 = ((isr_addr & (0xffffffff << 32)) >> 32) as u32;
        gate.reserved = 0u32;
    }

    #[allow(unused)]
    pub fn set_present(&mut self, index: usize) {
        if index < 256 {
            self.gates[index].flags |= 0b1u8 << 7;
        }
    }

    #[allow(unused)]
    pub fn clear_present(&mut self, index: usize) {
        if index < 256 {
            self.gates[index].flags &= !(0b1u8 << 7);
        }
    }

    pub fn load(&self) {
        unsafe {
            IDTR.write(Idtr::new(
                size_of::<InterruptGate>() as u16 * N_INTERRUPT_VECTORS as u16 - 1u16,
                self as *const Idt as u64,
            ));
            asm_load_idt(IDTR.as_ptr());
        }
    }
}
#[derive(Clone, Copy, Debug)]
#[repr(C, packed(1))]
pub struct InterruptGate {
    addr0: u16,
    segment_selector: u16,
    reserved_ist_index: u8,
    flags: u8,
    addr1: u16,
    addr2: u32,
    reserved: u32,
}

impl InterruptGate {
    const fn new() -> Self {
        InterruptGate {
            addr0: 0u16,
            segment_selector: 0u16,
            reserved_ist_index: 0u8,
            flags: 0u8,
            addr1: 0u16,
            addr2: 0u32,
            reserved: 0u32,
        }
    }
}

#[repr(C, packed)]
struct Idtr {
    size: u16,
    base: u64,
}

impl Idtr {
    fn new(size: u16, base: u64) -> Self {
        Idtr { size, base }
    }
}

#[inline(always)]
unsafe fn asm_load_idt(idtr: *const Idtr) {
    unsafe {
        asm!("lidt [rdi]", in("rdi") idtr);
    }
}
