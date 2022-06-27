use std::{marker::PhantomData, ops::Add};

use super::SegmentTreeComputation;

pub struct MaxSliceSumComputation<T> {
    phantom: PhantomData<T>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct MaxSliceSum<T> {
    total_sum: T,
    best_sum: T,
    best_prefix: T,
    best_suffix: T,
}

impl<T> MaxSliceSum<T>
where
    T: Ord + Add<Output = T> + Default + Clone,
{
    pub fn new(value: &T) -> Self {
        Self {
            total_sum: value.clone(),
            best_sum: value.clone(),
            best_prefix: value.clone(),
            best_suffix: value.clone(),
        }
    }

    pub fn answer(&self) -> &T {
        &self.best_sum
    }

    pub fn from_slice(slice: &[T]) -> Self {
        let total_sum = slice
            .iter()
            .cloned()
            .reduce(|acc, cur| acc + cur)
            .expect("Slice is not empty");

        let best_sum = slice
            .iter()
            .cloned()
            .map(|cur| (cur.clone(), cur))
            .reduce(|(sum, best), (_, cur)| {
                let sum = T::default().max(sum) + cur.clone();
                let best = best.max(sum.clone());
                (sum, best)
            })
            .expect("Slice is not empty")
            .1;

        let best_prefix = slice
            .iter()
            .cloned()
            .map(|cur| (cur.clone(), cur))
            .reduce(|(sum, best), (_, cur)| {
                let sum = sum + cur.clone();
                let best = best.max(sum.clone());
                (sum, best)
            })
            .expect("Slice is not empty")
            .1;

        let best_suffix = slice
            .iter()
            .rev()
            .cloned()
            .map(|cur| (cur.clone(), cur))
            .reduce(|(sum, best), (_, cur)| {
                let sum = sum + cur.clone();
                let best = best.max(sum.clone());
                (sum, best)
            })
            .expect("Slice is not empty")
            .1;

        Self {
            total_sum,
            best_sum,
            best_suffix,
            best_prefix,
        }
    }
}

impl<T> SegmentTreeComputation for MaxSliceSumComputation<T>
where
    T: Ord + Add<Output = T> + Default + Clone,
{
    type Input = T;

    type Output = MaxSliceSum<T>;

    fn combine(left_result: &Self::Output, right_result: &Self::Output) -> Self::Output {
        let total_sum = left_result.total_sum.clone() + right_result.total_sum.clone();
        let best_prefix = left_result
            .best_prefix
            .clone()
            .max(left_result.total_sum.clone() + right_result.best_prefix.clone());
        let best_suffix = right_result
            .best_suffix
            .clone()
            .max(right_result.total_sum.clone() + left_result.best_suffix.clone());
        let best_sum = best_prefix
            .clone()
            .max(best_suffix.clone())
            .max(total_sum.clone())
            .max(left_result.best_sum.clone())
            .max(right_result.best_sum.clone());

        Self::Output {
            total_sum,
            best_sum,
            best_prefix,
            best_suffix,
        }
    }

    fn update(_: &Self::Output, new_value: &Self::Input) -> Self::Output {
        Self::init(new_value)
    }

    fn init(value: &Self::Input) -> Self::Output {
        Self::Output::new(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::computation::{max_slice_sum::MaxSliceSumComputation, SegmentTreeComputation};

    use super::MaxSliceSum;

    #[test]
    fn test_new_max_slice_sum() {
        let tests = [5, 0, 1, -4, -1];

        for value in tests {
            let expected = MaxSliceSum {
                total_sum: value,
                best_prefix: value,
                best_suffix: value,
                best_sum: value,
            };
            let actual = MaxSliceSum::new(&value);

            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn test_from_slice() {
        let arr = [-2, 1, 2, 1, -10, 7, 2, -11, 4];
        let expected = MaxSliceSum {
            total_sum: arr.iter().sum(),
            best_prefix: 2,
            best_suffix: 4,
            best_sum: 9,
        };
        let actual = MaxSliceSum::from_slice(&arr);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_init() {
        let tests = [0, 1, 12345, 5463455];

        for value in tests {
            let expected = MaxSliceSum::new(&value);
            let actual = MaxSliceSumComputation::init(&value);

            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn test_update() {
        let tests = [(353, 0), (5435, 1), (0, 12345), (645345, 5463455), (1, 1)];

        for (prev, cur) in tests {
            let prev = MaxSliceSum::new(&prev);
            let expected = MaxSliceSum::new(&cur);
            let actual = MaxSliceSumComputation::update(&prev, &cur);

            assert_eq!(expected, actual);
        }
    }

    // TODO
    // #[test]
    // fn test_combine() {
    //     let tests = [
    //         (353, 0, 353),
    //         (5435, 1, 5435),
    //         (0, 12345, 12345),
    //         (645345, 5463455, 5463455),
    //         (1, 1, 1),
    //     ];

    //     for (prev, cur, expected) in tests {
    //         let actual = MaxComputation::combine(&prev, &cur);

    //         assert_eq!(expected, actual);
    //     }
    // }
}
