//! Driving a handler's future to completion.
//!
//! Every host call is synchronous, so a handler future never actually waits on
//! anything: it completes on its first poll. That makes the executor a single
//! poll with a no-op waker, rather than a runtime dependency compiled into
//! every mapping.

use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, Waker};

/// Run `future` to completion on the current thread.
///
/// # Panics
///
/// If the future returns `Pending`. Only a future that awaits something other
/// than an SDK host call can do that, and inside a mapping nothing could ever
/// wake it.
pub fn block_on<F: Future>(future: F) -> F::Output {
    let mut future = pin!(future);
    let mut cx = Context::from_waker(Waker::noop());
    match future.as_mut().poll(&mut cx) {
        Poll::Ready(output) => output,
        Poll::Pending => panic!(
            "a handler awaited a future that is not an SDK host call; \
             mappings cannot wait on timers, sockets or other runtimes"
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_ready_future_completes_on_the_first_poll() {
        assert_eq!(block_on(async { 1 + 1 }), 2);
    }

    #[test]
    #[should_panic(expected = "not an SDK host call")]
    fn a_pending_future_panics_with_a_reason() {
        block_on(std::future::pending::<()>());
    }
}
