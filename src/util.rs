use std::{time::{SystemTime, UNIX_EPOCH}, convert::TryInto, fs::File, io::Read};
use regex::Regex;

use crate::{InflectionData, WebData};
use crate::constants::err_code;

pub async fn raw_html(client: &reqwest::Client, word: &str) -> String {
    let response = client.get(format!("https://en.wiktionary.org/wiki/{word}#Polish"))
    .send()
    .await.expect(&*err_code("HTML 1")).text().await.expect(&*err_code("HTML 2"));

    let document = scraper::Html::parse_document(&response);
    return document.html();
}

pub fn find_line(input_arr: &Vec<String>, term: &str) -> i32 {
    for x in 0..input_arr.len() {
        if input_arr[x].contains(&term) {
            return x.try_into().expect(&*err_code("FL 1"));
        }
    }
    return -1;
}

pub fn str_split(input: &str, split: &str) -> Vec<String> {
    return input.split(split).map(String::from).collect::<Vec<_>>().to_vec();
}

pub fn str_cut(string: &str, i: usize, j: usize) -> String {
    return (&string.char_indices().clone().skip(i).take(j)).clone().map(|(_, c)| c).collect();
}

pub fn par_cont(pair: &Vec<InflectionData>, val: &str) -> bool {
    let mut cont = false;

    for p in pair {
        if p.inflected_word.eq(val) || p.inflected_word == val {
            cont = true;
        }
    }
    return cont;
}

pub fn match_txt<T: Copy>(pairs: &[(Vec<&str>, T)], unresolved: T, full_match: bool, str: &str) -> T {
    for (keys, val) in pairs {
        for key in keys {
            if !full_match {
                if str.contains(*key) {
                    return *val;
                }
            } else {
                if str.eq(*key) {
                    return *val;
                }
            }    
        }
    }
    return unresolved;
}

//Returns the val in key if found, otherwise, empty string
pub fn extract_txt(key: &str, val: &str) -> String {
    let pat = Regex::new(val).expect(&*err_code("Ex_Txt 1"));

    if let Some(_) = pat.captures(&key) {
        if let Some(captures) = pat.captures(&key) {
            if let Some(matched_text) = captures.get(1) {
                return matched_text.as_str().to_owned();
            }
        }
    }
    return String::new();
}

pub fn until_char(str: &str, stop_flag: &str) -> String {
    return if str.contains(stop_flag) {
        str[0..str.find(stop_flag).expect(&*err_code("until_char 1"))].to_owned()
    } else {
        str.to_owned()
    }
}

pub fn sys_time() -> u128 {
    return match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_nanos(),
        Err(_) => 0
    };
}

pub fn ns_to_s(s: u128) -> f64 {
    return (((s as f64) / 1_000_000_000.0) * 1000.0).round() / 1000.0;
}

pub fn client_data() -> WebData {
    let file_path = "WebData.json";
    let mut file = File::open(file_path).expect(&*err_code("Unable to read WebData.json"));
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect(&*err_code("Unable to read WebData.json"));
    return serde_json::from_value(serde_json::from_str(&contents).expect(&*err_code("CD_1"))).expect(&*err_code("CD_2"));
}