pub fn reply(message: &str) -> &str {
    let reply_to = message.trim();
    let reply_upper = reply_to.to_uppercase();
    let reply_lower = reply_to.to_lowercase();

    if reply_to.is_empty() {
        return "Fine. Be that way!"
    }
    else if reply_to == reply_upper && reply_to != reply_lower {
        if reply_to.ends_with("?") {
            return "Calm down, I know what I'm doing!"
        }
        else {
            return "Whoa, chill out!"
        }
    }
    else if reply_to.ends_with("?") {
        return "Sure."
    }
    else {
        return "Whatever."
    }
}
