use std::convert::TryInto;
use strum::{EnumString, Display};

use crate::util::WordClass::*;
use crate::util::WordNumericalCategory::*;

#[derive(Debug, Clone, PartialEq, EnumString, Display, Copy)]
pub enum WordClass {
    Noun,
    Verb,
    Adjective,
    TypeError
}

#[derive(Debug, Clone, PartialEq, EnumString, Display, Copy)]
pub enum WordGender {
    Feminine,
    Neuter,
    MasculineAnim,
    MasculineInam,
    MasculinePers,
    Ungendered
}

impl WordGender {
    fn value(&self) -> &str {
        return match *self {
            WordGender::Feminine => "f",
            WordGender::Neuter => "n",
            WordGender::MasculinePers => "m-pr",
            WordGender::MasculineAnim => "m-an",
            WordGender::MasculineInam => "m-in",
            _ => "",
        };
    }
}

#[derive(Debug, Clone, PartialEq, EnumString, Display, Copy)]
pub enum WordNumericalCategory {
    Singular,
    Plural,
    Both,
    Noncountable,
    NumericalCategoryError
}

#[derive(Clone)]
pub struct Word {
    pub lemma: Lemma,
    pub inflected_words: Vec<InflectionData>, // cat - pg - note
    pub pages: Vec<Page>
}

#[derive(Clone)]
pub struct Lemma {
    pub word: String,
    pub gender: WordGender,
    pub class: WordClass,
    pub num_cat: WordNumericalCategory,    
}

#[derive(Clone, Debug)]
pub struct InflectionData {
    pub inflected_word: String,
    pub keys: String,
    pub notes: String
}

#[derive(Clone)]
pub struct Page {
    pub title: String,
    pub body: String
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

pub fn par_cont(pair: &Vec<InflectionData>, val: &str) -> bool {
    let mut cont = false;

    for p in pair {
        if p.inflected_word.eq(val) || p.inflected_word == val {
            cont = true;
        }
    }
    return cont;
}

fn gen_pg_hd(class: &WordClass, notes: &str) -> String {
    let mut page_markup: String = String::new();

    //if exists, \n
    page_markup.push_str("==Polish==\n");
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

    return page_markup;
}

fn gen_noun(lemma: &Lemma, inflected_data: &InflectionData) -> Page {
    let mut page_markup: String = gen_pg_hd(&Noun, &inflected_data.notes);

    page_markup.push_str(["{{head|pl|noun form|g=", lemma.gender.value(), "}}\n"].join("").as_str());
    page_markup.push_str("\n");


    let mut sg: Vec<String> = Vec::new();
    let mut pl: Vec<String> = Vec::new();

    for key in str_split(&inflected_data.keys, "/") {
        if key.ends_with("pl") {
            pl.push(str_split(&key, "_")[0].to_owned());
        } else {
            sg.push(str_split(&key, "_")[0].to_owned());
        }
    }

    let mut note_prefix = String::new();

    if inflected_data.notes.len() > 0 {
        note_prefix = format!("{{{{lb|pl|{}}}}}", str_split(&inflected_data.notes, "-")[0]);
    }
    
    match lemma.num_cat {
        Both => {
            if sg.len() > 0 && pl.len() > 0 {
                let str = format!("#{} {{{{inflection of|pl|{}||{}|s|;|{}|p}}}}\n", note_prefix, lemma.word, sg.join("//"), pl.join("//"));
                page_markup.push_str(&str);    
            } else if sg.len() > 0 {
                page_markup.push_str(format!("#{} {{{{inflection of|pl|{}||{}|s}}}}\n", note_prefix, lemma.word, sg.join("//")).as_str());
            } else if pl.len() > 0 {
                page_markup.push_str(format!("#{} {{{{inflection of|pl|{}||{}|p}}}}\n", note_prefix, lemma.word, pl.join("//")).as_str());
            }
        },
        Singular => {
            page_markup.push_str(format!("#{} {{{{inflection of|pl|{}||{}|s}}}}\n", note_prefix, lemma.word, sg.join("//")).as_str());
        },
        Plural => {
            page_markup.push_str(format!("#{} {{{{inflection of|pl|{}||{}|p}}}}\n", note_prefix, lemma.word, pl.join("//")).as_str());
        },
        _ => {},
    }

    return Page {title: inflected_data.inflected_word, body: page_markup};
}

pub fn gen_pg(lemma: &Lemma, inflected_data: &InflectionData) -> Page {
    match &lemma.class {
       Noun => gen_noun(lemma, inflected_data),
       Verb => Page { title: "".to_string(), body: "".to_string()},
       Adjective => Page { title: "".to_string(), body: "".to_string()},
       _ => Page { title: "".to_string(), body: "".to_string()}, 
    }
}