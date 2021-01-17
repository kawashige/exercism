pub fn reply(message: &str) -> &str {
    let mut message_type = 0;
    if message.trim().ends_with("?") {
        message_type |= 1;
    }
    let mut chars = message.chars();
    if chars.any(|c| c.is_alphabetic())
        && chars.all(|c| !c.is_alphabetic() || c.is_ascii_uppercase())
    {
        message_type |= 1 << 1;
    }
    if message.trim().is_empty() {
        message_type |= 1 << 2;
    }

    match message_type {
        0b001 => "Sure.",
        0b010 => "Whoa, chill out!",
        0b011 => "Calm down, I know what I'm doing!",
        0b100 => "Fine. Be that way!",
        _ => "Whatever.",
    }
}
