pub fn encode(n: u64) -> String {
    if n == 0 {
        "zero".to_string()
    } else {
        encode_recurse(n)
    }
}

fn encode_recurse(n: u64) -> String {
    let s = if n == 0 {
        return String::new();
    } else if n >= 1_000_000_000_000_000_000 {
        encode_recurse(n / 1_000_000_000_000_000_000)
            + " quintillion "
            + &encode_recurse(n % 1_000_000_000_000_000_000)
    } else if n >= 1_000_000_000_000_000 {
        encode_recurse(n / 1_000_000_000_000_000)
            + " quadrillion "
            + &encode_recurse(n % 1_000_000_000_000_000)
    } else if n >= 1_000_000_000_000 {
        encode_recurse(n / 1_000_000_000_000)
            + " trillion "
            + &encode_recurse(n % 1_000_000_000_000)
    } else if n >= 1_000_000_000 {
        encode_recurse(n / 1_000_000_000) + " billion " + &encode_recurse(n % 1_000_000_000)
    } else if n >= 1_000_000 {
        encode_recurse(n / 1_000_000) + " million " + &encode_recurse(n % 1_000_000)
    } else if n >= 1_000 {
        encode_recurse(n / 1_000) + " thousand " + &encode_recurse(n % 1_000)
    } else if n >= 100 {
        encode_recurse(n / 100) + " hundred " + &encode_recurse(n % 100)
    } else if n >= 20 {
        let mut s = two_digit_spell(n / 10);
        if n % 10 != 0 {
            s += &"-";
            s += &one_digit_spell(n % 10);
        }
        s
    } else if n >= 10 {
        ten_to_nineteen_spell(n)
    } else {
        one_digit_spell(n)
    };
    s.trim_end().to_string()
}

fn one_digit_spell(n: u64) -> String {
    match n {
        0 => "",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        _ => unreachable!(),
    }
    .to_string()
}

fn ten_to_nineteen_spell(n: u64) -> String {
    match n {
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        _ => unreachable!(),
    }
    .to_string()
}

fn two_digit_spell(n: u64) -> String {
    match n {
        1 => "ten",
        2 => "twenty",
        3 => "thirty",
        4 => "forty",
        5 => "fifty",
        6 => "sixty",
        7 => "seventy",
        8 => "eighty",
        9 => "ninety",
        _ => unreachable!(),
    }
    .to_string()
}
