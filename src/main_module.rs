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


/**
 * module test
 **/
fn main() {
    //robot1::say_hello();
    robot1::say_hi();
    // function alias
    use robot1::say_hi as hi;
    hi();
}