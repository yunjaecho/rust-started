
pub struct SampleImpl {
    myName: String
}

impl SampleImpl {
    // static method
    pub fn new(name: String) -> SampleImpl {
        SampleImpl{myName: name}
    }

    // none static method
    pub fn hello_world(&self) {
        println!("My Name Is: {:?}", self.myName);
    }
}