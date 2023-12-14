use std::convert::TryInto;

use crate::InflectionData;
use crate::util::raw_html;
use crate::util::find_line;
use crate::util::str_split;
use crate::util::par_cont;
use crate::constants::*;
use crate::data_formats::*;
use crate::data_formats::WordClass::*;
use crate::data_formats::WordGender::*;
use crate::data_formats::WordNumericalCategory::*;
use crate::page_generation::*;

use regex::Regex;

//pub - temp
fn entry(input: &str) -> Vec<String> {
    let lines = str_split(&input.to_string(), "\n");
    
    let start: usize = find_line(&lines, HTML_PL_HEADER).try_into().unwrap();

    //checks whether this wiktionary page has multiple languages, and if so, crops to only polish
    let tmp_cut = &lines[(start+1)..lines.len()];
    let mut next_lang_start = find_line(&tmp_cut.to_vec(), "<h2>") + 1;

    if next_lang_start == -1 {
        next_lang_start = (&lines).len() as i32;
    }    

    let end = start+next_lang_start as usize;

    return (&lines[start..end]).to_vec();       
}

/// Narrows down the input block into just the inflection table data. If you want to print to find new indices, do it here
fn table(k: &Vec<String>) -> Vec<String> {
    //check if multiple tables

    let tbl_a_indx = find_line(&k, HTML_INF_TBL).try_into().unwrap();

    let g = k[tbl_a_indx..].to_vec();
    let tbl_b_indx = find_line(&g, "</table>").try_into().unwrap();

    return g[0..tbl_b_indx].to_vec();
}

fn gender(raw_html: &Vec<String>) -> WordGender {
    let k = &mut 0;

    for _x in 0..raw_html.len() {
        if raw_html[_x].contains(HTML_GENDER) {
            *k = _x;
            break;
        }
    }
    let gen_line = &raw_html[*k];
    
    if *k == 0 {
        return Ungendered;
    }

    let l = &gen_line.find(HTML_GENDER).unwrap() + HTML_GENDER.len();
    let k = &gen_line.find("</span>").unwrap();
    
    let ba = gen_line[l..*k].to_string();

    return match &ba[(ba.len() - HTML_ID_LEN)..] {
        HTML_GND_FEM => Feminine,
        HTML_GND_NEU => Neuter,
        HTML_GND_M_A => MasculineAnim,
        HTML_GND_M_I => MasculineInam,
        HTML_GND_M_P => MasculinePers,
        &_ => Ungendered
    };
}

fn class(raw_html: &Vec<String>) -> WordClass {
    let class = &mut TypeError;

    for i in 0..raw_html.len() {
        let pairs = vec![(HTML_CLASS_N, Noun), (HTML_CLASS_A, Adjective), (HTML_CLASS_V, Verb)];

        for (ind, wrd) in pairs {
            if raw_html[i].contains(ind) {
                *class = wrd;
                return class.clone();
            }            
        }
    }
    return class.clone();
}

fn _pronounciation(_raw_html: &Vec<String>) -> String {
    
    return "".to_owned();
}

fn num_cat(raw_html: &Vec<String>) -> WordNumericalCategory {
    let num_cat = &mut NumericalCategoryError;
    
    let v = table(raw_html);

    //noun
    if v[5].contains("plural") {
        *num_cat = Plural;
    }
    if v[5].contains("singular") {
        *num_cat = Singular;
    }
    if v[7].contains("plural") ||  v[6].contains("plural") && v[4].contains("singular") { //plural nouns & adjs
        *num_cat = Both;
    }

    return num_cat.clone();
}

