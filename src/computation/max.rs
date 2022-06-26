use std::cmp::Ord;

use super::SegmentTreeComputation;

pub struct MaxComputation;

impl<T> SegmentTreeComputation<T> for MaxComputation
where
    T: Ord,
{
    fn combine(left_result: T, right_result: T) -> T {
        left_result.max(right_result)
    }

    fn update(_: T, new_value: T) -> T {
        new_value
    }

    fn init(value: T) -> T {
        value
    }
}

#[cfg(test)]
mod tests {
    use crate::computation::{max::MaxComputation, SegmentTreeComputation};

    #[test]
    fn test_init() {
        let tests = [0, 1, 12345, 5463455];

        for value in tests {
            let expected = value;
            let actual = MaxComputation::init(value);

            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn test_update() {
        let tests = [(353, 0), (5435, 1), (0, 12345), (645345, 5463455), (1, 1)];

        for (prev, cur) in tests {
            let expected = cur;
            let actual = MaxComputation::update(prev, cur);

            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn test_combine() {
        let tests = [
            (353, 0, 353),
            (5435, 1, 5435),
            (0, 12345, 12345),
            (645345, 5463455, 5463455),
            (1, 1, 1),
        ];

        for (prev, cur, expected) in tests {
            let actual = MaxComputation::combine(prev, cur);

            assert_eq!(expected, actual);
        }
    }
}
