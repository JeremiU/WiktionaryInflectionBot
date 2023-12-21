use serde::{Serialize, Deserialize};
use strum::{EnumString, Display};
use strum_macros::EnumIter;

use crate::{NounNumericalCategory::*, WordGender::*, WordClass::*, match_txt};

#[derive(Debug, Clone, EnumIter, EnumString, Display, Copy)]
pub enum NounDeclension {
    NominativeSg, NominatePl, 
    GenitiveSg, GenitivePl,
    DativeSg, DativePl,
    AccusativeSg, AccusativePl,
    InstrumentalSg, InstrumentalPl,
    LocativeSg, LocativePl,
    VocativeSg, VocativePl,   
}

#[derive(Debug, Clone, PartialEq, EnumString, Display, Copy)]
pub enum WordClass {
    Noun,
    ProperNoun,
    Verb,
    Adjective,
    WordClassError
}

impl WordClass {
    pub fn match_txt(str: &str) -> WordClass {
        return match_txt(&[(vec!("===Noun==="), Noun), (vec!("===Adjective==="), Adjective), (vec!("===Verb==="), Verb), (vec!("===Proper noun==="), ProperNoun)], WordClassError, false, str);
    }
}

#[derive(Debug, Clone, PartialEq, EnumString, Display, Copy)]
pub enum WordGender {
    Feminine,
    Neuter,
    MasculineAnim,
    MasculineInam,
    MasculinePers,
    NVir,
    WordGenderError
}

impl WordGender {
    pub fn value(&self) -> &str {
        return match *self {
            Feminine => "f",
            Neuter => "n",
            MasculinePers => "m-pr",
            MasculineAnim => "m-an",
            MasculineInam => "m-in",
            NVir => "nv",
            _ => "",
        };
    }
    pub fn match_txt(str: &str) -> WordGender {
        return match_txt(&[(vec!("noun|nv", "g=nv"), NVir), (vec!("g=f", "noun|f", "noun-f"), Feminine),
        (vec!("m-in"), MasculineInam), (vec!("m-an"), MasculineAnim), (vec!("m-pr"), MasculinePers), 
        (vec!("noun|n", "g=n"), Neuter)], WordGenderError, false, str);
    }
}

#[derive(Debug, Clone, PartialEq, EnumString, Display, Copy)]
pub enum NounNumericalCategory {
    Singular,
    Plural,
    Both,
    NonNoun
}

impl NounNumericalCategory {
    pub fn size(&self) -> usize {
        return match *self {
            Singular | Plural => 7,
            Both => 14,
            NonNoun => 0
        }
    }
    pub fn match_txt(str: &str) -> NounNumericalCategory {
        return if str.contains("tatum=p") || str.contains("num=p") {Plural}
        else if str.contains("tatum=s") || str.contains("num=s") {Singular} else {Both};    
    }
}

#[derive(Clone, Debug)]
pub struct Word {
    pub lemma: Lemma,
    pub wiki_data: WikiContent,
    pub inflected_words: Vec<InflectionData>, // cat - pg - note
    pub pages: Vec<Page>
}

#[derive(Clone, Debug)]
pub struct Lemma {
    pub word: String,
    pub gender: WordGender,
    pub class: WordClass,
    pub num_cat: NounNumericalCategory,    
}

#[derive(Clone, Debug)]
pub struct InflectionData {
    pub inflected_word: String,
    pub keys: String,
    pub notes: String,
    pub pronounciation_base: String,
}

#[derive(Clone, Debug)]
pub struct Key {

}

#[derive(Clone, Debug)]
pub struct Page {
    pub title: String,
    pub body: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WikiLink {
    #[serde(rename = "title")]
    pub word: String,
    pub exists: bool
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Parse {
    #[serde(rename = "title")]
    pub word: String,
    // #[serde(rename = "pageid")]
    // pub page_id: i32,
    #[serde(rename = "text")]
    pub html_text: String,
    pub links: Vec<WikiLink>,
    // pub showtoc: bool,
    #[serde(rename = "wikitext")]
    pub wiki_text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WikiContent {
    pub parse: Parse
}

#[derive(Debug, Deserialize)]
pub struct WebData {
    pub api_url: String,
    pub acc_tok: String
}