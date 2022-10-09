pub(crate) mod compressed;
pub(crate) mod hashed;

macro_rules! poll_result_ok {
    ($poll:expr) => {
        match $poll {
            Ok(inner) => inner,
            Err(err) => return Poll::Ready(Err(err)),
        }
    };
}

use poll_result_ok;