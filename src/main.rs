use std::io::stdin;
use std::fs;
use std::thread;
use std::time::Duration;
use serde::{Serialize, Deserialize};
use yansi::{Paint,Color};
//use serde_json::{Key, Value};

// Boilerplate is a bit overkill of a template, but covers a lot
// of my most used crates and features. Get coding faster.

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
struct JsonObject {
    name: String,
}

fn user_input() -> String {
    let mut buffer = String::new();
    match stdin().read_line(&mut buffer) {
        Ok(_x) => return buffer.to_string(),
        Err(_e) => return "Error".to_string()
    }
}

fn load_json_to_string(file_path: String) -> String {
    fs::read_to_string(file_path).unwrap_or_default()
}

// Depending on JSON format and usage, Vec<JsonObject>
fn json_object(json_string: String) -> JsonObject {
    serde_json::from_str(&load_json_to_string(json_string))
    .unwrap_or_default()
}

fn main() {
    let _time_thread = thread::spawn(|| {
        for _i in 0..10 {
            println!("{}", Paint::green("Color and time check"));
            thread::sleep(Duration::from_secs(1));
        }
    });

    let mut choice = String::new();
    let save_file = json_object("./src/some.json".to_owned());
    while choice != "quit" {
        choice = user_input().trim().to_lowercase().replace("\n","");
        println!(" >{}", save_file.name);
    }

    //time_thread.join().unwrap();
}
