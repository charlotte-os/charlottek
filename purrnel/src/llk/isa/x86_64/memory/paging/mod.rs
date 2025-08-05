pub mod pte;
pub mod pth_walker;

use core::arch::asm;

use super::MemoryInterfaceImpl;
use crate::llk::isa::interface::memory::{AddressSpaceInterface, MemoryInterface, MemoryMapping};
use crate::memory::pmem::VAddr;

pub const PAGE_SIZE: usize = 4096;
pub const N_PAGE_TABLE_ENTRIES: usize = 512;
pub type PageTable = [pte::PageTableEntry; N_PAGE_TABLE_ENTRIES];

pub fn is_pagetable_unused(table_ptr: *const PageTable) -> bool {
    unsafe {
        for i in 0..N_PAGE_TABLE_ENTRIES {
            if (*table_ptr)[i].is_present() {
                return false;
            }
        }
    }
    true
}

pub struct AddressSpace {
    // control register 3 i.e. page table base register
    cr3: u64,
}

impl AddressSpaceInterface for AddressSpace {
    fn get_current() -> Self {
        let cr3: u64;
        unsafe {
            asm!("mov {}, cr3", out(reg) cr3);
        }
        AddressSpace { cr3 }
    }

    fn load(&self) -> Result<(), <MemoryInterfaceImpl as MemoryInterface>::Error> {
        unsafe {
            asm!("mov cr3, {}", in(reg) self.cr3);
        }
        Ok(())
    }

    fn find_free_region(
        &self,
        n_pages: usize,
        range: (VAddr, VAddr),
    ) -> Result<<MemoryInterfaceImpl as MemoryInterface>::VAddr, <MemoryInterfaceImpl as MemoryInterface>::Error> {
        todo!()
    }

    fn map_page(&mut self, mapping: MemoryMapping) -> Result<(), <MemoryInterfaceImpl as MemoryInterface>::Error> {
        let mut walker = pth_walker::PthWalker::new(self, mapping.vaddr);
        walker.walk()?;
        walker.map_page(
            mapping.paddr,
            mapping.page_type.is_writable(),
            mapping.page_type.is_user_accessible(),
            mapping.page_type.is_no_execute(),
        )?;
        Ok(())
    }

    fn unmap_page(
        &mut self,
        vaddr: <MemoryInterfaceImpl as MemoryInterface>::VAddr,
    ) -> Result<(), <MemoryInterfaceImpl as MemoryInterface>::Error> {
        if <VAddr as Into<usize>>::into(vaddr) == 0 {
            return Err(<MemoryInterfaceImpl as MemoryInterface>::Error::NullVAddrNotAllowed);
        }
        if vaddr.page_offset() != 0 {
            return Err(<MemoryInterfaceImpl as MemoryInterface>::Error::VAddrNotPageAligned);
        }
        let mut walker = pth_walker::PthWalker::new(self, vaddr);
        walker.unmap_page()?;
        Ok(())
    }

    fn is_mapped(
        &self,
        vaddr: <MemoryInterfaceImpl as MemoryInterface>::VAddr,
    ) -> Result<bool, <MemoryInterfaceImpl as MemoryInterface>::Error> {
        todo!()
    }

    fn translate_address(
        &self,
        vaddr: super::address::vaddr::VAddr,
    ) -> Result<super::address::paddr::PAddr, <MemoryInterfaceImpl as MemoryInterface>::Error> {
        let mut walker = pth_walker::PthWalker::new(self, vaddr);
        walker.walk()?;
        let paddr = unsafe { (*(walker.pt_ptr))[vaddr.pt_index()].get_frame().into() };
        Ok(paddr)
    }
}
