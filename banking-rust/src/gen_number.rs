
//welcome to the banking sim account number generator!!!

struct IdGenerator {
    counter: u32,
}

impl IdGenerator {
    fn new() -> Self {
        Self { counter: 0 }
    }

    pub fn next(&mut self) -> String {
        let id = format!("224{:07}", self.counter);
        self.counter += 1;
        id
    }
}
