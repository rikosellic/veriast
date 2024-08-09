use vstd::prelude::*;

use super::page::
{
    meta::{FrameMeta,PageMeta,PageUsage},
    Page,
};

verus!{
impl PageMeta for FrameMeta {
    //const USAGE: PageUsage = PageUsage::Frame;
    fn usage() -> (ret:PageUsage) 
    ensures ret is Frame,
    {
        PageUsage::Frame
    }


    fn on_drop(_page: &mut Page<Self>) {
        // Nothing should be done so far since dropping the page would
        // have all taken care of.
    }
}
}