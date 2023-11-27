use std::convert::TryInto;

use util::raw_html;
use util::find_line;
use util::str_split;
use util::par_cont;
use util::WordType;
use util::WordGender;
use util::WordType::*;
use util::WordGender::*;

fn entry(input: &str) -> Vec<String> {
    let lines = str_split(&input.to_string(), "\n");
    
    let start: usize = find_line(&lines, "Polish</span><span class=\"mw-editsection\">").try_into().unwrap();
    let pol_ent = &lines[start..lines.len()];

    return pol_ent.to_vec();       
}

fn table(input: &str) -> Vec<String> {
    let k = entry(input);

    let tbl_a_indx = find_line(&k, "class=\"inflection-table").try_into().unwrap();

    let g = k[tbl_a_indx..].to_vec();
    let tbl_b_indx = find_line(&g, "</table>").try_into().unwrap();

    return g[0..tbl_b_indx].to_vec();
}

fn find_links(bit: Vec<String>, wrd_type: WordType) -> Vec<(String, String)> {
    let to_check: &mut Vec<(String, String)> = &mut Vec::new();

    let (n_nom_pl, n_gen_sg, n_gen_pl, n_dat_sg, n_dat_pl, n_acc_sg, n_acc_pl, n_ins_sg, n_ins_pl, n_loc_sg, n_loc_pl, n_voc_sg, n_voc_pl) = (14, 19, 21, 26, 28, 33, 35, 40, 42, 47, 49, 54, 56);
    let n_slice = &vec![("nom_pl", n_nom_pl), ("gen_sg", n_gen_sg), ("gen_pl", n_gen_pl), ("dat_sg", n_dat_sg), ("dat_pl", n_dat_pl), ("acc_sg", n_acc_sg), ("acc_pl", n_acc_pl),  ("ins_sg", n_ins_sg), ("ins_pl", n_ins_pl), ("loc_sg", n_loc_sg), ("loc_pl", n_loc_pl), ("voc_sg", n_voc_sg), ("voc_pl", n_voc_pl)];
    
    let (a_nom_n, a_nom_f, a_nom_v, a_nom_nv) = (26, 28, 30, 32);
    let (a_gen_mn, a_gen_f, a_gen_pl) = (37, 39, 41);
    let (a_dat_mn, a_dat_pl) = (46, 48);
    let (a_acc_ma, a_acc_n, a_acc_f, a_acc_v, a_acc_nv) = (53, 57, 59, 61, 63);
    let (a_ins_mn, a_ins_pl) = (68, 70);
    let (a_loc_f, a_loc_pl) = (75, 77);

    let a_slice = &vec![("nom_voc_n", a_nom_n), ("nom_voc_f", a_nom_f), ("nom_voc_v", a_nom_v), ("nom_voc_nv", a_nom_nv),
        ("gen_mn", a_gen_mn), ("gen_dat_f", a_gen_f), ("gen_pl", a_gen_pl), ("dat_mn", a_dat_mn), ("dat_pl", a_dat_pl), 
        ("acc_ma", a_acc_ma), ("acc_n", a_acc_n), ("acc_ins_f", a_acc_f), ("acc_v", a_acc_v), ("acc_mv", a_acc_nv),
        ("ins_loc_mn", a_ins_mn), ("a_ins_pl", a_ins_pl), ("loc_f", a_loc_f), ("loc_pl", a_loc_pl)
    ];

    let p;

    match wrd_type {
        Noun => p = n_slice,
        Adjective => p = a_slice,
        _ => p = n_slice,
    };

    for (name, ind) in p {
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

fn gender(bit: &str) -> WordGender {
    let nlines = entry(&bit);

    let k = &mut 0;

    for _x in 0..nlines.len() {
        if nlines[_x].contains("<span class=\"gender\">") {
            *k = _x;
            break;
        }
    }
    let gen_line = &nlines[*k];
    
    if *k == 0 {
        return Ungendered;
    }

    let l = &gen_line.find("<span class=\"gender\">").unwrap() + "<span class=\"gender\">".len();
    let k = &gen_line.find("</span>").unwrap();
    
    let ba = gen_line[l..*k].to_string();

    let last_digs = 9; //">f</abbr>".len()
    return match &ba[(ba.len() - last_digs)..] {
        ">f</abbr>" => Feminine,
        ">n</abbr>" => Neuter,
        "im</abbr>" => MasculineAnim,
        "an</abbr>" => MasculineInam,
        "rs</abbr>" => MasculinePers,
        &_ => Ungendered
    };
}

fn class(bit: &String) -> WordType {
    let nlines = entry(&bit);

    let class = &mut TypeError;

    for i in 0..nlines.len() {
        let (v,n,a) = ("Verb</span>", "Noun</span>", "Adjective</span>");
        if nlines[i].contains(v) || nlines[i].contains(n) || nlines[i].contains(a) {
            if nlines[i].contains(v) {
                *class = Verb;
            }
            if nlines[i].contains(n) {
                *class = Noun;
            }
            if nlines[i].contains(a) {
                *class = Adjective;
            }
        }
    }
    return class.clone();
}

pub fn process(wrd: &str) -> Vec<(String, String)> {
    let url_data = raw_html(&wrd);

    let gender = gender(&url_data);
    let class = class(&url_data);

    println!("word: {}", wrd);
    println!("\tgender: {:?}", gender);
    println!("\tclass: {:?}", class);

    let x = table(&url_data);
    let frm = wrd_dupe_filter(format(find_links(x, class)));


    if (&frm).len() == 0 {
        println!("\tAll forms of {} exist!", &wrd);
    } else {
        println!("\texisting pages: {}/13", 13-(&frm).len());
        println!("\tpages to create: {:?}", &frm);
    }
    return frm;
}