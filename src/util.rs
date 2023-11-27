use std::convert::TryInto;

#[derive(Debug, Clone)]
pub enum WordType {
    Noun,
    Verb,
    Adjective,
    TypeError
}

#[derive(Debug, Clone)]
pub enum WordGender {
    Feminine,
    Neuter,
    MasculineAnim,
    MasculineInam,
    MasculinePers,
    Ungendered
}

pub fn raw_html(word: &str) -> String {
    let response = reqwest::blocking::get(format!("https://en.wiktionary.org/wiki/{word}#Polish"))
    .unwrap()
    .text()
    .unwrap();    

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

pub fn str_split(input: &String, split: &str) -> Vec<String> {
    return input.split(split).map(String::from).collect::<Vec<_>>().to_vec();
}

pub fn par_cont<T: std::cmp::PartialEq>(pair: &Vec<(T, T)>, key: &T) -> bool {
    let mut cont = false;
    for p in pair {
        let (k, _) = p;        
        if k == key {
            cont = true;
        }
    }
    return cont;
}