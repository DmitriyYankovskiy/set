use std::ops::{DerefMut, Deref};

use serde::{Serialize, Deserialize};



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Set<T: Clone> {
    object: T,
    count: usize,
}

impl <T: Clone> Deref for Set<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.object
    }
}


impl<T: Clone> DerefMut for Set<T> {    
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.object
    }
}

impl<T: Clone> Set<T> {
    pub fn new(object: T, count: usize) -> Set<T> {
        Set::<T> {
            object,
            count
        }
    }

    pub fn open(&self) -> Vec<T> {
        let object = self.object.clone();
        let mut v = Vec::<T>::new();

        for _ in 0..self.count {
            v.push(object.clone());
        }

        v
    }
}