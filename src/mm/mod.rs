use vstd::prelude::*;
use vstd::arithmetic::power2::{pow2,self};
use std::marker::Copy;

/// Virtual addresses.
pub type Vaddr = u64;

/// Physical addresses.
pub type Paddr = u64;


pub(crate)  mod kspace;
pub(crate)  mod page;
pub(crate)  mod page_table;

/// The level of a page table node or a frame.
pub type PagingLevel = u8;


//Lemmas about consts
verus!{
    // Modifided!!! Used to belong to PagingConstsTrait.
    // A minimal set of constants that determines the paging system.
    //
    // This provides an abstraction over most paging modes in common architectures.


    /// The smallest page size.
    /// This is also the page size at level 1 page tables.
    //#[verifier::external_body]
    pub const BASE_PAGE_SIZE: u64 = 4096;//unimplemented!();

    /// The number of levels in the page table.
    /// The numbering of levels goes from deepest node to the root node. For example,
    /// the level 1 to 5 on AMD64 corresponds to Page Tables, Page Directory Tables,
    /// Page Directory Pointer Tables, Page-Map Level-4 Table, and Page-Map Level-5
    /// Table, respectively.
    //#[verifier::external_body]
    pub const NR_LEVELS: PagingLevel = 4;//unimplemented!();

    /// The highest level that a PTE can be directly used to translate a VA.
    /// This affects the the largest page size supported by the page table.
    //#[verifier::external_body]
    pub const HIGHEST_TRANSLATION_LEVEL: PagingLevel = 2;//unimplemented!();

    pub const PAGE_SIZE: u64 = 4096;

    /// The size of a PTE.
    pub const PTE_SIZE: u64 = 8;

    /// The address width may be BASE_PAGE_SIZE.ilog2() + NR_LEVELS * IN_FRAME_INDEX_BITS.
    /// If it is shorter than that, the higher bits in the highest level are ignored.
    pub const ADDRESS_WIDTH: u64 = 48;//unimplemented!();


    pub proof fn lemma_base_page_size_is_power_of_2()
    ensures exists|i:nat| #[trigger] pow2(i)==BASE_PAGE_SIZE,
    {
        power2::lemma2_to64();
    }

    pub proof fn lemma_highest_translation_level_is_less_than_nr_levels()
    ensures HIGHEST_TRANSLATION_LEVEL <= NR_LEVELS,
    {}

    /// The number of sub pages in a huge page.
    pub(crate) const fn nr_subpage_per_huge() -> (ret:u64) 
    ensures ret == 512,
    {
        BASE_PAGE_SIZE / PTE_SIZE
    }
}
