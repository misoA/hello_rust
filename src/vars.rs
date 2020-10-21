pub fn run() {
    let name = "Miso";
    let mut age = 20;
    println!("My name is {} and I am {}.", name, age);

    age = 21;
    println!("My name is {} and I am {}.", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Miso", 20);
    println!("{} is {}", my_name, my_age);
}
