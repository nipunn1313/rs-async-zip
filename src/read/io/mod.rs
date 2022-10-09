pub(crate) mod compressed;
pub(crate) mod hashed;

/// A macro that returns the inner value of an Ok or early-returns in the case of an Err.
/// 
/// This is almost identical to the ? operator but handles the situation when a Result is used in combination with
/// Poll (eg. tokio's IO traits such as AsyncRead).
macro_rules! poll_result_ok {
    ($poll:expr) => {
        match $poll {
            Ok(inner) => inner,
            Err(err) => return Poll::Ready(Err(err)),
        }
    };
}

use poll_result_ok;