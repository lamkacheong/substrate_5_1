pub trait Timeable {
    fn time(&self) -> u32;
}

#[derive(Debug)]
enum light{
    green,
    yellow,
    red,
}

impl Timeable for light{
    fn time(&self) -> u32 {
        match(&self){
            light::green => 10,
            light::red => 20,
            light::yellow => 30,
        }
    }
}
fn main() {
    let some_light = light::red;
    println!("{:?}", some_light.time());
}