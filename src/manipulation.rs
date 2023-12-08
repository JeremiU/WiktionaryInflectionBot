use std::convert::TryInto;

use crate::util::raw_html;
use crate::util::find_line;
use crate::util::str_split;
use crate::util::par_cont;
use crate::util::Word;
use crate::util::WordClass;
use crate::util::WordClass::*;
use crate::util::WordGender;
use crate::util::WordGender::*;
use crate::constants::*;

fn entry(input: &str) -> Vec<String> {
    let lines = str_split(&input.to_string(), "\n");
    
    let start: usize = find_line(&lines, HTML_PL_HEADER).try_into().unwrap();
    let pol_ent = &lines[start..lines.len()];

    return pol_ent.to_vec();       
}

/// Narrows down the input block into just the inflection table data.
fn table(k: &Vec<String>) -> Vec<String> {
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
        let pairs = vec![(HTML_CLASS_V, Verb), (HTML_CLASS_N, Noun), (HTML_CLASS_A, Adjective)];

        for (ind, wrd) in pairs {
            if raw_html[i].contains(ind) {
                *class = wrd;
            }            
        }
    }
    return class.clone();
}

fn find_links(bit: &Vec<String>, wrd_type: &WordClass) -> Vec<(String, String)> {
    let to_check: &mut Vec<(String, String)> = &mut Vec::new();
    let p: &mut Vec<(&str, i32)> = &mut Vec::new();
    
    match wrd_type {
        &Noun => *p = ID_PAIRS_NOUN.to_vec(),
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
        &_ => *p = ID_PAIRS_NOUN.to_vec(), //to fix
    };
    
    for (name, ind) in p {
        let str_p = &bit[(*ind) as usize];
        
        let st = &str_p[(str_p.find("<a ").unwrap())..];

        if st.contains("(page does not exist)") {
            let i = st.find("\">").unwrap() + "\">".len();
            let j = st.find("</a>").unwrap();

            to_check.push((name.to_string(), st[i..j].to_string() ));
        }
    }
    return to_check.to_vec();
}

/// Filters duplicate entries, i.e. where multiple inflections are indentical
fn wrd_dupe_filter(bit: Vec<(String, String)>) -> Vec<(String, String)> {
    let filtered: &mut Vec<(String, String)> = &mut Vec::new();
    
    for (k, v) in bit {
        if par_cont(filtered, &v.to_string()) {
            let indx = filtered.iter().position(|(_, v2)| v2 == &v).unwrap();

            let (old_k, _) = &filtered[indx];
            filtered[indx] = (k + "/" + &old_k, v);
        } else {
            filtered.push((k, v));        
        }
    }
    return filtered.to_vec();
}

/// Takes in a word, returns a pair (word, Vec<(subword, subtype)>, Gender, Type)
fn prep_word(word: String) -> Word {
    let url_data = raw_html(&word);
    let k: Vec<String> = entry(&url_data);

    let gender = gender(&k);
    let class = class(&k);
    let table = table(&k);

    let inflected_words = wrd_dupe_filter(find_links(&table, &class));
    
    return Word {word, inflected_words, gender, class};
}

fn gen_pg(word: &Word) -> Vec<(String, String)> {
    let pg_list: &mut Vec<(String, String)> = &mut Vec::new();

    for pair in &word.inflected_words {
        let mut sg: Vec<String> = Vec::new();
        let mut pl: Vec<String> = Vec::new();

        let (k, v) = pair;
        let k_cut = str_split(&k, "/");

        for k2 in k_cut {
            if k2.contains("_pl") {
                pl.push(k2.clone().replace("_pl",""));
            }
            if k2.contains("_sg") {
                sg.push(k2.clone().replace("_sg",""));
            }
        }

        let mut page_markup: String = String::new();

        page_markup.push_str("==Polish==\n");
        page_markup.push_str("\n");
        page_markup.push_str("===Pronunciation===\n");
        page_markup.push_str("{{pl-p}}\n");
        page_markup.push_str("\n");

        if word.class == Noun {
            page_markup.push_str("===Noun===\n");
            page_markup.push_str("{{head|pl|noun form}}\n");
            page_markup.push_str("\n");
        
            if sg.len() > 0 {
                page_markup.push_str(format!("# {{{{inflection of|pl|{}||{}|s}}}}\n", &word.word, sg.join("//")).as_str());
            }
            if pl.len() > 0 {
                page_markup.push_str(format!("# {{{{inflection of|pl|{}||{}|p}}}}\n", &word.word, pl.join("//")).as_str());
            }
        }
        page_markup.push_str("\n");
        pg_list.push((v.to_string(), page_markup));
    }
    return pg_list.to_vec();
}

pub fn process(wrd: &str) -> Word {
    let prep_pair = prep_word(wrd.to_string());

    println!("word: {}", &prep_pair.word);
    println!("\tgender: {:?}", &prep_pair.gender);
    println!("\tclass: {:?}", &prep_pair.class);

    if (&prep_pair.inflected_words).len() == 0 {
        println!("\tAll forms of {} exist!", &wrd);
    } else {
        let num = &mut 0;
        match &prep_pair.class {
            Noun => *num = 13,
            Verb => *num = 40,
            Adjective => *num = 18,
            TypeError => *num = 0
        };
        println!("\tTotal pages: {}", *num);
        println!("\t{} page(s) to create: {:?}", (&prep_pair.inflected_words).len(), &prep_pair.inflected_words);
    }
    
    let v = gen_pg(&prep_pair);


    for i in v {
        println!("*{}*\n{}", i.0 , i.1);
    }
    return prep_pair;
}