use vstd::prelude::*;

verus!
{
// Metadata management of pages.
//
// You can picture a globally shared, static, gigantic arrary of metadata initialized for each page.
// An entry in the array is called a `MetaSlot`, which contains the metadata of a page. There would
// be a dedicated small "heap" space in each slot for dynamic metadata. You can store anything as the
// metadata of a page as long as it's [`Sync`].
//
// In the implemetation level, the slots are placed in the metadata pages mapped to a certain virtual
// address. It is faster, simpler, safer and more versatile compared with an actual static array
// implementation.


pub mod mapping {
    //! The metadata of each physical page is linear mapped to fixed virtual addresses
    //! in [`FRAME_METADATA_RANGE`].

    use vstd::prelude::*;
    use crate::proofs::basic::{paddr_range,meta_vaddr_range,axiom_size_of_metaslot};
    
    use core::mem::size_of;

    use super::MetaSlot;
    use crate::mm::{kspace::{FRAME_METADATA_RANGE,FRAME_METADATA_BASE_VADDR,FRAME_METADATA_CAP_VADDR}, Paddr, Vaddr, PAGE_SIZE,ADDRESS_WIDTH};
    
    
    /// Converts a physical address of a base page to the virtual address of the metadata slot.
    pub const fn page_to_meta(paddr: Paddr) -> (ret:Vaddr) 
    requires paddr_range(paddr),
    ensures meta_vaddr_range(ret),
    {
        let base = FRAME_METADATA_RANGE.start;
        
        let offset = paddr / PAGE_SIZE;
        
        proof{
            axiom_size_of_metaslot();
            assert((size_of::<MetaSlot>() as u64) ==16);
            assert (FRAME_METADATA_BASE_VADDR + 0xff_ffff_ffff < FRAME_METADATA_CAP_VADDR) by (compute_only);
        }

        base + offset * (size_of::<MetaSlot>() as u64) 
    }

    
    /// Converts a virtual address of the metadata slot to the physical address of the page.
    pub const fn meta_to_page(vaddr: Vaddr) -> (ret:Paddr) 
    requires meta_vaddr_range(vaddr),
    ensures paddr_range(ret),
    {
        let base = FRAME_METADATA_RANGE.start;
        
        proof{
            axiom_size_of_metaslot();
            assert((size_of::<MetaSlot>() as u64) ==16);
            assert(0<=FRAME_METADATA_CAP_VADDR -1- FRAME_METADATA_BASE_VADDR<=0xff_ffff_ffff) by (compute_only);
        }

        let offset = (vaddr - base) / (size_of::<MetaSlot>() as u64);

        offset * PAGE_SIZE
    }
    
    
}

}


use vstd::cell::PCell;

use super::Page;
use crate::mm::{page_table::PageTableEntryTrait,PagingLevel};
use crate::{
    arch::mm::{PageTableEntry}};
use core::{sync::atomic::{AtomicU8,AtomicU32},mem::ManuallyDrop};

verus!{
pub struct PageTablePageMeta<E: PageTableEntryTrait> 
{
    pub level: PagingLevel,
    /// The lock for the page table page.
    pub lock: AtomicU8,
    /// The number of valid PTEs. It is mutable if the lock is held.
    pub nr_children: PCell<u16>,
    _phantom: core::marker::PhantomData<E>,
}

#[repr(u8)]
#[derive(Copy,Clone)]
pub enum PageUsage {
    // The zero variant is reserved for the unused type. Only an unused page
    // can be designated for one of the other purposes.
    #[allow(dead_code)]
    Unused = 0,
    /// The page is reserved or unusable. The kernel should not touch it.
    #[allow(dead_code)]
    Reserved = 1,

    /// The page is used as a frame, i.e., a page of untyped memory.
    Frame = 32,

    /// The page is used by a page table.
    PageTable = 64,
    /// The page stores metadata of other pages.
    Meta = 65,
    /// The page stores the kernel such as kernel code, data, etc.
    Kernel = 66,
}

#[repr(C)]
pub struct FrameMeta {
    // If not doing so, the page table metadata would fit
    // in the front padding of meta slot and make it 12 bytes.
    // We make it 16 bytes. Further usage of frame metadata
    // is welcome to exploit this space.
    _unused_for_layout_padding: [u8; 8],
}

pub /*(super)*/ union MetaSlotInner {
    _frame: ManuallyDrop<FrameMeta>,
    _pt: ManuallyDrop<PageTablePageMeta<PageTableEntry>>,
}

pub /*(in crate::mm)*/ struct MetaSlot {
    /// The metadata of the page.
    ///
    /// It is placed at the beginning of a slot because:
    ///  - the implementation can simply cast a `*const MetaSlot`
    ///    to a `*const PageMeta` for manipulation;
    ///  - the subsequent fields can utilize the end padding of the
    ///    the inner union to save space.
    pub _inner: MetaSlotInner,
    /// To store [`PageUsage`].
    pub /* (super) */ usage: AtomicU8,
    pub /* (super) */ ref_count: AtomicU32,
}

/// All page metadata types must implemented this sealed trait,
/// which ensures that each fields of `PageUsage` has one and only
/// one metadata type corresponding to the usage purpose. Any user
/// outside this module won't be able to add more metadata types
/// and break assumptions made by this module.
///
/// If a page type needs specific drop behavior, it should specify
/// when implementing this trait. When we drop the last handle to
/// this page, the `on_drop` method will be called.
pub trait PageMeta: Sync + Sized {
    //const USAGE: PageUsage;
    spec fn usage() -> PageUsage;

    fn on_drop(page: &mut Page<Self>);
}
}


