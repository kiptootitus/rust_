pub mod helpersmod {
    pub fn get_full_name(first: &str, middle: &str, last: &str) -> String {
        format!("{} {} {}", first.trim(), middle.trim(), last.trim())
    }
}

pub mod privatefns {
    pub fn get_age(age: u32) -> u32 {
        return age + 5;
    }
}
