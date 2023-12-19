use crate::{str_split, Lemma, InflectionData, Page, WordClass, WordClass::*, NounNumericalCategory::*};

//NEEDS PRONOUNCIATION WORK
fn gen_pg_hd(class: &WordClass, notes: &str) -> String {
    let mut page_markup: String = String::new();
    let pronounciation = ""; //will not work

    //if exists, \n
    page_markup.push_str("==Polish==\n");
    page_markup.push_str("\n");

    if notes.len() > 0 {
        page_markup.push_str("===Alternative forms===\n");
        page_markup.push_str(format!("* {{{{l|pl|{}}}}}\n", str_split(&notes, "-")[1]).as_str());
        page_markup.push_str("\n");
    }

    page_markup.push_str("===Pronunciation===\n");
    let pronounciation = if !pronounciation.is_empty() {"|".to_owned() + pronounciation} else {"".to_owned()};

    page_markup.push_str(format!("{{{{pl-p{}}}}}\n", pronounciation).as_str());
    page_markup.push_str("\n");
    page_markup.push_str(format!("==={}===\n", class.to_string().replace("_", " ")).as_str());

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

    return Page {title: inflected_data.inflected_word.clone(), body: page_markup};
}

fn gen_adj(lemma: &Lemma, inflected_data: &InflectionData) -> Page {
    let mut page_markup: String = gen_pg_hd(&Adjective, &inflected_data.notes);

    return Page {title: inflected_data.inflected_word.clone(), body: page_markup};
}

pub fn gen_pg(lemma: &Lemma, inflected_data: &InflectionData) -> Page {
    match &lemma.class {
       Noun => gen_noun(lemma, inflected_data),
       Verb => Page { title: "".to_string(), body: "".to_string()},
       Adjective => Page { title: "".to_string(), body: "".to_string()},
       _ => Page { title: "".to_string(), body: "".to_string()}, 
    }
}