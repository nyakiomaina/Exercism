pub fn reply(message: &str) -> &str {
    let trimmed_msg = message.trim();

    if trimmed_msg.is_empty() {
        "Fine. Be that way!"
    } else {
        let is_question = trimmed_msg.ends_with('?');
        let is_yelling = trimmed_msg.chars().any(char::is_alphabetic) && trimmed_msg.to_uppercase() == trimmed_msg;

        match (is_question, is_yelling) {
            (true, true) => "Calm down, I know what I'm doing!",
            (true, false) => "Sure.",
            (false, true) => "Whoa, chill out!",
            _ => "Whatever."
        }
    }
}

fn main() {
    println!("{}", reply("How are you?"));
    println!("{}", reply("WATCH OUT!"));
    println!("{}", reply("DOES THIS LOOK LIKE I'M YELLING?"));
    println!("{}", reply("What if I yell a question?"));
    println!("{}", reply("WHAT IF I YELL A QUESTION?"));
    println!("{}", reply(""));
    println!("{}", reply("           "));
    println!("{}", reply("Anything else"));
}
