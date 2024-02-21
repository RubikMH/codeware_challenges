fn parse(code: &str) -> Vec<i32> {
    let input = String::from(code);
    let mut output = vec![];

    let mut init_number = 0;

    for item in input.split("") {
        match item {
            "i" => init_number += 1,
            "s" => init_number = init_number * init_number,
            "d" => init_number -= 1,
            "o" => output.push(init_number),
            _ => continue,
        }
    }

    output
}

fn main() {
    let result = parse("iiisdosodddddiso");
    println!("{:?}", result);
}
