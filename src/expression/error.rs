pub type EvalResult<T> = Result<T, EvalError>;

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum EvalError {
    #[error("division by zero")]
    DivisionByZero,
    #[error("calculating 0^0 is indeterminite")]
    ZeroToZero,
    #[error("This is an error just for tests. It says: '{0}'")]
    TestError(String),
}
