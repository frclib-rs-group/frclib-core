pub use tracing_unwrap::ResultExt as _;
trait ResultExt<T, E>: tracing_unwrap::ResultExt<T, E> + Sized {
    ///Consumes the result and logs the error if it is `Err`.
    /// 
    /// DOES NOT PANIC ON ERROR.
    /// This should be used for non-fatal errors.
    fn log(self) 
    where
        E: std::fmt::Debug;

    ///Consumes the result and logs the error if it is `Err`.
    /// 
    /// DOES NOT PANIC ON ERROR.
    /// This should be used for non-fatal errors.
    fn log_msg(self, msg: &str) 
    where
        E: std::fmt::Debug;
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
    fn log(self) 
    where
        E: std::fmt::Debug
    {
        if let Err(e) = self {
            let location = std::panic::Location::caller();
            tracing::error!(
                unwrap.filepath = location.file(),
                unwrap.lineno = location.line(),
                unwrap.columnno = location.column(),
                "{:?}",
                e
            );
        }
    }

    fn log_msg(self, msg: &str) 
    where
        E: std::fmt::Debug
    {
        if let Err(e) = self {
            let location = std::panic::Location::caller();
            tracing::error!(
                unwrap.filepath = location.file(),
                unwrap.lineno = location.line(),
                unwrap.columnno = location.column(),
                "{}: {:?}",
                msg,
                e
            );
        }
    }
}