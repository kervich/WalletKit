use std::sync::LazyLock;
use sui_keys::keystore::InMemKeystore;

static mut RUNTIME: LazyLock<Runtime> = LazyLock::new(|| Runtime::new());

pub struct Runtime {
    pub keystore: InMemKeystore
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            keystore: InMemKeystore::default(),
        }
    }

    pub fn shared() -> &'static mut Runtime {
        #[allow(static_mut_refs)]
        unsafe { &mut RUNTIME }
    }
}
