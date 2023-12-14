use std::convert::TryInto;
use strum::{EnumString, Display};

use crate::util::WordClass::*;
use crate::util::WordNumericalCategory::*;

#[derive(Debug, Clone, PartialEq, EnumString, Display)]
pub enum WordClass {
    Noun,
    Verb,
    Adjective,
    TypeError
}

#[derive(Debug, Clone, PartialEq, EnumString, Display)]
pub enum WordGender {
    Feminine,
    Neuter,
    MasculineAnim,
    MasculineInam,
    MasculinePers,
    Ungendered
}
#[derive(Debug, Clone, PartialEq, EnumString, Display)]
pub enum WordNumericalCategory {
    Singular,
    Plural,
    Both,
    Noncountable,
    NumericalCategoryError
}
pub struct Word {
    pub word: String,
    pub inflected_words: Vec<(String, String, String)>, // cat - pg - note
    pub gender: WordGender,
    pub class: WordClass,
    pub num_cat: WordNumericalCategory,
    pub pages: Vec<(String, String)>
}

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

pub fn par_cont<T: std::cmp::PartialEq>(pair: &Vec<(T, T, T)>, val: &T) -> bool {
    let mut cont = false;

    for p in pair {
        let (_, v,_) = p;        
        if v.eq(val) || v == val {
            cont = true;
        }
    }
    return cont;
}

fn gen_pg_hd(class: &WordClass, notes: &str) -> String {
    let mut page_markup: String = String::new();

    page_markup.push_str("\n==Polish==\n");
    page_markup.push_str("\n");

    if notes.len() > 0 {
        page_markup.push_str("===Alternative forms===\n");
        page_markup.push_str(format!("* {{{{l|pl|{}}}}}\n", str_split(&notes, "-")[1]).as_str());
        page_markup.push_str("\n");
    }

    page_markup.push_str("===Pronunciation===\n");
    page_markup.push_str("{{pl-p}}\n");
    page_markup.push_str("\n");
    page_markup.push_str(format!("==={}===\n", class).as_str());

    //add gender
    page_markup.push_str(["{{head|pl|",format!("{}", class.to_string().to_lowercase()).as_str()," form}}\n"].join("").as_str());
    
    page_markup.push_str("\n");

    return page_markup;
}

fn gen_noun(base_word: String, inflected_data: (String, String, String), num_cat: &WordNumericalCategory) -> (String, String) {
    let (keys, inflected_word, notes) = inflected_data;    
    let mut page_markup: String = gen_pg_hd(&Noun, &notes);

    let mut sg: Vec<String> = Vec::new();
    let mut pl: Vec<String> = Vec::new();

    for key in str_split(&keys, "/") {
        if key.ends_with("pl") {
            pl.push(str_split(&key, "_")[0].to_owned());
        } else {
            sg.push(str_split(&key, "_")[0].to_owned());
        }
    }

    let mut note_prefix = String::new();

    if notes.len() > 0 {
        note_prefix = format!("{{{{lb|pl|{}}}}}", str_split(&notes, "-")[0]);
    }
    
    match num_cat {
        Both => {
            if sg.len() > 0 && pl.len() > 0 {
                let str = format!("#{} {{{{inflection of|pl|{}||{}|s|;|{}|p}}}}\n", note_prefix, base_word, sg.join("//"), pl.join("//"));
                page_markup.push_str(&str);    
            } else if sg.len() > 0 {
                page_markup.push_str(format!("#{} {{{{inflection of|pl|{}||{}|s}}}}\n", note_prefix, base_word, sg.join("//")).as_str());
            } else if pl.len() > 0 {
                page_markup.push_str(format!("#{} {{{{inflection of|pl|{}||{}|p}}}}\n", note_prefix, base_word, pl.join("//")).as_str());
            }
        },
        Singular => {
            page_markup.push_str(format!("#{} {{{{inflection of|pl|{}||{}|s}}}}\n", note_prefix, base_word, sg.join("//")).as_str());
        },
        Plural => {
            page_markup.push_str(format!("#{} {{{{inflection of|pl|{}||{}|p}}}}\n", note_prefix, base_word, pl.join("//")).as_str());
        },
        _ => {},
    }

    return (inflected_word, page_markup);
}

pub fn gen_pg(base_word: String, inflected_word: (String, String, String), class: &WordClass, num_cat: &WordNumericalCategory) -> (String, String) {
    match &class {
       Noun => gen_noun(base_word, inflected_word, num_cat),
       Verb => ("".to_string(), "".to_string()),
       Adjective => ("".to_string(), "".to_string()),
       _ => ("".to_string(), "".to_string()), 
    }
}