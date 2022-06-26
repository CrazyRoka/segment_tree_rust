use super::SegmentTreeComputation;
use std::ops::Add;

pub struct SumComputation;

impl<T> SegmentTreeComputation<T, T> for SumComputation
where
    T: Add<Output = T> + Clone,
{
    fn combine(left_result: &T, right_result: &T) -> T {
        left_result.clone() + right_result.clone()
    }

    fn update(_: &T, new_value: &T) -> T {
        new_value.clone()
    }

    fn init(value: &T) -> T {
        value.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::SumComputation;
    use crate::computation::SegmentTreeComputation;

    #[test]
    fn test_init() {
        let tests = [0, 1, 12345, 5463455];

        for value in tests {
            let expected = value;
            let actual = SumComputation::init(&value);

            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn test_update() {
        let tests = [(353, 0), (5435, 1), (0, 12345), (645345, 5463455), (1, 1)];

        for (prev, cur) in tests {
            let expected = cur;
            let actual = SumComputation::update(&prev, &cur);

            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn test_combine() {
        let tests = [
            (353, 0, 353),
            (5435, 1, 5436),
            (0, 12345, 12345),
            (645345, 5463455, 6108800),
            (1, 1, 2),
        ];

        for (prev, cur, expected) in tests {
            let actual = SumComputation::combine(&prev, &cur);

            assert_eq!(expected, actual);
        }
    }
}
