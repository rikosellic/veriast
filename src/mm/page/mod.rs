use vstd::prelude::*;
use vstd::ptr::{PPtr,PointsTo};

pub mod meta;
use meta::{FrameMeta, MetaSlot, PageMeta, PageUsage};
use core::marker::PhantomData;
use super::{PagingLevel, PAGE_SIZE};


verus!{
    pub struct Page<M: PageMeta> {
        pub(super) ptr:  PPtr<MetaSlot>, //*const MetaSlot,
        pub(super) _marker: PhantomData<M>,
    }

    impl<M: PageMeta> Page<M> {
        /// Get a `Page` handle with a specific usage from a raw, unused page.
        ///
        /// The caller should provide the initial metadata of the page.
        
        /// Get the paging level of this page.
        ///
        /// This is the level of the page table entry that maps the frame,
        /// which determines the size of the frame.
        ///
        /// Currently, the level is always 1, which means the frame is a regular
        /// page frame.
        pub const fn level(&self) -> (ret:PagingLevel)
        ensures ret == 1,
        {
            1
        }

        /// Size of this page in bytes.
        pub const fn size(&self) -> (ret:u64) 
        ensures ret == PAGE_SIZE,
        {
            PAGE_SIZE
        }
        
        /* 
        /// Get the metadata of this page.
        pub fn meta<'a>(&self, perm:Tracked<&'a PointsTo<M>>) -> &'a M 
        requires 
            self.ptr.id() == perm.pptr,

        {
            //unsafe { &*(self.ptr as *const M) }
            let casted_ptr: PPtr<M> = PPtr::from_usize(self.ptr.to_usize());
            casted_ptr.borrow(perm)
            
        }
            */
      
    }
}
