use regex::Regex;

use crate::{find_line, raw_html, str_split, constants::HTML_PL_HEADER};

//doesn't work with mult. etymology words
pub async fn get_wrd_sect(client: &reqwest::Client, word: &str) -> i8 {
    let raw_html = raw_html(client, word).await;
    let raw_html = str_split(&raw_html, "\n");
    
    let pl_hd: usize = find_line(&raw_html, HTML_PL_HEADER).try_into().unwrap();
    let mut counter = 0;

    for i in 0..raw_html.len() {
        let txt = &raw_html[i];
        if txt.starts_with("<h3>") {
            counter = counter + 1;
            
        }
        if txt.starts_with("<h3>") {
            let pat_wrd = Regex::new(r"Noun</span>").unwrap();
            if let Some(_) = pat_wrd.captures(txt) {
                if i > pl_hd {
                    return counter;
                }               
            }
        }
    }
    return counter;
}

pub fn check_wrd() {}