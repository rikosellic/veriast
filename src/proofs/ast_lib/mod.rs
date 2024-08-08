use vstd::prelude::*;

use crate::mm::{Vaddr, Paddr, PAGE_SIZE, ADDRESS_WIDTH};
use crate::mm::kspace::{FRAME_METADATA_BASE_VADDR,FRAME_METADATA_CAP_VADDR};

//Specificationss about ranges
verus!{
    pub open spec fn paddr_range(p:Paddr) -> bool
    {
        p <= 0xffff_ffff_ffff
    }

    pub open spec fn meta_vaddr_range(v:Vaddr) -> bool
    {
        FRAME_METADATA_BASE_VADDR <= v < FRAME_METADATA_CAP_VADDR
    }

}