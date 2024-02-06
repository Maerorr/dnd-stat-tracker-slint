pub struct Item {
    pub name: String,
    pub description: String,
    pub amount: i32,
    pub row: i32,
    pub col: i32,
}

impl Item {
    pub fn new(name: &str, description: &str, amount: i32, row: i32, col: i32) -> Item {
        Item {
            name: name.to_string(),
            description: description.to_string(),
            amount,
            row,
            col,
        }
    }
}