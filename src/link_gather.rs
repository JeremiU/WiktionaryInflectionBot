use crate::{WikiContent, WikiLink, util::extract_txt, until_char};
use edit_distance::edit_distance;
use regex::Regex;

pub fn get_links(content: WikiContent, wrd: &str) -> Vec<WikiLink> {
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
        if j == k {
            return lines[j].to_owned();
        } else {
            return lines[j..k].join("\n");
        }
    }

    let mut parsed: Vec<WikiLink> = content.links.into_iter().filter(|x| filter_link(x.word.as_str(), vec!["File:", "Wikipedia","Wiktionary:","Appendix:","Rhymes:"])).collect();
    parsed.sort_by_key(|word| edit_distance(&word.word, wrd));

    let v = filter_two(&content.wiki_text);

    if v.contains("nolinks") {
        return Vec::new();
    }

    let inf_tmps = vec!["noms=","gens=","dats=","accs=","inss=","vocs=","locs=","nomp=","genp=","datp=","accp=","insp=","vocp=","locp="];
    
    for inf_tmp in inf_tmps {
        if v.contains(inf_tmp) {
            let x = v.replace("{{", "").replace("}}", "");
            let man_input = until_char(extract_txt(&x, format!("\\|{}([^<]*)", inf_tmp).as_str()).as_str(), "|");
            //TODO: use manual input to filter
        }
    }
    return parsed;
}