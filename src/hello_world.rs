fn main() {
    // · Hello world

    // let variables are inmutanbles
    let name: &str = "Santiago"; // &st string variable
    // let variables can be changes
    let mut age: u8 = 25; // u numbers without symbol i integer numbers
    age = age + 1;
    println!("Hi, my name is {} and I am {} years", name, age);

    // · challange
    let max_temperature: u8 = 25;
    let min_temperature: i8 = -25;
    println!("In Colombia the min temperature is {} and the max temperature is {}", min_temperature, max_temperature);
}
