use conrod::widget::id::{Id, Generator};
use std;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IdPool(Vec<Id>);

impl IdPool {

    /// Construct a caching generator
    pub fn new() -> Self {
        IdPool(Vec::new())
    }

    /// Resizes the `IdPool`'s inner `Vec` to the given target length
    pub fn resize(&mut self, target_len: usize, id_generator: &mut Generator) {
        if self.len() < target_len {
            self.0.reserve(target_len);
            while self.len() < target_len {
                self.0.push(id_generator.next());
            }
        }
        while self.len() > target_len {
            self.0.pop();
        }
    }

    pub fn repopulate(&mut self, id_generator: &mut Generator) {
        self.resize(20, id_generator);
    }

    /// Gets an Id from the IdPool. Call repopulate first to guarantee Some(Id)
    pub fn get(&mut self) -> Option<Id> {
        self.0.pop()
    }
}

impl std::ops::Deref for IdPool {
    type Target = [Id];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}