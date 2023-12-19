use std::convert::TryInto;
use regex::Regex;

use crate::InflectionData;

pub async fn raw_html(client: &reqwest::Client, word: &str) -> String {
    let response = client.get(format!("https://en.wiktionary.org/wiki/{word}#Polish"))
    .send()
    .await.unwrap().text().await.unwrap();

    let document = scraper::Html::parse_document(&response);
    return document.html();
}

pub fn find_line(input_arr: &Vec<String>, term: &str) -> i32 {
    for x in 0..input_arr.len() {
        if input_arr[x].contains(&term) {
            return x.try_into().unwrap();
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
    let pat = Regex::new(val).unwrap();

    if let Some(_) = pat.captures(&key) { //deprecative
        if let Some(captures) = pat.captures(&key) {
            if let Some(matched_text) = captures.get(1) {
                return matched_text.as_str().to_owned();
            }
        }
    }
    return "".to_owned();
}