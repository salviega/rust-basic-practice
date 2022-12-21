fn main() {
    // · condicionals
    
    let mut age: String = String::new();
    std::io::stdin().read_line(&mut age).unwrap();    
    let age: u8 = age.trim().parse().unwrap();
    
    if age >= 18 && age !=30 {
            println!("You can come-in in the discoteca");
    } else if age == 30 {
            println!("People over 30 are not allowed to enter");
    } else {
            println!("You are under 18 years, cannot enter");
    }
    println!("Your are {}", age); 

    // challange

    println!("Choose the red or blue pill");
    let mut answer: String = String::new();
    std::io::stdin().read_line(&mut answer).unwrap();

    answer = answer.trim().to_string();
    if answer == "red".to_string() {
            println!("Return to the real life");
    } else if answer == "blue".to_string() {
            println!("Return to the Matrix")
    } else {
            println!("Boooom! Matrix is dead");
    }
}
