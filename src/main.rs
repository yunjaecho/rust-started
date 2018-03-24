struct Robot {

}

impl Robot {
    fn greet(&self) {
        println!("Hello, Developer!")
    }
}
/**
* module
* items, functions, structs, impls are defined here
**/
mod robot1 {
    /**
     * private function
    **/
    fn say_hello() {
        println!("Hello")
    }

    /**
     * public function
    **/
    pub fn say_hi() {
        println!("Hi")
    }
}

mod macros;


fn main() {
    println!("Hello, robot!");
    let mut bot = Robot {};
    bot.greet();

    let mut a = 5;
    let mut b = 1;
    while  a == 5 {
        b += 1;
        if b == 5 {
            a += 1;
        }
    }

    let x = 0;
    for x in 0..10 {
        println!("{}", x);
    }

    //robot1::say_hello();
    robot1::say_hi();
    // function alias
    use robot1::say_hi as hi;
    hi();

    welcome!();
}