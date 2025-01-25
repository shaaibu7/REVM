pub mod stack {
    const MAXIMUM_STACK_SIZE: usize = 1024;

    #[derive(Debug)]
    pub struct Stack<T> {
        pub items: Vec<T>,
    }

    impl<T: std::fmt::Display> Stack<T> {
        pub fn new() -> Self {
            Stack { items: Vec::new() }
        }

        pub fn push(&mut self, value: T) -> Result<(), &'static str> {
            if self.items.len() >= MAXIMUM_STACK_SIZE {
                return Err("EVM stack overflow")
            } else {
                self.items.push(value);
                Ok(())
            }
        }

        pub fn pop(&mut self) -> Result<T, &'static str> {
            self.items.pop().ok_or("EVM stack undeflow")
        }

        pub fn display(&self) -> String {
            let mut result = Vec::new();
            
            for (i, item) in self.items.iter().rev().enumerate() {
                let label = if i == 0 {
                    "first"
                } else if i == self.items.len() - 1 {
                    "last"
                } else {
                    ""
                };
                result.push(format!("{} {}", item, label));
            }
            result.join("\n")

        }
    }

}