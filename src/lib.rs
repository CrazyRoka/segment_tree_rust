pub use crate::{
    computation::SegmentTreeComputation,
    errors::{SegmentTreeError, SegmentTreeResult},
    segment_tree::{MaxSegmentTree, MaxSliceSumSegmentTree, SegmentTree, SumSegmentTree},
};

mod computation;
mod errors;
mod segment_tree;
