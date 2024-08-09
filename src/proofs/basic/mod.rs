use vstd::prelude::*;
use vstd::arithmetic::power2::{pow2,self};
use crate::mm::reexport::*;

// Specification for the range of addresses.
verus!{
    pub open spec fn paddr_range(p: u64) -> bool {
        p <= 0xffff_ffff_ffff
    }

    pub open spec fn meta_vaddr_range(v: u64) -> bool {
        FRAME_METADATA_BASE_VADDR <= v < FRAME_METADATA_CAP_VADDR
    }
}

// Basic properties of the page table constants.
verus!{ 
    pub proof fn lemma_base_page_size_is_power_of_2()
    ensures exists|i:nat| #[trigger] pow2(i)==BASE_PAGE_SIZE,
    {
        power2::lemma2_to64();
    }

    pub proof fn lemma_highest_translation_level_is_less_than_nr_levels()
    ensures HIGHEST_TRANSLATION_LEVEL <= NR_LEVELS,
    {}

    #[verifier::external_body]
    pub proof fn axiom_size_of_metaslot()
    ensures core::mem::size_of::<MetaSlot>() == 16,
    {}
}