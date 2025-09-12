fn main() {
    let x: i32 = 5;
    println!("x1 = {}, address = {:p}", x, &x);

    let x: i32 = x + 1;
    println!("x2 = {}, address = {:p}", x, &x);

    let x: i32 = 10;
    println!("x3 = {}, address = {:p}", x, &x);

    let x = x * 2;
    println!("x4 = {}, address = {:p}", x, &x);

    let x = "hello";
    println!("x5 = {}, address = {:p}", x, &x);


    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS = {}", MAX_POINTS); 
}
