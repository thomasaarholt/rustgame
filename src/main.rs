use std::path::Path;
use std::ffi::OsStr;
use std::fs;
use serde_json::Value;


fn main() -> Result<(), std::io::Error> {
    
    color_backtrace::install();
    let path = Path::new("src/locations/");
    let mut myvec = vec![];
    for entry in fs::read_dir(path)? {

        let data: String = fs::read_to_string(entry.unwrap().path().to_str().unwrap()).expect("could not find the file");
        let data3 = &*data;
        let v: Value = serde_json::from_str(data3).expect("Something wrong");
        let loc = _Location {
            name: v["name"].as_str().expect("name conversion error").to_string(), 
            max_hp:v["hp"].as_u64().expect("hp conversion error"), 
            attack:v["attack"].as_u64().expect("attack damage conversion error")
        };
        myvec.push(loc);

    }
    for entry in myvec {
        entry.print()
    }
    Ok(())
}
#[derive(Debug)]
struct _Location {
    name: String,
    max_hp: u64,
    attack: u64,
}

impl _Location {
    fn print(&self) {
        println!("The {} has {} hp and does {} damage", &self.name, &self.max_hp, &self.attack);
    }
}