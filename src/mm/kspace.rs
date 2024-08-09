#![allow(dead_code)]

//! Kernel memory space management.
//!
//! The kernel memory space is currently managed as follows, if the
//! address width is 48 bits (with 47 bits kernel space).
//!
//! TODO: the cap of linear mapping (the start of vm alloc) are raised
//! to workaround for high IO in TDX. We need actual vm alloc API to have
//! a proper fix.
//!
//! ```text
//! +-+ <- the highest used address (0xffff_ffff_ffff_0000)
//! | |         For the kernel code, 1 GiB. Mapped frames are untracked.
//! +-+ <- 0xffff_ffff_8000_0000
//! | |
//! | |         Unused hole.
//! +-+ <- 0xffff_ff00_0000_0000
//! | |         For frame metadata, 1 TiB.
//! | |         Mapped frames are untracked.
//! +-+ <- 0xffff_fe00_0000_0000
//! | |         For vm alloc/io mappings, 1 TiB.
//! | |         Mapped frames are tracked with handles.
//! +-+ <- 0xffff_fd00_0000_0000
//! | |
//! | |
//! | |
//! | |         For linear mappings.
//! | |         Mapped physical addresses are untracked.
//! | |
//! | |
//! | |
//! +-+ <- the base of high canonical address (0xffff_8000_0000_0000)
//! ```
//!
//! If the address width is (according to [`crate::arch::mm::PagingConsts`])
//! 39 bits or 57 bits, the memory space just adjust porportionally.

use vstd::prelude::*;

use core::ops::Range;
use super::{
    Paddr, Vaddr, PAGE_SIZE,
};

use crate::mm::ADDRESS_WIDTH;

verus!
{   
    /// The shortest supported address width is 39 bits. And the literal
    /// values are written for 48 bits address width. Adjust the values
    /// by arithmetic left shift.
    pub const ADDR_WIDTH_SHIFT: i64 = ADDRESS_WIDTH as i64 - 48;
    pub const FRAME_METADATA_CAP_VADDR: Vaddr = 0xffff_ff00_0000_0000 << ADDR_WIDTH_SHIFT;
    pub const FRAME_METADATA_BASE_VADDR: Vaddr = 0xffff_fe00_0000_0000 << ADDR_WIDTH_SHIFT;
    pub /*(in crate::mm)*/ const FRAME_METADATA_RANGE: Range<Vaddr> =
        FRAME_METADATA_BASE_VADDR..FRAME_METADATA_CAP_VADDR;
}