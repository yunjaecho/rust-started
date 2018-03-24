fn main() {
    let mut x = 17;
    println!("x = {}", x);
    x = my_func();
    println!("x = {}", x);

    x = calc(100, 100);
    println!("x = {}", x);
}


fn my_func() -> i32 {
    let x= 9;
    // return is not semicolon
    x
}

fn calc(x:i32, y:i32) -> i32{
    let result = x * y;
    // return is not semicolon
    result;
}