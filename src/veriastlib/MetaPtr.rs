use vstd::prelude::*;

use std::marker::PhantomData;
use crate::mm::page::meta::{MetaSlot,PageMeta};
use crate::proofs::basic::{paddr_range,meta_vaddr_range};
use crate::veriastlib::wellformed::WellFormed;

verus!{
    pub struct MetaPtr<M:PageMeta> {
        pub uptr: u64,
        pub phantom: PhantomData<M>,
    }

    impl<M:PageMeta> WellFormed for MetaPtr<M> {
        open spec fn wf(&self) -> bool {
            meta_vaddr_range(self.uptr)
        }
    }

    impl<M:PageMeta> MetaPtr<M>{
        pub open spec fn id(&self) -> nat {
            self.uptr as nat
        }
        
        #[inline(always)]
        #[verifier(external_body)]
        pub const fn from_usize(u: u64) -> (p: Self)
            requires
                meta_vaddr_range(u),
            ensures
                p.uptr === u,
        {
            MetaPtr { uptr: u, phantom: PhantomData }
        }

        #[inline(always)]
        pub const fn to_usize(&self) -> (u: u64)
            ensures
                u as nat == self.id(),
                u === self.uptr,
        {
            self.uptr
        }
    }

    #[verifier(external_body)]
    #[verifier::reject_recursive_types_in_ground_variants(M)]
    pub tracked struct MetaPointsTo<M:PageMeta> {
        pub phantom: PhantomData<M>,
        no_copy: NoCopy,
    }

    impl<M:PageMeta> MetaPointsTo<M> {
        pub open spec fn view(&self) -> MetaPointsToData<M>;
    }

    pub ghost struct MetaPointsToData<M:PageMeta>{
        pub ptr: nat,
        pub value: Option<MetaSlot>,
        pub phantom: PhantomData<M>,
    }

    impl<M:PageMeta> MetaPointsToData<M> {
        pub open spec fn id(&self) -> nat {
            self.ptr
        }
    
        pub open spec fn pptr(&self) -> nat {
            self.ptr
        }
    
        pub open spec fn value(&self) -> Option<MetaSlot>
        {
            self.value
        }
    }

    /*impl<M:PageMeta> WellFormed for MetaPointsToData<M> {
        open spec fn wf(&self) -> bool {
            self.ptr.wf();
        }
    } */
}