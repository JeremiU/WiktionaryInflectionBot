use strum::{EnumString, Display};

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