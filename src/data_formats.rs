use serde::{Serialize, Deserialize};
use strum::{EnumString, Display};

#[derive(Debug, Clone, PartialEq, EnumString, Display, Copy)]
pub enum WordClass {
    Noun,
    ProperNoun,
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
    NVir,
    Ungendered
}

impl WordGender {
    pub fn value(&self) -> &str {
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
    NonNoun
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
    pub pronounciation: String,
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