use std::collections::HashMap;
use std::hash::Hash;

pub struct Cell<T, U> {
    memory: HashMap<T,U>
}

pub trait Storage<T, U>{
    fn set(&mut self, key: T, value: U);
    fn get(&self, key: T) -> Option<&U>;
}

impl<T, U> Cell<T, U> {
    pub fn new() -> Self {
        Cell{
            memory: HashMap::new()
        }
    }
}

impl<T: Eq + Hash, U> Storage<T, U> for Cell<T, U> {
    fn set(&mut self, key: T, value: U) {
        self.memory.insert(key,value);
    }

    fn get(&self, key: T) -> Option<&U> {
        self.memory.get(&key)
    }
}

#[cfg(test)]
mod tests {
    // Bring everything from the outer scope into the test module
    use super::*;

    #[test]
    fn test_store_and_retrieve() {
        let mut cell = Cell::new();
        cell.set("username", "alice");

        // Assert that we can retrieve the value we stored
        assert_eq!(cell.get(&"username"), Some(&"alice"));
    }

    #[test]
    fn test_retrieve_nonexistent() {
        let cell: Cell<String, i32> = Cell::new();

        // Asserting that a missing key returns None
        assert_eq!(cell.get(String::from("missing")), None);
    }
}