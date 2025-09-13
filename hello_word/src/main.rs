// STRUCTS
// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// enum TrafficLight {
//     Red,
//     Yellow,
//     Green,
// }



fn main() {
    // let x: i32 = 5;
    // println!("x1 = {}, address = {:p}", x, &x);

    // let x: i32 = x + 1;
    // println!("x2 = {}, address = {:p}", x, &x);

    // let x: i32 = 10;
    // println!("x3 = {}, address = {:p}", x, &x);

    // let x = x * 2;
    // println!("x4 = {}, address = {:p}", x, &x);

    // let x = "hello";
    // println!("x5 = {}, address = {:p}", x, &x);


    // const MAX_POINTS: u32 = 100_000;
    // println!("MAX_POINTS = {}", MAX_POINTS); 

    // tutorial 3

    //Data types
    //INTEGER
    // length    signed    unsigned
    // 8 bits    i8        u8
    // 16 bits   i16       u16
    // 32 bits   i32       u32
    // 64 bits   i64       u64
    // 128 bits  i128      u128
    // arch      isize     usize

    // let small_number: u8 = 255;
    // println!("small_number = {}, address = {:p}", small_number, &small_number);

    // let big_number: u128 = 128;
    // println!("big_number = {}, address = {:p}", big_number, &big_number);

    // let decimal = 98_222;
    // let hex = 0xff;
    // let octal = 0o77;
    // let binary = 0b1111_0000;
    // let byte = b'A';

    // println!("decimal = {}", decimal);
    // println!("hex = {}", hex);
    // println!("octal = {}", octal);
    // println!("binary = {}", binary);
    // println!("byte = {}", byte);    

    // let t = true;
    // let f: bool = false; // with explicit type annotation
    // println!("t = {}, f = {}", t, f);

    // if t{
    //     println!("t is true");
    // }else{
    //     println!("t is false");
    // }

    // let not_t = !t;
    // println!("not_t = {}", not_t);

    // let b : bool = 1 > 2;
    // println!("b = {}", b);  

    // let c = 'z';
    // let z = 'Z';
    // let heart_eyed_cat = '😻';
    // println!("c = {}, z = {}, heart_eyed_cat = {}", c, z, heart_eyed_cat);

//     let tup: (i32, f64, char) = (500, 6.4, 'x');
  

//   //access by index
//     println!("tup.0 = {}", tup.0);
//     println!("tup.1 = {}", tup.1);
//      println!("tup.2 = {}", tup.2);



//arrays
// let arr =  [1, 2, 3, 4, 5];
// println!("arr[0] = {}", arr[0]);
// println!("arr[1] = {}", arr[1]);
// println!("arr[2] = {}", arr[2]);

// for element in arr.iter() {
//     println!("element = {}", element);
// }

//Struct example
// let user = User {
//     email: String::from("someone@me.com"),
//     username: String::from("username"),
//     active: true,
//     sign_in_count: 1,
// };
// println!("user email = {}", user.email);
// println!("user username = {}", user.username);
// println!("user active = {}", user.active);
// println!("user sign_in_count = {}", user.sign_in_count);

//ENUMS

// let light = TrafficLight::Red;

// match light {
//     TrafficLight::Red => println!("Stop!"),
//     TrafficLight::Yellow => println!("Slow down!"),
//     TrafficLight::Green => println!("Go!"),
// }



another_function(); 



} 

fn another_function() {
    print!("Another function");
}
