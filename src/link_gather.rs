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

    let txt = filter_two(&content.wiki_text.to_owned());
    if txt.contains("nolinks") {
        return Vec::new();
    }

    let mut parsed: Vec<WikiLink> = content.links.clone().into_iter().filter(|x| filter_link(&x.word, vec!["File:", "Wikipedia","Wiktionary:","Appendix:","Rhymes:", "\\"])).collect();
    parsed = parsed.into_iter().filter(|x| !has_non_polish(&x.word.to_lowercase())).collect();
    // parsed.sort_by_key(|word| edit_distance(&word.word, wrd));

    let inf_tmps = vec!["noms=","gens=","dats=","accs=","inss=","vocs=","locs=","nomp=","genp=","datp=","accp=","insp=","vocp=","locp="];
    let x = txt.replace("{{", "").replace("}}", "");
    for inf_tmp in inf_tmps {
        if txt.contains(inf_tmp) {
            let _man_input = until_char(&*extract_txt(&x, format!("\\|{inf_tmp}([^<]*)").as_str()), "|");
            //TODO: use manual input to filter
        }
    }

    let dec_tmps = vec!["pl-decl-noun-m-in", "pl-decl-noun-m-an","pl-decl-noun-m-pr","pl-decl-noun-f","pl-decl-noun-n","pl-decl-noun-f-adj","pl-decl-noun-n-adj"];

    for dec_tmp in dec_tmps {
        let _root = extract_txt(&x, format!("\\|{dec_tmp}([^<]*)").as_str());
    }
    return parsed;
}