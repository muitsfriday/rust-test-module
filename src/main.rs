use module_example::Command;
use module_example::Controller;

fn main() {
    let ctrl = Controller::new("Hello");
    ctrl.describe();
    let r = Controller::compute(Command::Mul(10, 32));

    println!("Hello, world! {:?}", r);
}
