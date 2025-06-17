#![recursion_limit = "256"]

pub use egg::*;


define_language! {
    #[derive(serde::Serialize)]
    pub enum SimpleLanguage {
        Symbol(Symbol),
        "out" = OUT(Vec<Id>),
        "dff" = DFF(Vec<Id>),
        "nor" = NOR(Vec<Id>),
        "and" = AND(Vec<Id>), 
        "nand" = NAND(Vec<Id>),
        "not" = NOT(Vec<Id>), 
        "or" = OR(Vec<Id>),
        "xor" = XOR(Vec<Id>),
        "xor3" = XOR3(Vec<Id>),
        "buff" = BUFF(Vec<Id>),
        "maj" = MAJ(Vec<Id>),
        "majI" = MAJI(Vec<Id>),
        "fa" = FA(Vec<Id>),
        "fst" = FST(Vec<Id>),
        "snd" = SND(Vec<Id>),
    }
}

pub fn convert_to_simple_language_enum(in_id: Vec<Id>, stripped_name: &str) -> Option<SimpleLanguage> {
    let language_enum = match stripped_name {
        "out" => {Some(SimpleLanguage::OUT(in_id))},
        "dff" => {Some(SimpleLanguage::DFF(in_id))},
        "nor" => {Some(SimpleLanguage::NOR(in_id))},
        "and" => {Some(SimpleLanguage::AND(in_id))},
        "nand" => {Some(SimpleLanguage::NAND(in_id))},
        "not" => {Some(SimpleLanguage::NOT(in_id))},
        "or" => {Some(SimpleLanguage::OR(in_id))},
        "xor" => {Some(SimpleLanguage::XOR(in_id))},
        "xor3" => {Some(SimpleLanguage::XOR3(in_id))},
        "buff" => {Some(SimpleLanguage::BUFF(in_id))},
        "maj" => {Some(SimpleLanguage::MAJ(in_id))},
        "majI" => {Some(SimpleLanguage::MAJI(in_id))},
        "fa" => {Some(SimpleLanguage::FA(in_id))},
        "fst" => {Some(SimpleLanguage::FST(in_id))},
        "snd" => {Some(SimpleLanguage::SND(in_id))},
        _ => None,
    };
    language_enum
}
pub fn get_node_type(enode: &SimpleLanguage) -> &str {
    match enode {
        SimpleLanguage::OUT(_) => "out",
        SimpleLanguage::DFF(_) => "dff",
        SimpleLanguage::NOR(_) => "nor",
        SimpleLanguage::AND(_) => "and",
        SimpleLanguage::NAND(_) => "nand",
        SimpleLanguage::NOT(_) => "not",
        SimpleLanguage::OR(_) => "or",
        SimpleLanguage::XOR(_) => "xor",
        SimpleLanguage::XOR3(_) => "xor3",
        SimpleLanguage::BUFF(_) => "buff",
        SimpleLanguage::MAJ(_) => "maj",
        SimpleLanguage::MAJI(_) => "majI",
        SimpleLanguage::FA(_) => "fa",
        SimpleLanguage::FST(_) => "fst",
        SimpleLanguage::SND(_) => "snd",
        _ => "",
    }
}
pub fn get_input_list(cell:String) -> Vec<&'static str> {
    match cell.as_str() {
        _ =>{
            panic!("Invalid input for {}", cell);
        }
    }
}
