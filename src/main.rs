fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let mut input = a;
    input.retain(|x| !b.contains(x));
    input
}

fn main() {
    let result = array_diff(vec![1, 2, 2], vec![1]);
    println!("{:?}", result);
}
