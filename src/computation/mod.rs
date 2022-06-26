pub use max::MaxComputation;
pub use sum::SumComputation;

mod max;
mod sum;

pub trait SegmentTreeComputation<I, O> {
    fn combine(left_result: &O, right_result: &O) -> O;

    fn update(prev_value: &O, new_value: &I) -> O;

    fn init(value: &I) -> O;
}
