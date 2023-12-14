use std::convert::TryInto;

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