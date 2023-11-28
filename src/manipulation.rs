use std::convert::TryInto;

use util::raw_html;
use util::find_line;
use util::str_split;
use util::str_cut;
use util::par_cont;
use util::Word;
use util::WordClass;
use util::WordClass::*;
use util::WordGender;
use util::WordGender::*;

fn entry(input: &str) -> Vec<String> {
    let lines = str_split(&input.to_string(), "\n");
    
    let start: usize = find_line(&lines, "Polish</span><span class=\"mw-editsection\">").try_into().unwrap();
    let pol_ent = &lines[start..lines.len()];

    return pol_ent.to_vec();       
}

/// Narrows down the input block into just the inflection table data.
fn table(k: &Vec<String>) -> Vec<String> {
    let tbl_a_indx = find_line(&k, "class=\"inflection-table").try_into().unwrap();

    let g = k[tbl_a_indx..].to_vec();
    let tbl_b_indx = find_line(&g, "</table>").try_into().unwrap();

    return g[0..tbl_b_indx].to_vec();
}

fn gender(raw_html: &Vec<String>) -> WordGender {
    let k = &mut 0;

    for _x in 0..raw_html.len() {
        if raw_html[_x].contains("<span class=\"gender\">") {
            *k = _x;
            break;
        }
    }
    let gen_line = &raw_html[*k];
    
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

fn class(raw_html: &Vec<String>) -> WordClass {
    let class = &mut TypeError;

    for i in 0..raw_html.len() {
        let pairs = vec![("Verb</span>", Verb), ("Noun</span>", Noun), ("Adjective</span>", Adjective)];

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

    let (v_1_sg_pres, v_1_pl_pres, v_1_sg_m_past, v_1_sg_f_past, v_1_pl_mp_past, v_1_pl_nv_past) = (34, 36, 62, 64, 68, 70);
    let (v_1_sg_m_cnd, v_1_pl_m_cnd, v_1_sg_f_cnd, v_1_pl_nv_cnd) = (154, 160, 156, 162);

    let (v_2_sg_pres, v_2_pl_pres, v_2_sg_m_past, v_2_sg_f_past, v_2_pl_mp_past, v_2_pl_nv_past) = (41, 43, 75, 77, 81, 83);
    let (v_2_sg_m_cnd, v_2_pl_m_cnd, v_2_sg_f_cnd, v_2_pl_nv_cnd) = (167,173,169,175);

    let (v_3_sg_pres, v_3_pl_pres, v_3_sg_m_past, v_3_sg_f_past, v_3_pl_mp_past, v_3_pl_nv_past) = (48, 50, 88, 90, 94, 96);
    let (v_3_sg_m_cnd, v_3_pl_m_cnd, v_3_sg_f_cnd, v_3_pl_nv_cnd) = (180, 186, 182, 188);

    let (v_pl_imp, v_2_sg_imp, v_2_pl_imp, v_act_adj_par) = (200, 207, 209, 221);

    let (v_ctp_adv_par, v_noun) = (234, 239);

    let (nv_ctp_adv_par, nv_noun) = (247, 252);

    let nv_pass_adj_par = 234;

    let v_slice = &mut vec![("v_1_sg_pres", v_1_sg_pres), ("v_1_pl_pres", v_1_pl_pres), ("v_1_sg_m_past", v_1_sg_m_past), ("v_1_sg_f_past", v_1_sg_f_past), ("v_1_pl_mp_past", v_1_pl_mp_past), ("v_1_pl_nv_past", v_1_pl_nv_past),
        ("v_2_sg_pres", v_2_sg_pres), ("v_2_pl_pres", v_2_pl_pres), ("v_2_sg_m_past", v_2_sg_m_past), ("v_2_sg_f_past", v_2_sg_f_past), ("v_2_pl_mp_past", v_2_pl_mp_past), ("v_2_pl_nv_past", v_2_pl_nv_past),
        ("v_3_sg_pres", v_3_sg_pres), ("v_3_pl_pres", v_3_pl_pres), ("v_3_sg_m_past", v_3_sg_m_past), ("v_3_sg_f_past", v_3_sg_f_past), ("v_3_pl_mp_past", v_3_pl_mp_past), ("v_3_pl_nv_past", v_3_pl_nv_past),
        ];

    let nv_slice = &mut v_slice.clone();
    nv_slice.push(("nv_ctp_adv_par", nv_ctp_adv_par));
    nv_slice.push(("nv_noun", nv_noun));
    nv_slice.push(("nv_pass_adj_par", nv_pass_adj_par));

    let lv_slice = &mut v_slice.clone();
    lv_slice.push(("v_ctp_adv_par", v_ctp_adv_par));
    lv_slice.push(("v_noun", v_noun));
    
    for pair in [("v_1_sg_m_cnd", v_1_sg_m_cnd), ("v_1_pl_m_cnd", v_1_pl_m_cnd), ("v_1_sg_f_cnd", v_1_sg_f_cnd), ("v_1_pl_nv_cnd", v_1_pl_nv_cnd), 
                 ("v_2_sg_m_cnd", v_2_sg_m_cnd), ("v_2_pl_m_cnd", v_2_pl_m_cnd), ("v_2_sg_f_cnd", v_2_sg_f_cnd), ("v_2_pl_nv_cnd", v_2_pl_nv_cnd),
                 ("v_3_sg_m_cnd", v_3_sg_m_cnd), ("v_3_pl_m_cnd", v_3_pl_m_cnd), ("v_3_sg_f_cnd", v_3_sg_f_cnd), ("v_3_pl_nv_cnd", v_3_pl_nv_cnd),
                 ("v_pl_imp", v_pl_imp), ("v_2_sg_imp", v_2_sg_imp), ("v_2_pl_imp", v_2_pl_imp), ("v_act_adj_par", v_act_adj_par)] {
        lv_slice.push(pair);
        nv_slice.push(pair);
    }

    let pv_slice = &mut v_slice.clone();
    pv_slice.push(("v_pass_adj_par", 175));
    pv_slice.push(("v_ant_adv_par", 188));
    pv_slice.push(("v_noun", 193));
    let p: &mut Vec<(&str, i32)> = &mut Vec::new();

    match wrd_type {
        &Noun => *p = n_slice.to_vec(),
        &Adjective => *p = a_slice.to_vec(),
        &Verb => 
            if bit.len() > 240 {
                *p = nv_slice.to_vec();
            } else {
                if bit[2].contains("imperfective") {
                    *p = lv_slice.to_vec();                    
                } else {
                    *p = pv_slice.to_vec();       
                }
            }
        &_ => *p = n_slice.to_vec(),
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
    
    return Word {word, inflected_words, gender, class };
}

fn gen_pg(pg_pairs: &Vec<(String, String)>) -> Vec<(String, String)> {
    let pg_list = &mut Vec::new();

    for pair in pg_pairs {
        let (k,_v) = pair;

        let mut page_markup: String = String::new();

        page_markup.push_str("==Polish==");
        page_markup.push_str("\n");
        page_markup.push_str("===Pronunciation===");
        page_markup.push_str("{{pl-p}}");
        page_markup.push_str("\n");

        let l = str_cut(&k, 0, 3);
        match l.as_str() {
            "nom" | "gen" | "dat" | "acc" | "ins" | "loc" | "voc" => { //noun
                page_markup.push_str("===Noun===");
                page_markup.push_str("{{{{head|pl|noun|g=n}}}}");
            },
            _ => println!("IDK! {}", l),
        }
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
    gen_pg(&prep_pair.inflected_words);

    return prep_pair;
}