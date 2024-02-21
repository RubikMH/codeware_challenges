fn create_phone_number(numbers: &[u8]) -> String {
    let mut output = String::from("");

    for (index, item) in numbers.iter().enumerate() {
        match index {
            0 => {
                output.push('(');
                output.push_str(&item.to_string());
            }
            2 => {
                output.push_str(&item.to_string());
                output.push(')');
            }
            3 => {
                output.push(' ');
                output.push_str(&item.to_string());
            }
            6 => {
                output.push('-');
                output.push_str(&item.to_string());
            }

            _ => output.push_str(&item.to_string()),
        }
    }

    output
}

fn main() {
    let result = create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);

    println!("{}", result);
}
