use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone, Copy)]
pub enum SegmentTreeError {
    #[error("Index {index} is out of bounds. It should be smaller or equal to {len}")]
    OutOfBounds { index: usize, len: usize },
    #[error("Left index <{left}> should be lower or equal to the right index <{right}>")]
    InvalidRange { left: usize, right: usize },
}

pub type SegmentTreeResult<T> = Result<T, SegmentTreeError>;
