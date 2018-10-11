extern crate chrono;

use chrono::{DateTime, FixedOffset};

struct Entry {
    name: String,
    domain: String,
    username: String,
    password: String,
    password_pattern: String, // TODO:TS Reverse Regex
    created_date: DateTime<FixedOffset>
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
