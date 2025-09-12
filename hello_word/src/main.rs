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

    let small_number: u8 = 255;
    println!("small_number = {}, address = {:p}", small_number, &small_number);

    let big_number: u128 = 128;
    println!("big_number = {}, address = {:p}", big_number, &big_number);

    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';

    println!("decimal = {}", decimal);
    println!("hex = {}", hex);
    println!("octal = {}", octal);
    println!("binary = {}", binary);
    println!("byte = {}", byte);    

    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("t = {}, f = {}", t, f);

    if t{
        println!("t is true");
    }else{
        println!("t is false");
    }

    let not_t = !t;
    println!("not_t = {}", not_t);


} 
