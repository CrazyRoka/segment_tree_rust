pub use max::MaxComputation;
pub use max_slice_sum::{MaxSliceSum, MaxSliceSumComputation};
pub use sum::SumComputation;

mod max;
mod max_slice_sum;
mod sum;

pub trait SegmentTreeComputation {
    type Input;

    type Output;

    fn combine(left_result: &Self::Output, right_result: &Self::Output) -> Self::Output;

    fn update(prev_value: &Self::Output, new_value: &Self::Input) -> Self::Output;

    fn init(value: &Self::Input) -> Self::Output;
}
