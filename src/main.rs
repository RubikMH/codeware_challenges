use std::collections::HashMap;

fn count(input: &str) -> HashMap<char, i32> {
    let mut output: HashMap<char, i32> = Default::default();

    for item in input.chars() {
        if output.contains_key(&item) {
            output.entry(item).and_modify(|x| *x += 1);
        } else {
            output.insert(item, 1);
        }
    }

    output
}
fn main() {
    let result = count("aabb");

    println!("{:?}", result);
}
