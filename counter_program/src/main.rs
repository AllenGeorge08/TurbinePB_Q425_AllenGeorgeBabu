struct Counter {
    value: i32,
}

impl Counter {
    fn new() -> Self {
        Counter { value: 0 }
    }

    fn increment(&mut self) -> i32 {
        self.value += 1;
        self.value
    }

    fn decrement(&mut self) -> i32 {
        self.value -= 1;
        self.value
    }

    fn get(&self) -> i32 {
        self.value
    }

    fn set(&mut self, value: i32) {
        self.value = value;
    }
}

fn main() {
    // let mut counter = Counter::new();
    let mut counter = Counter::new();
    let incremented_value = counter.increment();
    println!("{}", counter.get());

    counter.set(31);
    println!("{}", counter.get());
}
