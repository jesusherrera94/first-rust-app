fn main() {
    println!("Introduce your age: ");
    let mut age: String = String::new(); // to set a empty string
    std::io::stdin().read_line(&mut age).unwrap();

    let age_number: u8 = age.trim().parse().unwrap();

    if age_number >= 18 {
        println!("you can access to the club");
    } else {
        println!("you can't access to the club");
    }

    println!("You are {} yrs old", age_number)

    
}
