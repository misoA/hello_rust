pub fn run() {
    let mut hello = String::from("Hello ");

    // print
    println!("{}", hello);

    // get length
    println!("Length: {}", hello.len());

    // push char
    hello.push('W');
    println!("{}", hello);

    // push str
    hello.push_str("orld!");
    println!("{}", hello);

    println!("Capacity: {}", hello.capacity());
    println!("Is Empty: {}", hello.is_empty());
    println!("Contains 'World' : {}", hello.contains("World"));
    println!("Replace 'World' : {}", hello.replace("World", "There"));

    // Loop
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
    println!("{}", s);
}
