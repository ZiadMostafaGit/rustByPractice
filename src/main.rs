// fn main(){

// println!("hello world");



// }




// // Modify `4` in assert to make it work
// use std::mem::size_of_val;
// fn main() {
//     let unit: () = ();
//     assert!(size_of_val(&unit) == 0);

//     println!("Success!");
// }

// fn main() {
//     let x = 5u32;

//     let y = {
//         let x_squared = x * x;
//         let x_cube = x_squared * x;

//         // This expression will be assigned to `y`
//         x_cube + x_squared + x
//     };

//     let z = {
//         // The semicolon suppresses this expression and `()` is assigned to `z`
//         2 * x;
//     };

//     println!("x is {:?}", x);
//     println!("y is {:?}", y);
//     println!("z is {:?}", z);
// }

// // Make it work with two ways
// fn main() {
//     let v = {
//         let mut x = 1;
//         x += 2;
//         x
//     };
 
//     assert_eq!(v, 3);
 
//     println!("Success!");
//  }





// // Make it work with two ways
// fn main() {
//     let v = {
//         let mut x = 1;
//         x +2
        
//     };
 
//     assert_eq!(v, 3);
 
//     println!("Success!");
//  }



// fn main() {
//     let v = ( x = 3);
 
//     assert!(v == 3);
 
//     println!("Success!");
//  }



// fn main() {
//     let s = sum(1 , 2);
//     assert_eq!(s, 3);

//     println!("Success!");
// }

// fn sum(x: i32, y: i32) -> i32 {
//     return x + y;
// }



// fn main() {
//     // Don't modify the following two lines!
//     let (x, y) = (1, 2);
//     let s = sum(x, y);

//     assert_eq!(s, 3);

//     println!("Success!");
// }

// fn sum(x:i32, y: i32)->i32{
//     return x + y;
// }



// fn main() {
//     print();
//  }
 
//  // Replace i32 with another type
//  fn print()  {
//     println!("Success!");
//  }

// // Solve it in two ways
// // DON'T let `println!` work
// fn main() {
//     never_return();

//     println!("Failed!");
// }

// fn never_return() -> ! {

//     // Implement this function, don't modify the fn signatures
//     std::process::exit(1);
    
// }



// fn main() {
//     let test = get_option(2); // Call get_option with 1
//     println!("Success! {:?}", test); // Print the result
// }

// fn get_option(tp: u8) -> Option<i32> {
//     match tp {
//         1 => {
//             return Some(2003); // Return Some with value if tp is 1
//         }
//         _ => {
//             // If tp is anything else, we do not return an Option
//             // We will call the diverging function instead
//         }
//     };

//     // Rather than returning None, we use a diverging function instead
//     never_return_fn(); // This will be called if tp is not 1
// }

// // IMPLEMENT this function in THREE ways

// // Implementation 1: Exit the program
// fn never_return_fn() -> ! {
//     std::process::exit(1); // Terminates the program with exit code 1
// }




// fn main() {
//     // FILL in the blank
//     let b = false;

//     let _v = match b {
//         true => 1,
//         // Diverging functions can also be used in match expression to replace a value of any value
//         false => {
//             println!("Success!");
//             panic!("we have no value for `false`, but we can panic");
//         }
//     };

//     println!("Exercise Failed if printing out this line!");
// }




// fn main() {
//     // Use as many approaches as you can to make it work
//     let x = String::from("Hello world");
//     let y = x.clone();
//     println!("{}, {}",x, y);
// }


// // Don't modify code in main!
// fn main() {
//     let s1 = String::from("Hello world");
//     let s2 = take_ownership(s1);

//     println!("{}", s2);
// }

// // Only modify the code below!
// fn take_ownership(s: String)->String{
//     return s;

// }


// // Fix the error without removing any code
// fn main() {
//     let s = String::from("Hello World");

//     print_str(&s);

//     println!("{}", s);
// }

// fn print_str(s: &String)  {
//     println!("{}",s)
// }

// Don't use clone ,use copy instead



// fn main() {
//     let s = give_ownership();
//     println!("{}", s);
// }

// // Only modify the code below!
// fn give_ownership() -> String {
//     let s = String::from("Hello world");
//     // Convert String to Vec
//     let _s = s.as_bytes();
//     s
// }



// fn main() {
//     let x = (1, 2, (), "hello");
//     let y = x;
//     println!("{:?}, {:?}", x, y);
// }



// make the necessary variable mutable



// fn main() {
//     let s = String::from("Hello ");
    
//     let mut s1 = s;

//     s1.push_str("World!");

//     println!("Success!");
// }






// fn main() {
//     let x = Box::new(5);
    
//     let mut y=x.clone();     // update this line, don't change other lines!
    
//     *y = 4;
    
//     assert_eq!(*x, 5);

//     println!("Success!");
// }


// fn main() {
//     #[derive(Debug)]
//     struct Person {
//         name: String,
//         age: Box<u8>,
//     }

//     let person = Person {
//         name: String::from("Alice"),
//         age: Box::new(20),
//     };

//     // `name` is moved out of person, but `age` is referenced
//     let Person { ref name, ref age } = person;

//     println!("The person's age is {}", age);

//     println!("The person's name is {}", name);

//     // Error! borrow of partially moved value: `person` partial move occurs
//     println!("The person struct is {:?}", person);

//     // `person` cannot be used but `person.age` can be used as it is not moved
//     println!("The person's age from person struct is {}", person.age);
// }






// fn main() {
//     let t = (String::from("hello"), String::from("world"));
 
//      // Fill the blanks
//      let (s1, s2) = t.clone();
 
//      println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
//  }



// fn main(){


// let name:String=String::from("ziad");
// let size:usize=name.len();
// println!("name is {},and linght is {}",name,size);









// }






// fn main() {
//     let x = 5;
//     // Fill the blank
//     let p = &x;
 
//     println!("the memory address of x is {:p}", p); // One possible output: 0x16fa3ac84
//  }




// fn main() {
//     let x = 5;
//     let y = &x;

//     // Modify this line only
//     assert_eq!(5,  *y);

//     println!("Success!");
// }



// // Fix error
// fn main() {
//     let mut s = String::from("hello, ");

//     borrow_object(&mut s);


//     println!("Success!");
// }

// fn borrow_object(s: &mut String) {

//     s.push_str("world");
//     println!("{}",*s);






// }





// // Fix error
// fn main() {
//     let mut s = String::from("hello, ");

//     push_str(&mut s);

//     println!("Success!");
// }

// fn push_str(s: &mut String) {
//     s.push_str("world");

//     println!("{}",*s);



// }


// fn main() {
//     let mut s = String::from("hello, ");

//     // Fill the blank to make it work
//     let p = &mut s;
    
//     p.push_str("world");

//     println!("Success!");
// }




// fn main() {
//     let mut s = String::from("hello, ");

//     // Fill the blank to make it work
//     let   ref mut  p =  s;
    
//     p.push_str("world");

//     println!("Success!");
// }






// fn main() {
//     let c = '中';

//     let r1 = &c;
//     // Fill the blank，dont change other code
//     let ref r2 = c;

//     assert_eq!(*r1, *r2);
    
//     // Check the equality of the two address strings
//     assert_eq!(get_addr(r1),get_addr(r2));

//     println!("Success! the tow address are {} and {}",get_addr(r1),get_addr(r2));
// }

// // Get memory address string
// fn get_addr(r: &char) -> String {
//     format!("{:p}", r)
// }





