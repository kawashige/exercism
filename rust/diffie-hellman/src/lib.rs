use rand::{thread_rng, Rng};

pub fn private_key(p: u64) -> u64 {
    let mut rng = thread_rng();
    rng.gen_range(1..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    pow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    pow(b_pub, a, p)
}

fn pow(a: u64, b: u64, m: u64) -> u64 {
    match b {
        0 => 1,
        _ if b % 2 == 0 => {
            let p = pow(a, b / 2, m);
            p * p % m
        }
        _ => a * pow(a, b - 1, m) % m,
    }
}
