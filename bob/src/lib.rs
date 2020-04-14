pub fn reply(message: &str) -> &str {
    let trimmed = message.trim();
    let empty = trimmed.is_empty();
    let last = trimmed.chars().last();
    let question = last == Some('?');
    let shouting_chars = trimmed.chars().filter(|c| c.is_alphabetic());
    let shouting = shouting_chars.clone().all(char::is_uppercase) && shouting_chars.clone().next().is_some();

    if empty {
        "Fine. Be that way!"
    } else if question && shouting {
        "Calm down, I know what I'm doing!"
    } else if question {
        "Sure."
    } else if shouting {
        "Whoa, chill out!"
    } else {
        "Whatever."
    }
}