fn find_links(bit: &Vec<String>, wrd_type: &WordClass) -> Vec<InflectionData> {
    let to_check: &mut Vec<InflectionData> = &mut Vec::new();
    let p: &mut Vec<(&str, i32)> = &mut Vec::new();
   
    match wrd_type {
        &Noun => {
            if bit.len() >= 57 {
                *p = ID_PAIRS_NOUN.to_vec()
            } else {
                *p = ID_PAIRS_NOUN_SG.to_vec();
            }
        },
        &Adjective => *p = ID_PAIRS_ADJC.to_vec(),
        &Verb => 
            if bit.len() > 240 {
                *p = ID_PAIRS_VB_FULL.to_vec();
            } else {
                if bit[2].contains("imperfective") {
                    *p = ID_PAIRS_VB_IMP.to_vec();                    
                } else {
                    *p = ID_PAIRS_VB_PFT.to_vec();       
                }
            }
        &_ => *p = ID_PAIRS_NOUN.to_vec(),
    };
    
    for (key, ind) in p {
        let str_p = &bit[(*ind) as usize];
        
        let st = &str_p[(str_p.find("<a ").unwrap())..];
        let k = str_split(st, "href=");

        for i in 1..k.len() {
            let mut inflected_word = String::new();
            let mut notes = String::new();

            let pat_wrd = Regex::new(r">([^<]*)</a>").unwrap();
            let pat_dep = Regex::new(r"(deprecative)").unwrap();
            let pat_arc = Regex::new(r"(archaic)").unwrap();

            if let Some(captures) = pat_wrd.captures(&k[i]) {
                if let Some(matched_text) = captures.get(1) {
                    let extracted_text = matched_text.as_str();
                    inflected_word = extracted_text.to_string();
                }
            } else {
                panic!("ERR extr wrd: {}", k[i]);
            }

            if let Some(_) = pat_dep.captures(&k[i]) { //deprecative
                if let Some(captures) = pat_wrd.captures(&k[i-1]) {
                    if let Some(matched_text) = captures.get(1) {
                        let extracted_text = matched_text.as_str();
                        println!("ALT: {}", extracted_text);
                        notes = format!("deprecative-{}", &extracted_text);
                    }
                }
            }

            if let Some(_) = pat_arc.captures(&k[i]) { //archaic
                if let Some(captures) = pat_wrd.captures(&k[i-1]) {
                    if let Some(matched_text) = captures.get(1) {
                        let extracted_text = matched_text.as_str();
                        println!("ALT: {}", extracted_text);
                        notes = format!("archaic-{}", &extracted_text);
                    }    
                }
            }

            to_check.push(InflectionData {inflected_word: inflected_word.to_owned(), keys : key.to_owned(), notes});
        }
    }
    return to_check.to_vec();
}

/// Filters duplicate entries, i.e. where multiple inflections are indentical
fn wrd_dupe_filter(bit: Vec<InflectionData>) -> Vec<InflectionData> {
    let filtered: &mut Vec<InflectionData> = &mut Vec::new();
    
    for id in bit {
        if par_cont(filtered, &id.inflected_word) {
            let indx = filtered.iter().position(|i| i.inflected_word == id.inflected_word).unwrap();
            filtered[indx] = InflectionData {keys: id.keys + "/" + &filtered[indx].keys, ..id};
        } else {
            filtered.push(id);        
        }
    }
    return filtered.to_vec();
}

async fn no_dupes(client: &reqwest::Client, list: Vec<InflectionData>) -> Vec<InflectionData>  {
    let mut no_dupes: Vec<InflectionData> = Vec::new();

    for id in list {
        let lines = str_split(raw_html(client, &id.inflected_word).await.as_str(), "\n");
        if find_line(&lines, HTML_PL_HEADER) == -1 {
            no_dupes.push(id);
        }
    }
    return no_dupes.clone();
}

/// Takes in a word, returns a pair (word, Vec<(subword, subtype)>, Gender, Type)
async fn prep_word(client: &reqwest::Client, word: &str) -> Word {
    let raw_html: Vec<String> = entry(&raw_html(&client, word).await);

    let word = word.replace("_", " ");

    let gender = gender(&raw_html);
    let class = class(&raw_html);
    let num_cat = num_cat(&raw_html);

    let table = table(&raw_html);

    let inflected_words = no_dupes(client, wrd_dupe_filter(find_links(&table, &class))).await;

    let lemma = Lemma {word : word.clone(), gender, class, num_cat};

    let mut pgs = Vec::new();

    for inflected_word in &inflected_words {
        pgs.push(gen_pg(&lemma, &inflected_word));
    }

    return Word {lemma: lemma.clone(), inflected_words, pages : pgs.clone()};
}

//entry point
pub async fn process(client: &reqwest::Client, wrd: &str) -> Word {
    let word_data = prep_word(&client, wrd).await;

    println!("word: {}", word_data.lemma.word);
    println!("\tgender: {:?}", word_data.lemma.gender);
    println!("\tclass: {:?}", word_data.lemma.class);    

    if (word_data.inflected_words).len() == 0 {
        println!("All forms of {} exist!", &wrd);
    } else {

        let num = &mut 0;
        match word_data.lemma.class {
            Adjective => *num = 18,
            Noun => *num = 13,
            Verb => *num = 40,
            TypeError => *num = 0
        };
        println!("\tTotal pages: {}", *num);
        println!("\t{} page(s) to create: {:?}", (&word_data.inflected_words).len(), &word_data.inflected_words);
    }
    return word_data;
}