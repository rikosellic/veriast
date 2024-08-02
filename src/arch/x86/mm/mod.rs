use vstd::prelude::*;


use crate::{
    mm::{
        page_table::PageTableEntryTrait,
        Paddr, PagingLevel, Vaddr, 
    }
};
verus!{
// Modifided!!! Used to belong to PageTableFlags.
pub const PRESENT:u64 =         1 << 0;
/// Controls whether writes to the mapped frames are allowed.
pub const WRITABLE:u64 =        1 << 1;
/// Controls whether accesses from userspace (i.e. ring 3) are permitted.
pub const USER:u64 =            1 << 2;
/// If this bit is set, a “write-through” policy is used for the cache, else a “write-back”
/// policy is used.
pub const WRITE_THROUGH:u64 =   1 << 3;
/// Disables caching for the pointed entry is cacheable.
pub const NO_CACHE:u64 =        1 << 4;
/// Whether this entry has been used for linear-address translation.
pub const ACCESSED:u64 =        1 << 5;
/// Whether the memory area represented by this entry is modified.
pub const DIRTY:u64 =           1 << 6;
/// Only in the non-starting and non-ending levels, indication of huge page.
pub const HUGE:u64 =            1 << 7;
/// Indicates that the mapping is present in all address spaces, so it isn't flushed from
/// the TLB on an address space switch.
pub const GLOBAL:u64 =          1 << 8;
/// TDX shared bit.
#[cfg(feature = "intel_tdx")]
pub const SHARED:u64 =          1 << 51;
/// Forbid execute codes on the page. The NXE bits in EFER msr must be set.
pub const NO_EXECUTE:u64 =      1 << 63;

#[derive(Clone, Copy)]
pub struct PageTableEntry(u64);

#[cfg(not(feature = "intel_tdx"))]
//Modified!!! Used to belong to PageTableEntry.
/// 51:12
const PHYS_ADDR_MASK: u64 = 0xF_FFFF_FFFF_F000;
#[cfg(feature = "intel_tdx")]
const PHYS_ADDR_MASK: u64 = 0x7_FFFF_FFFF_F000;
const PROP_MASK: u64 = !PHYS_ADDR_MASK & !HUGE;

impl PageTableEntryTrait for PageTableEntry {
    fn is_present(&self) -> bool {
        self.0 & PRESENT != 0
    }
    
    fn new_pt(paddr: Paddr) -> Self {
        // In x86 if it's an intermediate PTE, it's better to have the same permissions
        // as the most permissive child (to reduce hardware page walk accesses). But we
        // don't have a mechanism to keep it generic across architectures, thus just
        // setting it to be the most permissive.
        let flags = PRESENT
            | WRITABLE
            | USER;
        Self(paddr & PHYS_ADDR_MASK | flags)
    }

    fn is_last(&self, level: PagingLevel) -> bool {
        level == 1 || (self.0 & HUGE != 0)
    }

    fn paddr(&self) -> Paddr {
        self.0 & PHYS_ADDR_MASK
    }
}
}