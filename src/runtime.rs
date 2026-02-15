use std::sync::LazyLock;
use tokio::runtime::Builder;

// static mut RUNTIME: LazyLock<Runtime> = LazyLock::new(|| Runtime::new());

pub struct Runtime {
    pub runtime: tokio::runtime::Runtime
}

impl Runtime {
    pub fn new() -> Self {
        let runtime = Builder::new_multi_thread()
            .worker_threads(4)
            .thread_name("sui-client-runtime")
            .thread_stack_size(3 * 1024 * 1024)
            .enable_time()
            .enable_io()
            .build()
            .unwrap();

        Runtime {
            runtime
        }
    }

    /* pub fn shared() -> &'static mut Runtime {
        #[allow(static_mut_refs)]
        unsafe { &mut RUNTIME }
    } */
}
