// Making a todo app via rust
// Following a code camp: https://www.freecodecamp.org/news/how-to-build-a-to-do-app-with-rust/

use std::collections::HashMap;

struct Todo {
    // use rust built-in HashMap to store key - value pairs
    map: HashMap<String, bool>,
}

impl Todo {
    fn insert(&mut self, key: String) {
        //insert a new item into our map
        // we pass true as value
        self.map.insert(key, true);
    }
}

fn main(){
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item");

    println!("{:?}, {:?}", action, item);
}