use std::collections::HashMap;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().expect("The key was not found");
    let value = arguments.next().expect("The value is not found");
    let mut database = Database::new().expect("Failed to read the Database");
    database.insert(String::from("KEY"), String::from("VALUE"));
    database.insert(key.clone(), value.clone());
}

struct Database {
    map: HashMap<String, String>,
    flush: bool,
}

#[allow(dead_code)]
impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let mut map = HashMap::new();
        let contents = std::fs::read_to_string("kv.db")?;
        for line in contents.lines() {
            let mut chunks = line.splitn(2, '\t');
            let key = chunks.next().expect("The key value is not found");
            let value = chunks.next().expect("The value is not found");
            map.insert(key.to_owned(), value.to_owned());
        }
        Ok(Database { map, flush: false })
    }
    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }
    fn flush(mut self) -> Result<(), std::io::Error> {
        self.flush = true;
        do_flush(&self)
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        if !self.flush {
            let _ = do_flush(self);
        }
    }
}

fn do_flush(database: &Database) -> std::io::Result<()> {
    let mut contents = String::new();
    for (key, value) in &database.map {
        contents.push_str(key);
        contents.push('\t');
        contents.push_str(value);
        contents.push('\n');
    }
    std::fs::write("kv.db", contents)
}
