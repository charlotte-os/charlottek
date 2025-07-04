use core::arch::global_asm;
use core::mem::size_of;
use core::ptr;

#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug)]
struct SegmentDescriptor {
    limit0: u16,
    base0: u16,
    base1: u8,
    access_byte: u8,
    limit1_flags: u8,
    base2: u8,
}

impl SegmentDescriptor {
    fn new() -> Self {
        SegmentDescriptor {
            limit0: 0,
            base0: 0,
            base1: 0,
            access_byte: 0,
            limit1_flags: 0,
            base2: 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed(1))]
struct TssDescriptor {
    low:  u64, // Low 64 bits of the descriptor
    high: u64, // High 64 bits of the descriptor
}

impl TssDescriptor {
    fn new() -> Self {
        TssDescriptor { low: 0, high: 0 }
    }
}

#[derive(Debug)]
#[repr(C, packed(1))]
pub struct Gdt {
    segment_descs: [SegmentDescriptor; 5],
    tss_desc: TssDescriptor,
}

impl Gdt {
    pub fn new(tss: &Tss) -> Self {
        unsafe {
            core::arch::asm!("cli");
        }

        let mut gdt = Gdt {
            segment_descs: [SegmentDescriptor::new(); 5],
            tss_desc: TssDescriptor::new(),
        };

        //Null descriptor
        gdt.set_segment_desc(0, 0, 0, 0, 0);
        //Kernel Mode Code Segment
        gdt.set_segment_desc(1, 0, 0xfffff, 0x9a, 0xa);
        //Kernel Mode Data Segment
        gdt.set_segment_desc(2, 0, 0xfffff, 0x92, 0xc);
        //User Mode Code Segment
        gdt.set_segment_desc(3, 0, 0xfffff, 0xfa, 0xa);
        //User Mode Data Segment
        gdt.set_segment_desc(4, 0, 0xfffff, 0xf2, 0xc);
        //Task State Segment
        gdt.set_tss_desc(ptr::addr_of!(*tss) as u64, size_of::<Tss>() as u32);

        unsafe {
            core::arch::asm!("sti");
        }

        gdt
    }

    fn set_tss_desc(&mut self, base: u64, limit: u32) {
        let low = ((limit as u64) & 0xFFFF)
            | ((base & 0xFFFFFF) << 16)
            | ((0x89u64) << 40) // Type for TSS
            | ((limit as u64 & 0xF0000) << 32)
            | ((base & 0xFF000000) << 32);
        let high = (base >> 32) & 0xffffffff;

        self.tss_desc.low = low;
        self.tss_desc.high = high;
    }

    fn set_segment_desc(
        &mut self,
        index: usize,
        base: u32,
        limit: u32,
        access_byte: u8,
        flags: u8,
    ) {
        let dest_sd = &mut (self.segment_descs[index]);

        dest_sd.limit0 = (limit & 0xffff) as u16;
        dest_sd.limit1_flags = ((limit & 0xff0000) >> 16) as u8; // Only the lower 4 bits of this field encodes limit bits

        dest_sd.base0 = (base & 0xffff) as u16;
        dest_sd.base1 = ((base & 0xff0000) >> 16) as u8;
        dest_sd.base2 = ((base & 0xff000000) >> 24) as u8;

        dest_sd.access_byte = access_byte;

        dest_sd.limit1_flags |= (flags & 0xff) << 4; // The upper 4 bits
        // of this field
        // encodes flags
    }

    pub fn load(&self) {
        unsafe {
            asm_load_gdt(self);
        }
    }

    pub fn reload_segment_regs() {
        unsafe {
            asm_reload_segment_regs();
        }
    }

    pub fn load_tss() {
        unsafe {
            asm_load_tss();
        }
    }
}

#[derive(Debug)]
#[repr(C, packed(1))]
pub struct Tss {
    res: u32,
    rsp0: u64,
    unused: [u8; 90],
    iopb: u16,
}

impl Tss {
    pub fn new(rsp0: u64) -> Self {
        Tss {
            res: 0,
            rsp0,
            unused: [0u8; 90],
            iopb: size_of::<Tss>() as u16,
        }
    }
}

global_asm! { include_str!("gdt.asm") }
unsafe extern "C" {
    fn asm_load_gdt(gdt: *const Gdt);
    fn asm_reload_segment_regs();
    fn asm_load_tss();
}
