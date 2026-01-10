use std::sync::LazyLock;
use std::collections::HashMap;
use sui_keys::keystore::InMemKeystore;

static mut RUNTIME: LazyLock<Runtime> = LazyLock::new(|| Runtime::new());

pub struct Runtime {
    pub keystores: HashMap<String, InMemKeystore>
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            keystores: HashMap::new(),
        }
    }

    pub fn get_keystore(&mut self, id: &str) -> &mut InMemKeystore {
        self.keystores.get_mut(id).unwrap()
    }

    pub fn shared() -> &'static mut Runtime {
        #[allow(static_mut_refs)]
        unsafe { &mut RUNTIME }
    }
}
