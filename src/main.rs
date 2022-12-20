fn main() {
    // let variables are inmutanbles
    let name: &str = "Santiago"; // &st string variable
    // let variables can be changes
    let mut age: u8 = 25; // u numbers without symbol i integer numbers
    age = age + 1;
    println!("Hi, my name is {} and I am {} years", name, age);
}
