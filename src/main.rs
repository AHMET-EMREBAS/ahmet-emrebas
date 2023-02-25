fn main() {
    let mut args = std::env::args().skip(1);

    let first = args.next().expect("First arguments is not provided!");

    let second = args.next().expect("Second argument is required!");

    println!("Key: {key} and Value: {value}", key = first, value = second);
}
