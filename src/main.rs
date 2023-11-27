mod util;

use std::convert::TryInto;

use util::raw_html;
use util::find_line;
use util::vec_slice;
use util::str_split;
use util::par_cont;


fn table(input: &str) -> Vec<String> {
    let y = str_split(input.to_string(), "\n");
    let len = y.len().try_into().unwrap();      

    let pol_indx = find_line(&y, "Polish</span><span class=\"mw-editsection\">").try_into().unwrap();
    let k = vec_slice(&y, pol_indx, len);

    let tbl_a_indx = find_line(&k, "class=\"inflection-table\"").try_into().unwrap();
    let tbl_b_indx = find_line(&k, "</td></tr></tbody></table></div></div>").try_into().unwrap();

    return vec_slice(&k, tbl_a_indx, tbl_b_indx);
}

fn find_links(bit: Vec<String>) -> Vec<(String, String)> {
    let (nom_sg, nom_pl, gen_sg, gen_pl, dat_sg, dat_pl, acc_sg, acc_pl, ins_sg, ins_pl, loc_sg, loc_pl, voc_sg, voc_pl) = (12, 14, 19, 21, 26, 28, 33, 35, 40, 42, 47, 49, 54, 56);
    
    let to_check: &mut Vec<(String, String)> = &mut Vec::new();
    let slice = &[("nom sg", nom_sg), ("nom_pl", nom_pl), ("gen_sg", gen_sg), ("gen_pl", gen_pl), ("dat_sg", dat_sg), ("dat_pl", dat_pl), ("acc_sg", acc_sg), ("acc_pl", acc_pl),  ("ins_sg", ins_sg), ("ins_pl", ins_pl), ("loc_sg", loc_sg), ("loc_pl", loc_pl), ("voc_sg", voc_sg), ("voc_pl", voc_pl)];

    for (name, ind) in slice {
        let str_p = &bit[(*ind) as usize];

        let st = &str_p[(str_p.find("<a ").unwrap())..];

        if st.contains("(page does not exist)") {
            to_check.push((name.to_string(), st.to_string()));
        }
    }
    return to_check.to_vec();
}

fn format(bit: Vec<(String, String)>) -> Vec<(String, String)> {
    let mut ret = Vec::new();
    for (inf, ind) in bit {
        let i = ind.find("\">").unwrap() + "\">".len();
        let j = ind.find("</a>").unwrap();
        ret.push((ind[i..j].to_string(), inf));
    }
    return ret.to_vec();
}

fn wrd_dupe_filter(bit: Vec<(String, String)>) -> Vec<(String, String)> {
    let filtered: &mut Vec<(String, String)> = &mut Vec::new();
    
    for (k, v) in bit {
        if par_cont(filtered, &(k.clone().to_string())) {
            let indx = filtered.iter().position(|(k2, _)| k2 == &k).unwrap();
            let (_, old_v) = &filtered[indx];
            filtered[indx] = (k, (v + "/" + &old_v));
        } else {
            filtered.push((k, v));        
        }
    }
    return filtered.to_vec();
}

fn gender_raw(bit: String) -> String {
    let lines = str_split(bit, "\n");
    let j: usize = find_line(&lines, "Polish</span><span class=\"mw-editsection\">").try_into().unwrap();
    let nlines = &lines[j..lines.len()];

    let k = &mut 0;

    for _x in 0..nlines.len() {
        if nlines[_x].contains("<span class=\"gender\">") {
            *k = _x;
            break;
        }
    }
    let gen_line = &nlines[*k];
    
    let l = &gen_line.find("<span class=\"gender\">").unwrap() + "<span class=\"gender\">".len();
    let k = &gen_line.find("</span>").unwrap();
    return gen_line[l..*k].to_string();
}

fn gender(bit: &str) -> String {
    let last_digs = ">f</abbr>".len();
    return (match &bit[(bit.len() - last_digs)..] {
        ">f</abbr>" => "f",
        ">n</abbr>" => "n",
        "im</abbr>" => "m-a",
        "an</abbr>" => "m-i",
        "rs</abbr>" => "m-p",
        &_ => "err"
    }).to_string();
}

fn process(wrd: &str) -> Vec<(String, String)> {
    let url_data = raw_html(&wrd);
    let x = table(&url_data);
    let frm = wrd_dupe_filter(format(find_links(x)));

    println!("word: {}", wrd);
    println!("\tgender: {}", gender(&gender_raw(url_data)));

    if (&frm).len() == 0 {
        println!("\tAll forms of {} exist!", &wrd);
    } else {
        println!("\texisting pages: {}/13", 13-(&frm).len());
        println!("\tpages to create: {:?}", &frm);
    }
    return frm;
}

fn main() {
    process("wałówka");
    process("okno");
    process("koń");
    process("kwadrat");
}