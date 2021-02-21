use once_cell::sync::Lazy;
use rand::{thread_rng, Rng};
use std::{collections::HashSet, sync::Mutex};

static NAMES: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| {
    let m = HashSet::new();
    Mutex::new(m)
});

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        Self {
            name: Self::gen_unused_name(),
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn reset_name(&mut self) {
        self.name = Self::gen_unused_name();
    }

    fn gen_unused_name() -> String {
        let mut names = NAMES.lock().unwrap();
        let mut name = Self::gen_name();
        while names.contains(&name) {
            name = Self::gen_name();
        }
        names.insert(name.clone());
        name
    }

    fn gen_name() -> String {
        let mut rng = thread_rng();
        let mut s = String::new();
        for _ in 0..2 {
            let i = rng.gen_range(0..26) as u8;
            s.push((0x41 + i) as char);
        }
        for _ in 0..3 {
            let i = rng.gen_range(0..10) as u8;
            s.push((0x30 + i) as char);
        }
        s
    }
}
