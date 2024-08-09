use vstd::prelude::*;

use super::PageTableEntryTrait;
use crate::{
    mm::{
        page::{
        self,
        meta::{PageMeta, PageTablePageMeta,PageUsage},
        Page,
    }
}
};

verus!{

impl<E:PageTableEntryTrait> PageMeta for PageTablePageMeta<E>
{
    //const USAGE: PageUsage = PageUsage::PageTable;
    open spec fn usage() -> (ret:PageUsage) 
    {
        PageUsage::PageTable
    }

    fn on_drop(_page: &mut Page<Self>) {
       //TODO!!
    }

}
}