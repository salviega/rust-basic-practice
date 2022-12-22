fn main() {
    // · loop cicle

    let number_one: i32 = 123;
    let number_two: i32 = 321;

    let sum: i32 = number_one + number_two;
    println!("{}", sum);

    loop {
            if sum < 433 {
                    println!("This is not the number");
            } else if sum == 444 {
                    println!("Yes this is the number");
                    break;
            } else {
                    println!("This number is greater than result");
            }
    }
}
