fn main() {
    println!("Introduce your name: ");
    let mut name: String = String::new(); // to set a empty string


    // to capture the input from the user
    std::io::stdin().read_line(&mut name).unwrap();
    // obtatain the age from the user and convert it to number
    println!("Introduce your age: ");
    let mut age: String = String::new();
    std::io::stdin().read_line(&mut age).unwrap();
    let age_number: u8 = age.trim().parse().unwrap();
    println!("Welcome! {}", name.trim().to_string());
    print!("Your age is: {}", age_number);

    // the {} are positional arguments
    // println!("Hello my name is {} and my age is: {}", name, age);
}
