struct Robot {

}

impl Robot {
    fn greet(&self) {
        println!("Hello, Developer!")
    }
}

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
}