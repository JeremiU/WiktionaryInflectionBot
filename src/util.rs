use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq)]
pub enum WordClass {
    Noun,
    Verb,
    Adjective,
    TypeError
}

#[derive(Debug, Clone, PartialEq)]
pub enum WordGender {
    Feminine,
    Neuter,
    MasculineAnim,
    MasculineInam,
    MasculinePers,
    Ungendered
}

pub struct Word {
    pub word: String,
    pub inflected_words: Vec<(String, String)>,
    pub gender: WordGender,
    pub class: WordClass
}

pub fn raw_html(word: &str) -> String {
    let response = reqwest::blocking::get(format!("https://en.wiktionary.org/wiki/{word}#Polish")).unwrap().text().unwrap();    
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

pub fn par_cont<T: std::cmp::PartialEq>(pair: &Vec<(T, T)>, val: &T) -> bool {
    let mut cont = false;

    for p in pair {
        let (_, v) = p;        
        if v.eq(val) || v == val {
            cont = true;
        }
    }
    return cont;
}