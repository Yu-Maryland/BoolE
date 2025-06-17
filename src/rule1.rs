use egraph_mapping::{SimpleLanguage};
use egg::*;

pub fn make_rules1_map() -> Vec<Rewrite<SimpleLanguage, ()>> {
    let mut rules=vec![
        rewrite!("Commutative"; "(and ?op1 ?op2)" => "(and ?op2 ?op1)"),
        
        rewrite!("Negation_1"; "(not Bool_False)" => "Bool_True"),
        rewrite!("Negation_2"; "(not Bool_True)" => "Bool_False"),

        rewrite!("Idempotency"; "(and ?op ?op)" => "?op"),

        rewrite!("Complements_1"; "(and Bool_True ?op)" => "?op"),
        rewrite!("Complements_2"; "(and Bool_False ?op)" => "Bool_False"),
        rewrite!("Complements_3"; "(and (not ?op) ?op)" => "Bool_False"),
        
        rewrite!("Complements_4"; "(and ?op Bool_True)" => "?op"),
        rewrite!("Complements_5"; "(and ?op Bool_False)" => "Bool_False"),
        rewrite!("Complements_6"; "(and ?op (not ?op))" => "Bool_False"),

        rewrite!("Covering_1"; "(and ?op1 (not (and (not ?op1) (not ?op2))))" => "?op1"),
        rewrite!("Covering_2"; "(and ?op1 (not (and (not ?op2) (not ?op1))))" => "?op1"),
        rewrite!("Covering_3"; "(and (not (and (not ?op1) (not ?op2))) ?op1)" => "?op1"),
        rewrite!("Covering_4"; "(and (not (and (not ?op2) (not ?op1))) ?op1)" => "?op1"),
    ];
    rules.extend(vec![
        rewrite!("Double Negation"; "(not (not ?op))" <=> "?op"),
        // rewrite!("Associativity"; "(and (and ?op1 ?op2) ?op3)" <=> "(and ?op1 (and ?op2 ?op3))"),
    ].concat());

    rules
}

pub fn make_rules1() -> Vec<Rewrite<SimpleLanguage, ()>> {

    let mut rules=vec![
        rewrite!("De Morgan"; "(not (and ?op1 ?op2))" => "(or (not ?op1) (not ?op2))"),
        // rewrite!("Double Negation"; "(not (not ?op))" => "?op"),
        // rewrite!("De Morgan"; "(not (and ?op1 ?op2))" => "(or (not ?op1) (not ?op2))"),
        // rewrite!("inner_rule_18_1"; "(or (or ?op1 ?op2) ?op3)" => "(or ?op1 ?op2 ?op3)"),
        // rewrite!("inner_rule_20_1"; "(and ?op1 ?op2 ?op3)" => "(and (and ?op1 ?op2) ?op3)"),
        // rewrite!("inner_rule_20_2";"(and ?op1 (and ?op2 ?op3))" => "(and ?op1 ?op2 ?op3)"),
    ];

    rules

}
