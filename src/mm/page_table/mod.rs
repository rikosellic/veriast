use vstd::prelude::*;

use super::{Paddr,PagingLevel,Vaddr};
use crate::{
    arch::mm::PageTableEntry,
};


verus!{
    /// The interface for defining architecture-specific page table entries.
///
/// Note that a default PTE shoud be a PTE that points to nothing.
pub trait PageTableEntryTrait: Copy + Sized + Sync {
    /// Create a set of new invalid page table flags that indicates an absent page.
    ///
    /// Note that currently the implementation requires an all zero PTE to be an absent PTE.
    //fn new_absent() -> Self {
    //    Self::default()
    //}

    /// If the flags are present with valid mappings.
    fn is_present(&self) -> bool;

    /// Create a new PTE with the given physical address and flags that map to a page.
    //fn new_page(paddr: Paddr, level: PagingLevel, prop: PageProperty) -> Self;

    /// Create a new PTE that map to a child page table.
    fn new_pt(paddr: Paddr) -> Self;

    /// Get the physical address from the PTE.
    /// The physical address recorded in the PTE is either:
    /// - the physical address of the next level page table;
    /// - or the physical address of the page it maps to.
    fn paddr(&self) -> Paddr;

    //fn prop(&self) -> PageProperty;

    //fn set_prop(&mut self, prop: PageProperty);

    /// If the PTE maps a page rather than a child page table.
    ///
    /// The level of the page table the entry resides is given since architectures
    /// like amd64 only uses a huge bit in intermediate levels.
    fn is_last(&self, level: PagingLevel) -> bool;
}
}