mod math;

pub struct Controller {
    name: String,
}

pub enum Command {
    Mul(i32, i32),
    Dot(Vec<f32>, Vec<f32>),
    None,
}

impl Controller {
    pub fn new(name: &str) -> Self {
        Controller {
            name: name.to_owned(),
        }
    }

    pub fn describe(&self) {
        println!("Controller name: {}", self.name);
    }

    pub fn compute(command: Command) -> Result<f32, String> {
        match command {
            Command::Mul(x, y) => Result::Ok(math::mul(x, y) as f32),
            Command::Dot(x, y) => math::dot(x, y),
            _ => Result::Err("Bad command".to_string()),
        }
    }
}
