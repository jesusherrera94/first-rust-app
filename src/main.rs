fn main() {
    let number_1 = 123;
    let number_2 = 321;
    let sum = number_1 + number_2;

    loop {
        // print the two numbers on the terminal
        println!("write the answer of : {} + {}", number_1, number_2);

        // get the answer from the user
        let mut answer: String = String::new();
        std::io::stdin().read_line( &mut answer).unwrap();

        let user_answer: i32 =  answer.trim().parse().unwrap();

        if user_answer == sum {
            println!("correct answer");
            break;
        } else {
            println!("wrong answer, try again");
        }
    }
    
}
