fn main() {
        // · variables
        
        println!("Please, add your name");
        // &str is faster than String for the computer
        //let mut name: String = "Hello world".to_string();
        let mut nombre: String = String::new(); 
           
        //std: standard libray
        //io: input and output from console
        //stdin().read_line: inputs of the console
        //unwrap: manager errors
        std::io::stdin().read_line(&mut nombre).unwrap();
        nombre = nombre.trim().to_string();
        println!("Your nombre is {}", nombre);
        
        //get age from console
        //convert the age in a number
    
        println!("now add your age");
        let mut edad: String = String::new();
        std::io::stdin().read_line(&mut edad).unwrap();
    
        // trim(): delete the spaces at the beginning and at the end
        // parse(): conver string to number
        let edad: u8 = edad.trim().parse().unwrap();
        println!("Your are {}", edad); 
        
        // challange
        let mut _name: String = String::new();
        let mut _country: String = String::new();
    
        println!("Hi!, please introduce your name and your country");
        std::io::stdin().read_line(&mut _name).unwrap();
        std::io::stdin().read_line(&mut _country).unwrap();

        _name = _name.trim().to_string();
        _country = _country.trim().to_string();
    
        println!("Your name is {} and you are from {}", _name, _country);

}