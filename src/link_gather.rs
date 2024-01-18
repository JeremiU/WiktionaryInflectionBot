use crate::{WikiContent, WikiLink, util::extract_txt, until_char, has_non_polish};

pub fn get_links(content: &WikiContent, _wrd: &str) -> Vec<WikiLink> {
    fn filter_link(title: &str, bad_keys: Vec<&str>) -> bool {
        let mut val: bool = true;

        for key in bad_keys {
            if title.contains(&key) {
                val = false;
            }
        }
        if title.starts_with("-") {
            val = false;
        }
        return val.to_owned();
    }
    fn filter_two(text: &str) -> String {
        let lines: Vec<String> = text.split("\n").map(|s| s.to_string()).collect();
        let mut j = 1;
        let mut k = lines.len() - 1;

        while j < lines.len() {
            if lines[j-1].contains("====Declension====") {
                break;
            } else {
                j = j + 1;
            }
        }
        while k < lines.len() && k > j {
            if lines[k].contains("====Collocations====") || lines[k].contains("====Related terms====") {
                break;
            } else {
                k = k - 1;
            }
        }
        return if j == k {
            lines[j].to_owned()
        } else {
            lines[j..k].join("\n")
        }
    }

    let txt = filter_two(&content.wiki_text.to_owned()).replace("{","").replace("}","");
    if txt.contains("nolinks") {
        return Vec::new();
    }

    let parts: Vec<&str> = txt.split("|").collect();

    //Have fun reading !
    //Complete documentation:
    //      https://en.wiktionary.org/wiki/Module:pl-noun
    //Basic documentation (LACKING):
    //      https://en.wiktionary.org/wiki/Category:Polish_noun_inflection-table_templates
    match parts[0] {
        "pl-decl-noun-m-in" => {}
        "pl-decl-noun-m-an" => {}
        "pl-decl-noun-m-pr" => {
            println!("Wow: {:?}", txt.split("|").collect::<Vec<&str>>());
            println!("Nominative | SG: {} \\ PL: {}", parts[4], format!("{}/{}{}y", parts[3], parts[1], parts[2]));
        }
        "pl-decl-noun-f" => {}
        "pl-decl-noun-f-adj" => {}
        "pl-decl-noun-n" => {}
        "pl-decl-noun-n-adj" => {}
        _ => {}
    }

    let mut parsed: Vec<WikiLink> = content.links.clone().into_iter().filter(|x| filter_link(&x.word, vec!["File:", "Wikipedia","Wiktionary:","Appendix:","Rhymes:", "\\"])).collect();
    parsed = parsed.into_iter().filter(|x| !has_non_polish(&x.word.to_lowercase())).collect();
    // parsed.sort_by_key(|word| edit_distance(&word.word, wrd));

    let inf_tmps = vec!["noms=","gens=","dats=","accs=","inss=","vocs=","locs=","nomp=","genp=","datp=","accp=","insp=","vocp=","locp="];
    for inf_tmp in inf_tmps {
        if txt.contains(inf_tmp) {
            let _man_input = until_char(&*extract_txt(&txt, format!("\\|{inf_tmp}([^<]*)").as_str()), "|");
            //TODO: use manual input to filter
        }
    }
    return parsed;
}