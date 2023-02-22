use task::TaskDefinition;

pub struct Printer {
    counter_: u32,
}

impl Printer {
    pub fn new() -> Printer {
        Printer { counter_: 0 }
    }
}

impl TaskDefinition for Printer {
    fn init(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn run(&mut self) -> Result<(), String> {
        self.counter_ = self.counter_ + 1;
        println!("Printer counter = {}", self.counter_);
        Ok(())
    }
    fn terminate(&mut self) {}
}
