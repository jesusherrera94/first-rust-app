

fn add_one(number_to_add: i32) -> i32 {
    return number_to_add + 1
}


fn main() {
    let mut names: Vec<String> = Vec::new();
    println!("Introduce nombre: ");

    for _i in 0..3 {
        let mut nombre =  String::new();
        std::io::stdin().read_line(&mut nombre).unwrap();
        names.push(nombre);
    }
    // to print arrays we use the debug trait
    println!("{:?}", names);
    // we access to the single position with the index
    // names[0] = "Hola";

    for name in names {
        println!("name: {} ", name);
    }

    let number = 10;
    let result = add_one(number);
    println!("Result: {}", result);
    
}
