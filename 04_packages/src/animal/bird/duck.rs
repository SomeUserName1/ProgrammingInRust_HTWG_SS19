pub enum DuckNum {
    Quacker,
    Loud,
    Quiet,
}

pub struct Duck {
    pub quack: String,
    amount: u32,
}

impl Duck {
    pub fn default() -> Self {
        Duck { quack: String::from("Quack"), amount: 3 }
    }

    pub fn new(quack: String, amount: u32) -> Duck {
        Duck { quack, amount }
    }

    pub fn quack(&self) -> String {
        let mut quacks = String::from("");
        for _i in 0..self.amount {
            quacks += &self.quack;
        }
        quacks
    }
}
