pub use max::MaxComputation;
pub use sum::SumComputation;

mod max;
mod sum;

pub trait SegmentTreeComputation<T> {
    fn combine(left_result: T, right_result: T) -> T;

    fn update(prev_value: T, new_value: T) -> T;

    fn init(value: T) -> T;
}
