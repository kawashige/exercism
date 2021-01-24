#[macro_export]
macro_rules! hashmap {
    ($($($key:literal => $val:expr)+$(,)?)*) => {{
        let mut hm = ::std::collections::HashMap::new();
        $($(
            hm.insert($key, $val);
        )+)*
        hm
    }};
}
