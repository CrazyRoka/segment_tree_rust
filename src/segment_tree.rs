use crate::{
    computation::{MaxComputation, SegmentTreeComputation, SumComputation},
    errors::{SegmentTreeError, SegmentTreeResult},
};
use std::marker::PhantomData;

pub struct SegmentTree<T, K> {
    data: Vec<T>,
    len: usize,
    phantom: PhantomData<K>,
}

pub type SumSegmentTree<T> = SegmentTree<T, SumComputation>;
pub type MaxSegmentTree<T> = SegmentTree<T, MaxComputation>;

impl<T, K> SegmentTree<T, K>
where
    T: Default + Clone + Copy,
    K: SegmentTreeComputation<T>,
{
    pub fn build(arr: &[T]) -> Self {
        if arr.is_empty() {
            Self {
                data: vec![],
                len: 0,
                phantom: PhantomData,
            }
        } else {
            let len = arr.len();
            let mut data = vec![T::default(); len * 4];

            Self::internal_build(arr, &mut data, 1, 0, len - 1);

            Self {
                data,
                len,
                phantom: PhantomData,
            }
        }
    }

    pub fn get(&self, left: usize, right: usize) -> SegmentTreeResult<T> {
        if left > right {
            Err(SegmentTreeError::InvalidRange { left, right })
        } else if right >= self.len {
            Err(SegmentTreeError::OutOfBounds {
                index: right,
                len: self.len,
            })
        } else {
            Ok(self.internal_get(1, 0, self.len - 1, left, right))
        }
    }

    pub fn modify(&mut self, pos: usize, value: T) -> SegmentTreeResult<()> {
        if pos >= self.len {
            Err(SegmentTreeError::OutOfBounds {
                index: pos,
                len: self.len,
            })
        } else {
            self.internal_modify(1, 0, self.len - 1, pos, value);
            Ok(())
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn internal_build(
        input: &[T],
        data: &mut Vec<T>,
        index: usize,
        cur_left: usize,
        cur_right: usize,
    ) {
        data[index] = if cur_left == cur_right {
            K::init(input[cur_left])
        } else {
            let mid = (cur_left + cur_right) / 2;
            Self::internal_build(input, data, index * 2, cur_left, mid);
            Self::internal_build(input, data, index * 2 + 1, mid + 1, cur_right);

            K::combine(data[index * 2], data[index * 2 + 1])
        }
    }

    fn internal_get(
        &self,
        index: usize,
        cur_left: usize,
        cur_right: usize,
        left: usize,
        right: usize,
    ) -> T {
        if left == cur_left && right == cur_right {
            self.data[index]
        } else {
            let mid = (cur_left + cur_right) / 2;

            if mid < left {
                self.internal_get(index * 2 + 1, mid + 1, cur_right, left, right)
            } else if mid + 1 > right {
                self.internal_get(index * 2, cur_left, mid, left, right)
            } else {
                let left_result = self.internal_get(index * 2, cur_left, mid, left, right.min(mid));
                let right_result =
                    self.internal_get(index * 2 + 1, mid + 1, cur_right, left.max(mid + 1), right);

                K::combine(left_result, right_result)
            }
        }
    }

    fn internal_modify(
        &mut self,
        index: usize,
        cur_left: usize,
        cur_right: usize,
        pos: usize,
        value: T,
    ) {
        dbg!(index, cur_left, cur_right, pos);
        self.data[index] = if cur_left == cur_right {
            K::update(self.data[index], value)
        } else {
            let mid = (cur_left + cur_right) / 2;

            if pos <= mid {
                self.internal_modify(index * 2, cur_left, mid, pos, value);
            } else {
                self.internal_modify(index * 2 + 1, mid + 1, cur_right, pos, value);
            }

            K::combine(self.data[index * 2], self.data[index * 2 + 1])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{MaxSegmentTree, SumSegmentTree};
    use crate::{SegmentTreeError, SegmentTreeResult};
    use std::fmt::Debug;

    #[test]
    fn test_build() {
        let arr = [1, 3, 7, 27];
        let tree = SumSegmentTree::build(&arr);

        /*
                  38
                 /  \
                4   34
               / \  / \
               1 3  7 27
        */
        assert!(!tree.is_empty());
        assert_eq!(tree.len(), arr.len());
        assert_eq!(tree.data.len(), 16);
        assert_eq!(tree.data[0], 0);
        assert_eq!(tree.data[1], 38);
        assert_eq!(tree.data[2], 4);
        assert_eq!(tree.data[3], 34);
        assert_eq!(tree.data[4], 1);
        assert_eq!(tree.data[5], 3);
        assert_eq!(tree.data[6], 7);
        assert_eq!(tree.data[7], 27);
        assert_eq!(tree.data[8..], vec![0; 8]);
    }

    #[test]
    fn test_empty_tree() {
        let arr: [usize; 0] = [];
        let mut tree = SumSegmentTree::build(&arr);

        assert_eq!(tree.len(), arr.len());
        assert_eq!(tree.data.len(), 0);
        assert!(tree.is_empty());

        let values = [0, 1, 2, 100];
        for value in values {
            let expected = Err(SegmentTreeError::OutOfBounds {
                index: value,
                len: 0,
            });
            let actual = tree.get(value, value);

            assert_eq!(actual, expected);

            let expected = Err(SegmentTreeError::OutOfBounds { index: 0, len: 0 });
            let actual = tree.modify(0, value);

            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_get() {
        let arr = [1, 3, 7, 27, 73];
        let tree = SumSegmentTree::build(&arr);

        assert!(!tree.is_empty());
        assert_eq!(tree.len(), arr.len());

        for left in 0..arr.len() {
            for right in left..arr.len() {
                let expected: SegmentTreeResult<usize> = Ok(arr[left..=right].iter().sum());
                let actual = tree.get(left, right);

                assert_eq!(actual, expected);
            }
        }
    }

    #[test]
    fn test_get_out_of_bounds() {
        let arr = [1, 3, 7, 27, 73];
        let tree = SumSegmentTree::build(&arr);

        assert!(!tree.is_empty());
        assert_eq!(tree.len(), arr.len());

        let values = [(0, 5), (1, 6), (6, 1234), (3, 435345)];

        for (left, right) in values {
            let expected = Err(SegmentTreeError::OutOfBounds {
                index: right,
                len: 5,
            });
            let actual = tree.get(left, right);

            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_get_invalid_range() {
        let arr = [1, 3, 7, 27, 73];
        let tree = SumSegmentTree::build(&arr);

        assert!(!tree.is_empty());
        assert_eq!(tree.len(), arr.len());

        let values = [(5, 4), (2, 1), (2, 0), (4, 1)];

        for (left, right) in values {
            let expected = Err(SegmentTreeError::InvalidRange { left, right });
            let actual = tree.get(left, right);

            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_modify() {
        let arr = [1, 3, 7, 27];
        let mut tree = SumSegmentTree::build(&arr);

        assert!(!tree.is_empty());
        assert_eq!(tree.modify(3, 10), Ok(()));
        assert_eq!(tree.modify(1, 73), Ok(()));

        /*
                  91
                 /  \
                74   17
               / \  / \
               1 73 7 10
        */
        assert_eq!(tree.data.len(), 16);
        assert_eq!(tree.len(), arr.len());
        assert_eq!(tree.data[0], 0);
        assert_eq!(tree.data[1], 91);
        assert_eq!(tree.data[2], 74);
        assert_eq!(tree.data[3], 17);
        assert_eq!(tree.data[4], 1);
        assert_eq!(tree.data[5], 73);
        assert_eq!(tree.data[6], 7);
        assert_eq!(tree.data[7], 10);
        assert_eq!(tree.data[8..], vec![0; 8]);
    }

    #[test]
    fn test_max_segment_tree() {
        let arr = [1, 3, 7, 27, 73, 7542, 1, -5, -543, 9];
        let tree = MaxSegmentTree::build(&arr);

        assert!(!tree.is_empty());
        assert_eq!(tree.len(), arr.len());

        verify_max_segment_tree(&tree, &arr);
    }

    fn verify_max_segment_tree<T>(tree: &MaxSegmentTree<T>, arr: &[T])
    where
        T: Default + Clone + Copy + Ord + Debug,
    {
        for left in 0..arr.len() {
            for right in left..arr.len() {
                let expected: SegmentTreeResult<T> = Ok(*arr[left..=right].iter().max().unwrap());
                let actual = tree.get(left, right);

                assert_eq!(actual, expected);
            }
        }
    }
}
