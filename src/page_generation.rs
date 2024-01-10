use crate::{str_split, Lemma, InflectionData, Page, WordClass, WordClass::*, NounNumericalCategory::*};

//NEEDS PRONUNCIATION WORK
fn gen_pg_hd(class: &WordClass, notes: &str) -> String {
    let mut page_markup = String::new();
    let mut pronunciation = String::new(); //will not work

    //if exists, \n
    page_markup.push_str("==Polish==\n");
    page_markup.push_str("\n");

    if notes.len() > 0 {
        page_markup.push_str("===Alternative forms===\n");
        page_markup.push_str(&*format!("* {{{{l|pl|{}}}}}\n", str_split(&notes, "-")[1]));
        page_markup.push_str("\n");
    }

    page_markup.push_str("===Pronunciation===\n");
    if !pronunciation.is_empty() {
        pronunciation = format!("|{pronunciation}")
    };

    page_markup.push_str(&*format!("{{{{pl-p{pronunciation}}}}}\n"));
    page_markup.push_str("\n");
    page_markup.push_str(&*format!("==={}===\n", class.to_string().replace("_", " ")));

    page_markup.to_owned()
}

fn gen_noun(lemma: &Lemma, inflected_data: &InflectionData) -> Page {
    let mut page_markup = gen_pg_hd(&Noun, &inflected_data.notes).to_owned();

    page_markup.push_str(&*["{{head|pl|noun form|g=", lemma.gender.value(), "}}\n\n"].join(""));

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
                page_markup.push_str(&*format!("#{note_prefix} {{{{inflection of|pl|{}||{}|s|;|{}|p}}}}\n", lemma.word, sg.join("//"), pl.join("//")));
            } else if sg.len() > 0 {
                page_markup.push_str(&*format!("#{note_prefix} {{{{inflection of|pl|{}||{}|s}}}}\n", lemma.word, sg.join("//")));
            } else if pl.len() > 0 {
                page_markup.push_str(&*format!("#{note_prefix} {{{{inflection of|pl|{}||{}|p}}}}\n", lemma.word, pl.join("//")));
            }
        },
        Singular => {
            page_markup.push_str(&*format!("#{note_prefix} {{{{inflection of|pl|{}||{}|s}}}}\n", lemma.word, sg.join("//")));
        },
        Plural => {
            page_markup.push_str(&*format!("#{note_prefix} {{{{inflection of|pl|{}||{}|p}}}}\n", lemma.word, pl.join("//")));
        },
        _ => {},
    }

    Page {title: inflected_data.inflected_word.to_owned(), body: page_markup}
}

fn gen_adj(_lemma: &Lemma, inflected_data: &InflectionData) -> Page {
    let mut page_markup = gen_pg_hd(&Adjective, &inflected_data.notes);
    page_markup.push_str(r"{{head|pl|adjective form}}\n");

    for key in str_split(&inflected_data.keys, "/") {
        match key.as_str() {
            "nom|voc|v_pl" => {},
            _ => {}
        }
        println!("{}: {key}", inflected_data.inflected_word);
    }
    Page {title: inflected_data.inflected_word.to_owned(), body: page_markup}
}

pub fn gen_pg(lemma: &Lemma, inflected_data: &InflectionData) -> Page {
    match &lemma.class {
        Adjective => gen_adj(lemma, inflected_data),
        Noun => gen_noun(lemma, inflected_data),
        Verb => Page { title: String::new(), body: String::new()},
        _ => Page { title: String::new(), body: String::new()},
    }
}