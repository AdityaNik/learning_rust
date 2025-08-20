macro_rules! generate_functions {
    ($($func_name:ident),*) => {
        $(
            fn $func_name() {
                println!("Hello from {}", stringify!($func_name));
            }
        )*
    };
}

// generate_functions!(foo, bar, baz);

fn main() {
    print!("Hello from main");
    foo();  // Prints: Hello from foo
    bar();  // Prints: Hello from bar
    baz();  // Prints: Hello from baz
    generate_functions!(foo, bar, baz);
}

// use serde::{Serialize, Deserialize};

// #[derive(Serialize, Deserialize, Debug)]
// struct User {
//     username: String,
//     password: String 
// }

// fn main() {
//     let user = User {
//         username: String::from("aditya"),
//         password: String::from("2334") 
//     };

//     // let res = serde_json::to_string(&user);

//     // let result = match res {
//     //     Ok(result) => result,
//     //     Err(_err) => "Error while converting to string...".to_string(),
//     // };

//     // println!("As a String: {}", result);


//     // let u: User = serde_json::from_str(&result).unwrap();

//     // println!("As a Struct: {:?}", u);


//     let res =  serde_yaml::to_string(&user);
//     let result = match res {
//         Ok(result) => result,
//         Err(_err) => "Error accured during conversion...".to_string()
//     };

//     println!("String conversion: {}", result);

//     let new_res: Result<User, serde_yaml::Error>= serde_yaml::from_str(&result);

//     match new_res {
//         Ok(result ) => println!("Again back to struct: {:?}", result),
//         Err(_err) => println!("Error while converting back to struct...")
//     };

// }


// trait Shape<T> {
//     fn area(&self) -> T;
// }
// struct Rect<T> {
//     width: T,
//     height: T
// }

// impl<T: std::ops::Mul<Output = T> + Copy> Shape<T> for Rect<T> {
//     fn area(&self) -> T {
//         return self.width * self.height;
//     }
// }

// struct Temp {
//     width: f32,    
//     height: f32,
// }

// impl Temp {
//     fn area(&self) -> f32 {
//         return self.width * self.height;
//     }

//     fn what_is_this_implementation() {
//         print!("This is a implemetation of a sturcture named 'temp'");
//     }
// }

// #[derive(Debug)]
// enum Shape {
//     Square(f32),
//     Circle(f32),
//     Ractangle(f32, f32),
// }

// impl Shape {
//     fn calculate_area(&self) -> f32 {
    //         return match self {
        //             Shape::Square(_side) => _side * _side,
        //             Shape::Circle(_radius) => PI * _radius * _radius,
        //             Shape::Ractangle(_width, _height) =>  _width * _height
        //         }
        //     }
        // }
        
        
        // fn option_enum() -> Option<u32> {
        //     let a: u8 = 1;
        //     if a == 1 {
        //         return Some(5);
        //     }else {
        //         return None;
        //     }
        // }

        

// tom's abious minimal language.
