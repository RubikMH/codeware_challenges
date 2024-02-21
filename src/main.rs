fn spin_words(words: &str) -> String {
    let mut output = String::from("");

    let mut temp_vec = vec![];
    for item in words.split(" ") {
        if item.len() >= 5 {
            let rev_word = item.chars().rev().collect::<String>();
            temp_vec.push(rev_word)
        } else {
            temp_vec.push(item.to_owned());
        }
    }

    output.push_str(&temp_vec.join(" "));
    output
}
fn main() {
    let result = spin_words("You are almost to the last test");

    println!("{}", result);
}
