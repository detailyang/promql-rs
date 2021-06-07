use nom::error::{ContextError, ErrorKind, ParseError};
use thiserror::Error;

#[derive(Debug, PartialEq)]
pub enum Error<I> {
    Parser(ParserError),
    Nom(I, ErrorKind),
}

impl<I> ParseError<I> for Error<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        Self::Nom(input, kind)
    }

    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}

impl<I> ContextError<I> for Error<I> {
    fn add_context(_input: I, _ctx: &'static str, other: Self) -> Self {
        other
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum ParserError {
    #[error("invalid duration: {0}")]
    InvalidDuration(humantime::DurationError),

    #[error("ranges only allowed for vector selectors")]
    RangeOnlyVectorSelectors,

    #[error("invalid matrix selector")]
    InvalidMatrixSelector,

    #[error("invalid offset epxr")]
    InvalidOffsetExpr,
}
