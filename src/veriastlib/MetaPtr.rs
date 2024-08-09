use vstd::prelude::*;

use std::marker::PhantomData;
use crate::mm::reexport::*;
use crate::proofs::basic::{paddr_range,meta_vaddr_range};
use crate::veriastlib::wellformed::WellFormed;

// A safe verus api developed to replace the *const MetaSlot in the Page definition for verification.
// We choose to define MetaPtr instead of using PPtr for customized control.


verus!{
    pub struct MetaPtr {
        pub uptr: u64,
        pub ghost current_usage: Option<PageUsage>,
    }

    impl WellFormed for MetaPtr {
        open spec fn wf(&self) -> bool {
            meta_vaddr_range(self.uptr)
        }
    }

    impl core::marker::Copy for MetaPtr {}

    impl core::clone::Clone for MetaPtr {
        #[verifier(external_body)]
        fn clone(&self) -> (ret: Self)
            ensures
                *self === ret,
        {
            MetaPtr { uptr: self.uptr, current_usage: self.current_usage }
        }
    }

    impl MetaPtr{
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
            MetaPtr { uptr: u, current_usage: None }
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
        phantom: PhantomData<M>,
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
    
}