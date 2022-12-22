use regex::Regex;

fn main() {
        // · scientific calculator

        // One: get Regex
        let re_mul: Regex = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();
        let re_add: Regex = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
        let re_res: Regex = Regex::new(r"(\d+)\s?\-\s?(\d+)").unwrap();
        let re_div: Regex = Regex::new(r"(\d+)\s?/\s?(\d+)").unwrap();


        // Two: get user data
        println!("please, introduces a number");
        let mut expression: String = String::new();
        std::io::stdin().read_line(&mut expression).unwrap();

        loop {
                // Three: writer arithmetic expression
                // what am I bringing
                let caps = re_mul.captures(expression.as_str());

                if caps.is_none() {
                        break;
                }

                let caps = caps.unwrap();
                // {:?} more information of the variable
                println!("{:?}", caps);
                let cap_expression = caps.get(0).unwrap().as_str();
                let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
                let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

                let addition = left_value * right_value;
                
                expression = expression.replace(cap_expression, &addition.to_string());
        }

        loop {
                // Three: writer arithmetic expression
                // what am I bringing
                let caps = re_add.captures(expression.as_str());

                if caps.is_none() {
                        break;
                }

                let caps = caps.unwrap();

                let cap_expression = caps.get(0).unwrap().as_str();
                let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
                let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

                let addition = left_value + right_value;
                
                expression = expression.replace(cap_expression, &addition.to_string());
        }

        loop {
                // Three: writer arithmetic expression
                // what am I bringing
                let caps = re_res.captures(expression.as_str());

                if caps.is_none() {
                        break;
                }

                let caps = caps.unwrap();

                let cap_expression = caps.get(0).unwrap().as_str();
                let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
                let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

                let addition = left_value - right_value;
                
                expression = expression.replace(cap_expression, &addition.to_string());
        }

        loop {
                // Three: writer arithmetic expression
                // what am I bringing
                let caps = re_div.captures(expression.as_str());

                if caps.is_none() {
                        break;
                }

                let caps = caps.unwrap();

                let cap_expression = caps.get(0).unwrap().as_str();
                let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
                let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

                let addition = left_value / right_value;
                
                expression = expression.replace(cap_expression, &addition.to_string());
        }


        // Four: Show the results
        println!("Result is {}", expression);
}
