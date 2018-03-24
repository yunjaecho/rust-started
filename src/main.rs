
mod smaple_module;
use smaple_module::SampleImpl;

pub fn main() {
    let myobject = SampleImpl::new("Matt".to_string());
    myobject.hello_world();
}