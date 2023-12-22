use core::panic;
use std::convert::TryInto;
use regex::Regex;

use crate::{InflectionData, constants::*, data_formats::*, data_formats::WordClass::*, page_generation::*};
use crate::util::{extract_txt, find_line, str_split, par_cont, raw_html};
use crate::wikt_text;

fn entry(input: &str) -> Vec<String> {
    let lines = str_split(&input.to_string(), "\n");
    let start: usize = find_line(&lines, HTML_PL_HEADER).try_into().unwrap();
    //checks whether this wiktionary page has multiple languages, and if so, crops to only polish
    let tmp_cut = &lines[(start+1)..lines.len()];
    let mut next_lang_start = find_line(&tmp_cut.to_vec(), "<h2>") + 1;

    if next_lang_start == -1 {
        next_lang_start = (&lines).len() as i32;
    }    
    if next_lang_start == 0 {
        next_lang_start = ((&lines).len() - start) as i32;
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

fn gender(wiki_data: &WikiContent) -> WordGender {
    return WordGender::match_txt(&wiki_data.parse.wiki_text);
}

fn class(wiki_data: &WikiContent) -> WordClass {
    return WordClass::match_txt(&wiki_data.parse.wiki_text);
}

fn pronounciation(wiki_data: &WikiContent) -> String {
    let pro = extract_txt(&wiki_data.parse.wiki_text, r"\{\{pl-p\|([^|]*)\}\}"); 
    return if pro.starts_with("a=") {"".to_owned()} else {pro};
}

fn num_cat(wiki_data: &WikiContent) -> NounNumericalCategory {
    let wiki_text = &wiki_data.parse.wiki_text;
    return NounNumericalCategory::match_txt(&wiki_text);
}

fn find_links(wiki_data: &WikiContent, class: &WordClass) -> Vec<InflectionData> {
    let bit = table(&entry(&wiki_data.parse.html_text));
    let to_check: &mut Vec<InflectionData> = &mut Vec::new();
    let p: &mut Vec<(&str, i32)> = &mut Vec::new();
   
    match class {
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
        
        let st = &str_p[(str_p.find("<a").unwrap())..];
        let k = str_split(st, "href=");

        for i in 1..k.len() {
            let inflected_word = extract_txt(&k[i], r">([^<]*)</a>");
            let dep = &extract_txt(&k[i], r"(deprecative)");
            let arc = &extract_txt(&k[i], r"(archaic)");
            let mut notes = String::new();
            
            if !(*dep).is_empty() {
                notes = format!("deprecative-{}", extract_txt(&k[i-1], r">([^<]*)</a>"));
            }
            if !(*arc).is_empty() {
                notes = format!("archaic-{}", extract_txt(&k[i-1], r">([^<]*)</a>"));
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

fn get_noun_infl_wt(wiki_data: &WikiContent) -> Vec<InflectionData> {
    let mut infl_forms = Vec::new();
    let txt = &wiki_data.parse.wiki_text;
    let class = class(wiki_data);
    let num_cat = num_cat(wiki_data);

    let pat_wrd = Regex::new(r"|gens=([^<]*)").unwrap();

    if infl_forms.len() != num_cat.size() {
        panic!("Incorrect arr size!");
    } 
    return infl_forms;
}

/// Takes in a word, returns a pair (word, Vec<(subword, subtype)>, Gender, Type)
pub async fn process(client: &reqwest::Client, word: &str) -> Option<Word> {
    let wiki_data = wikt_text(client, word).await;
    if wiki_data.is_none() {
        return None;
    }
    let wiki_data = wiki_data?; 

    let word = word.replace("_", " ");

    let pronounciation_base = pronounciation(&wiki_data);
    let gender = gender(&wiki_data);
    let class = class(&wiki_data);
    let num_cat = num_cat(&wiki_data);
    println!("{}", wiki_data.parse.word);
    let inflected_words = wrd_dupe_filter(find_links(&wiki_data, &class));

    let lemma = Lemma {word : word.clone(), gender, class, num_cat};

    let mut pgs = Vec::new();

    for inflected_word in &inflected_words {
        pgs.push(gen_pg(&lemma, &inflected_word));
    }
    return Some(Word {lemma: lemma.clone(), wiki_data, inflected_words, pages : pgs.clone(), pronounciation_base});
}