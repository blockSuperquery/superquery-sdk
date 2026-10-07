//! What a `#[handler]` export does between receiving bytes and returning a
//! status. Kept here, not in the macro, so the generated code stays a few
//! lines a developer can read.

use std::future::Future;

use crate::error::{Error, Result};
use crate::executor::block_on;
use crate::host::abi::{HandlerStatus, report_handler_error};
use crate::input::HandlerInput;

/// Decode `payload`, run `handler` on it, and report any error to the host.
pub fn dispatch<I, F>(payload: &[u8], handler: impl FnOnce(I) -> F) -> HandlerStatus
where
    I: HandlerInput,
    F: Future<Output = Result<()>>,
{
    #[cfg(target_arch = "wasm32")]
    install_panic_hook();

    let input = match I::from_payload(payload) {
        Ok(input) => input,
        Err(err) => {
            report_handler_error(&err.to_string());
            return HandlerStatus::BadPayload;
        }
    };

    match block_on(handler(input)) {
        Ok(()) => HandlerStatus::Ok,
        Err(err @ Error::Payload(_)) => {
            report_handler_error(&err.to_string());
            HandlerStatus::BadPayload
        }
        Err(err) => {
            report_handler_error(&err.to_string());
            HandlerStatus::Failed
        }
    }
}

/// The body of every `sq_handle_*` export: take ownership of the host's
/// buffer and dispatch.
#[cfg(target_arch = "wasm32")]
pub fn run_export(ptr: u32, len: u32, dispatch: fn(&[u8]) -> HandlerStatus) -> u32 {
    // SAFETY: the ABI requires the host to pass a buffer it filled after
    // allocating it with `sq_alloc(len)`, and hands ownership to the guest.
    #[allow(unsafe_code)]
    let payload = unsafe { crate::host::wasm::take_buffer(ptr, len) };
    dispatch(&payload) as u32
}

/// Mappings build with `panic = "abort"`, so a panic traps and the host only
/// sees "unreachable". Logging first gives the operator the actual message.
#[cfg(target_arch = "wasm32")]
fn install_panic_hook() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        std::panic::set_hook(Box::new(|info| {
            crate::host::log::error(&format!("mapping panicked: {info}"));
        }));
    });
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use crate::testing::TestStore;

    struct Number(u32);

    impl HandlerInput for Number {
        fn from_payload(payload: &[u8]) -> Result<Self> {
            std::str::from_utf8(payload)
                .ok()
                .and_then(|s| s.parse().ok())
                .map(Number)
                .ok_or_else(|| Error::Payload("not a number".into()))
        }
    }

    async fn even_only(n: Number) -> Result<()> {
        if n.0 % 2 == 0 {
            Ok(())
        } else {
            Err(Error::mapping(format!("{} is odd", n.0)))
        }
    }

    #[test]
    fn success_reports_nothing() {
        let store = TestStore::new();
        let _host = store.install();
        assert_eq!(dispatch(b"4", even_only), HandlerStatus::Ok);
        assert!(store.handler_errors().is_empty());
    }

    #[test]
    fn a_handler_error_is_status_one_with_its_message() {
        let store = TestStore::new();
        let _host = store.install();
        assert_eq!(dispatch(b"3", even_only), HandlerStatus::Failed);
        assert_eq!(store.handler_errors(), ["3 is odd"]);
    }

    #[test]
    fn an_undecodable_payload_is_status_two_and_never_runs_the_handler() {
        let store = TestStore::new();
        let _host = store.install();
        let status = dispatch(b"x", |_: Number| async { panic!("must not run") });
        assert_eq!(status, HandlerStatus::BadPayload);
        assert_eq!(
            store.handler_errors(),
            ["could not decode handler payload: not a number"]
        );
    }
}
