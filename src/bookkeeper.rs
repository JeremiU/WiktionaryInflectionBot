use std::env;
use std::fs::{File, OpenOptions, read_to_string};
use std::io::Write;
use std::time::{Duration, UNIX_EPOCH};
use chrono::{DateTime, Utc};
use chrono_tz::America::New_York;
use serde::{Deserialize, Serialize};
use serde_json::from_str;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bookkeeper {
    pub count: i32,
    pub last_updated: u128
}

pub fn get_keeper() -> Bookkeeper {
    let mut dir = String::new();
    match env::current_dir() {
        Ok(path) => {
            dir = path.to_str().unwrap().to_owned();
        },
        Err(e) => println!("Error getting current directory: {}", e),
    }
    return from_str(&read_to_string(format!("{}//bookkeeper.json", dir)).expect("err1")).expect("err2");
}

pub fn get_ts() -> u128 {
    return get_keeper().last_updated;
}

pub fn get_count() -> i32 {
    return get_keeper().count;
}

pub fn increment_count(amt: i32) {
    write(Bookkeeper {
        count: get_count() + amt,
        last_updated: get_ts(),
    });
}

pub fn update_ts(ts: u128) {
    write(Bookkeeper {
        count: get_count(),
        last_updated: ts,
    });
}

pub fn append_list(words: Vec<&str>) {
    let mut rep = "\n".to_owned();
    for word in words {
        rep.push_str(format!("\"{}\", ",word).as_str());
    }

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("bookkeeper.txt").expect("Err opening bookkeeper.txt !");
    file.write_all(rep.as_bytes()).expect("Couldn't write to bookkeeper.txt !");
}

pub fn time() -> String {
    let datetime: DateTime<Utc> = (UNIX_EPOCH + Duration::from_nanos(get_ts() as u64)).into();
    return datetime.with_timezone(&New_York).format("%Y-%m-%d %H:%M:%S").to_string();
}

fn write(bookkeeper: Bookkeeper) {
    let json = serde_json::to_string(&bookkeeper).unwrap();
    let mut file = File::create("bookkeeper.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();
}