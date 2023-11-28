pub const HTML_PL_HEADER: &str = "Polish</span><span class=\"mw-editsection\">";
pub const HTML_INF_TBL: &str = "class=\"inflection-table";
pub const HTML_GENDER: &str = "<span class=\"gender\">";

pub const HTML_ID_LEN: usize = 9;
pub const HTML_GND_FEM: &str = ">f</abbr>";
pub const HTML_GND_NEU: &str = ">n</abbr>";
pub const HTML_GND_M_A: &str = "im</abbr>";
pub const HTML_GND_M_I: &str = "an</abbr>";
pub const HTML_GND_M_P: &str = "rs</abbr>";

pub const HTML_CLASS_N: &str = "Noun</span>";
pub const HTML_CLASS_A: &str = "Adjective</span>";
pub const HTML_CLASS_V: &str = "Verb</span>";

pub const ID_PAIRS_NOUN: [(&str, i32); 13] = [("nom_pl", 14), ("gen_sg", 19), ("gen_pl", 21), ("dat_sg", 26), ("dat_pl", 28), ("acc_sg", 33), ("acc_pl", 35),  ("ins_sg", 40), ("ins_pl", 42), ("loc_sg", 47), ("loc_pl", 49), ("voc_sg", 54), ("voc_pl", 56)]; 
pub const ID_PAIRS_ADJC: [(&str, i32); 18] = [("nom_voc_n", 26), ("nom_voc_f", 28), ("nom_voc_v", 30), ("nom_voc_nv", 32), ("gen_mn", 37), ("gen_dat_f", 39), ("gen_pl", 41), ("dat_mn", 46), ("dat_pl", 48), ("acc_ma", 53), ("acc_n", 57), ("acc_ins_f", 59), ("acc_v", 61), ("acc_mv", 63), ("ins_loc_mn", 68), ("a_ins_pl", 70), ("loc_f", 75), ("loc_pl", 77)];

pub const ID_PAIRS_VB_FULL: [(&str, i32); 37] = [("v_1_sg_pres", 34), ("v_1_pl_pres", 36), ("v_1_sg_m_past", 62), ("v_1_sg_f_past", 64), ("v_1_pl_mp_past", 68), ("v_1_pl_nv_past", 70), ("v_2_sg_pres", 41), ("v_2_pl_pres", 43), ("v_2_sg_m_past", 75), ("v_2_sg_f_past", 77), ("v_2_pl_mp_past", 81), ("v_2_pl_nv_past", 83), ("v_3_sg_pres", 48), ("v_3_pl_pres", 50), ("v_3_sg_m_past", 88), ("v_3_sg_f_past", 90), ("v_3_pl_mp_past", 94), ("v_3_pl_nv_past", 96), ("nv_ctp_adv_par", 247), ("nv_noun", 252), ("nv_pass_adj_par", 234), ("v_1_sg_m_cnd", 154), ("v_1_pl_m_cnd", 160), ("v_1_sg_f_cnd", 156), ("v_1_pl_nv_cnd", 162), ("v_2_sg_m_cnd", 167), ("v_2_pl_m_cnd", 173), ("v_2_sg_f_cnd", 169), ("v_2_pl_nv_cnd", 175), ("v_3_sg_m_cnd", 180), ("v_3_pl_m_cnd", 186), ("v_3_sg_f_cnd", 182), ("v_3_pl_nv_cnd", 188), ("v_pl_imp", 200), ("v_2_sg_imp", 207), ("v_2_pl_imp", 209), ("v_act_adj_par", 221)];
pub const ID_PAIRS_VB_IMP: [(&str, i32); 36] = [("v_1_sg_pres", 34), ("v_1_pl_pres", 36), ("v_1_sg_m_past", 62), ("v_1_sg_f_past", 64), ("v_1_pl_mp_past", 68), ("v_1_pl_nv_past", 70), ("v_2_sg_pres", 41), ("v_2_pl_pres", 43), ("v_2_sg_m_past", 75), ("v_2_sg_f_past", 77), ("v_2_pl_mp_past", 81), ("v_2_pl_nv_past", 83), ("v_3_sg_pres", 48), ("v_3_pl_pres", 50), ("v_3_sg_m_past", 88), ("v_3_sg_f_past", 90), ("v_3_pl_mp_past", 94), ("v_3_pl_nv_past", 96), ("v_ctp_adv_par", 234), ("v_noun", 239), ("v_1_sg_m_cnd", 154), ("v_1_pl_m_cnd", 160), ("v_1_sg_f_cnd", 156), ("v_1_pl_nv_cnd", 162), ("v_2_sg_m_cnd", 167), ("v_2_pl_m_cnd", 173), ("v_2_sg_f_cnd", 169), ("v_2_pl_nv_cnd", 175), ("v_3_sg_m_cnd", 180), ("v_3_pl_m_cnd", 186), ("v_3_sg_f_cnd", 182), ("v_3_pl_nv_cnd", 188), ("v_pl_imp", 200), ("v_2_sg_imp", 207), ("v_2_pl_imp", 209), ("v_act_adj_par", 221)];
pub const ID_PAIRS_VB_PFT: [(&str, i32); 21] = [("v_1_sg_pres", 34), ("v_1_pl_pres", 36), ("v_1_sg_m_past", 62), ("v_1_sg_f_past", 64), ("v_1_pl_mp_past", 68), ("v_1_pl_nv_past", 70), ("v_2_sg_pres", 41), ("v_2_pl_pres", 43), ("v_2_sg_m_past", 75), ("v_2_sg_f_past", 77), ("v_2_pl_mp_past", 81), ("v_2_pl_nv_past", 83), ("v_3_sg_pres", 48), ("v_3_pl_pres", 50), ("v_3_sg_m_past", 88), ("v_3_sg_f_past", 90), ("v_3_pl_mp_past", 94), ("v_3_pl_nv_past", 96), ("v_pass_adj_par", 175), ("v_ant_adv_par", 188), ("v_noun", 193)];