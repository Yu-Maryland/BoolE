use egraph_mapping::{SimpleLanguage};
use egg::*;

pub fn make_empty_rules() -> Vec<Rewrite<SimpleLanguage, ()>> {
    let mut rules=vec![
    ];
    rules
}

pub fn make_rules2_map() -> Vec<Rewrite<SimpleLanguage, ()>> {

    let mut rules=vec![
        rewrite!("Associativity"; "(and (and ?op1 ?op2) ?op3)" => "(and ?op1 (and ?op2 ?op3))"),
    ];

    rules

}

pub fn make_rules2() -> Vec<Rewrite<SimpleLanguage, ()>> {

    let mut rules=vec![
        // rewrite!("De Morgan"; "(not (and ?op1 ?op2))" => "(or (not ?op1) (not ?op2))"),
        rewrite!("Double Negation"; "(not (not ?op))" => "?op"),
    ];

    rules

}

pub fn make_rules3_map() -> Vec<Rewrite<SimpleLanguage, ()>> {

    let mut rules=vec![
        rewrite!("MAJ3_11"; "(and (not (and ?op2 ?op3)) (not (and ?op1 (and (not (and ?op2 ?op3)) (not (and (not ?op2) (not ?op3)))))))" => "(not (maj ?op1 ?op2 ?op3))"),
        rewrite!("CUSTOM_RULE"; "(and (not (and ?op1 (not (and ?op2 (not ?op3))))) (not (and (not ?op2) ?op3)))" => "(maj (not ?op1) ?op2 (not ?op3))"),
        rewrite!("CUSTOM_RULE"; "(and (not (and ?op1 ?op2)) (not (and (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2)))) (not ?op3))))" => "(maj (not ?op1) (not ?op2) ?op3)"),
        rewrite!("CUSTOM_RULE"; "(and (not (and ?op1 ?op2)) (not (and (not (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2)))) ?op3)))" => "(maj (not ?op1) (not ?op2) (not ?op3))"),

        rewrite!("XOR3_7"; "(and (not (and (not ?op1) (and (not (and ?op2 ?op3)) (not (and (not ?op2) (not ?op3)))))) (not (and ?op1 (not (and (not (and ?op2 ?op3)) (not (and (not ?op2) (not ?op3))))))))" => "(xor3 (not ?op1) ?op2 ?op3)"),
        rewrite!("CUSTOM_RULE"; "(and (not (and (not (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2))))) ?op3)) (not (and (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2)))) (not ?op3))))" => "(xor3 (not ?op1) ?op2 ?op3)"),
        rewrite!("CUSTOM_RULE"; "(and (not (and (not (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2))))) ?op3)) (not (and (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2)))) (not ?op3))))" => "(xor3 (not ?op1) (not ?op2) (not ?op3))"),
        // rewrite!("CUSTOM_RULE"; "(and (not (and ?op1 (and (not (and (not ?op2) (not ?op3))) (not (and ?op2 ?op3))))) (not (and (not ?op1) (not (and (not (and (not ?op2) (not ?op3))) (not (and ?op2 ?op3)))))))" => "(xor3 ?op1 (not ?op2) (not ?op3))"),
        // rewrite!("Associativity"; "(and (and ?op1 ?op2) ?op3)" => "(and ?op1 (and ?op2 ?op3))"),
        // rewrite!("XOR3"; "(xor3 (not ?op1) ?op2 ?op3)" => "(not (xor3 ?op1 ?op2 ?op3))"),
        // rewrite!("XOR3"; "(xor3 ?op1 (not ?op2) ?op3)" => "(not (xor3 ?op1 ?op2 ?op3))"),
        // rewrite!("XOR3"; "(xor3 ?op1 ?op2 (not ?op3))" => "(not (xor3 ?op1 ?op2 ?op3))"),


    ];
    // rules.extend(vec![
    //     rewrite!("XOR3"; "(xor3 (not ?op1) ?op2 ?op3)" <=> "(not (xor3 ?op1 ?op2 ?op3))"),
    //     rewrite!("XOR3"; "(xor3 ?op1 (not ?op2) ?op3)" <=> "(not (xor3 ?op1 ?op2 ?op3))"),
    //     rewrite!("XOR3"; "(xor3 ?op1 ?op2 (not ?op3))" <=> "(not (xor3 ?op1 ?op2 ?op3))"),
    //     // rewrite!("XOR3"; "(xor3 (not ?op1) ?op2 (not ?op3))" <=> "(xor3 ?op1 ?op2 ?op3)"),
    //     // rewrite!("XOR3"; "(xor3 ?op1 (not ?op2) (not ?op3))" <=> "(xor3 ?op1 ?op2 ?op3)"),
    //     // rewrite!("XOR3"; "(xor3 (not ?op1) (not ?op2) ?op3)" <=> "(xor3 ?op1 ?op2 ?op3)"),
    // ].concat());

    rules

}

pub fn make_rules3() -> Vec<Rewrite<SimpleLanguage, ()>> {

    let mut rules=vec![
        rewrite!("MAJ3_11"; "(and (not (and ?op2 ?op3)) (not (and ?op1 (and (not (and ?op2 ?op3)) (not (and (not ?op2) (not ?op3)))))))" => "(not (maj ?op1 ?op2 ?op3))"),
        rewrite!("XOR3_7"; "(and (not (and (not ?op1) (and (not (and ?op2 ?op3)) (not (and (not ?op2) (not ?op3)))))) (not (and ?op1 (not (and (not (and ?op2 ?op3)) (not (and (not ?op2) (not ?op3))))))))" => "(xor3 (not ?op1) ?op2 ?op3)"),
    ];

    rules

}

// pub fn make_rules3() -> Vec<Rewrite<SimpleLanguage, ()>> {

//     let mut rules=vec![
//         rewrite!("MAJ3"; "(and (not (and ?op2 ?op3)) (not (and ?op1 (and (not (and ?op2 ?op3)) (not (and (not ?op2) (not ?op3)))))))" => "(not (maj ?op1 ?op2 ?op3))"),
//         rewrite!("XOR3"; "(and (not (and (not ?op1) (and (not (and ?op2 ?op3)) (not (and (not ?op2) (not ?op3)))))) (not (and ?op1 (not (and (not (and ?op2 ?op3)) (not (and (not ?op2) (not ?op3))))))))" => "(xor3 (not ?op1) ?op2 ?op3)"),

//         rewrite!("CUSTOM_RULE"; "(and (and (not (and (not ?op1) (not ?op2))) (not (and (not ?op2) (not ?op3)))) (not (and (not ?op1) (not ?op3))))" => "(maj ?op1 ?op2 ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (and (not (and (not ?op1) (not ?op2))) (not (and (not ?op2) ?op3))) (not (and (not ?op1) ?op3)))" => "(maj ?op1 ?op2 (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (and (not (and (not ?op1) ?op2)) (not (and ?op2 (not ?op3)))) (not (and (not ?op1) (not ?op3))))" => "(maj ?op1 (not ?op2) ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (and (not (and (not ?op1) ?op2)) (not (and ?op2 ?op3))) (not (and (not ?op1) ?op3)))" => "(maj ?op1 (not ?op2) (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (and (not (and ?op1 (not ?op2))) (not (and (not ?op2) (not ?op3)))) (not (and ?op1 (not ?op3))))" => "(maj (not ?op1) ?op2 ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (and (not (and ?op1 (not ?op2))) (not (and (not ?op2) ?op3))) (not (and ?op1 ?op3)))" => "(maj (not ?op1) ?op2 (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (and (not (and ?op1 ?op2)) (not (and ?op2 (not ?op3)))) (not (and ?op1 (not ?op3))))" => "(maj (not ?op1) (not ?op2) ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (and (not (and ?op1 ?op2)) (not (and ?op2 ?op3))) (not (and ?op1 ?op3)))" => "(maj (not ?op1) (not ?op2) (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not (and (not (and (not ?op1) (not ?op2))) (not (and ?op1 ?op2)))) (not ?op3))) (not (and ?op1 (not ?op2))))" => "(maj (not ?op1) ?op2 ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2)))) (not ?op3))) (not (and ?op1 ?op2)))" => "(maj (not ?op1) (not ?op2) ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2)))) ?op3)) (not (and ?op1 ?op2)))" => "(maj (not ?op1) (not ?op2) (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2))))) (not ?op3))) (not (and ?op1 (not ?op2))))" => "(maj (not ?op1) ?op2 ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not ?op1) (not (and (not ?op2) (not ?op3))))) (not (and ?op2 ?op3)))" => "(maj ?op1 (not ?op2) (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not ?op1) (not (and (not ?op2) ?op3)))) (not (and ?op2 (not ?op3))))" => "(maj ?op1 (not ?op2) ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not ?op1) (not ?op3))) (not (and (not ?op2) (not (and ?op1 ?op3)))))" => "(maj ?op1 ?op2 ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not ?op2) (not (and ?op1 (not ?op3))))) (not (and (not ?op1) ?op3)))" => "(maj ?op1 ?op2 (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not ?op2) (not ?op3))) (not (and (not ?op1) (not (and ?op2 ?op3)))))" => "(maj ?op1 ?op2 ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not ?op2) (not ?op3))) (not (and ?op1 (not (and ?op2 ?op3)))))" => "(maj (not ?op1) ?op2 ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not ?op2) ?op3)) (not (and (not ?op1) (not (and ?op2 (not ?op3))))))" => "(maj ?op1 ?op2 (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op1 (not (and (not ?op2) (not ?op3))))) (not (and ?op2 ?op3)))" => "(maj (not ?op1) (not ?op2) (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op1 (not (and (not ?op2) ?op3)))) (not (and ?op2 (not ?op3))))" => "(maj (not ?op1) (not ?op2) ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op1 (not (and ?op2 (not ?op3))))) (not (and (not ?op2) ?op3)))" => "(maj (not ?op1) ?op2 (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op1 (not (and ?op2 ?op3)))) (not (and (not ?op2) (not ?op3))))" => "(maj (not ?op1) ?op2 ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op1 (not ?op2))) (not (and (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2)))) (not ?op3))))" => "(maj (not ?op1) ?op2 ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op1 (not ?op2))) (not (and (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2)))) ?op3)))" => "(maj (not ?op1) ?op2 (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op1 (not ?op2))) (not (and (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2))) (not ?op3))))" => "(maj (not ?op1) ?op2 ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op1 (not ?op2))) (not (and (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2))) ?op3)))" => "(maj (not ?op1) ?op2 (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op1 (not ?op2))) (not (and (not (and (not (and (not ?op1) (not ?op2))) (not (and ?op1 ?op2)))) (not ?op3))))" => "(maj (not ?op1) ?op2 ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op1 (not ?op2))) (not (and (not (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2))))) (not ?op3))))" => "(maj (not ?op1) ?op2 ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op1 (not ?op2))) (not (and (not (and (not ?op1) ?op2)) (not ?op3))))" => "(maj (not ?op1) ?op2 ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op1 (not ?op2))) (not (and (not ?op3) (not (and (not ?op1) ?op2)))))" => "(maj (not ?op1) ?op2 ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op1 ?op2)) (not (and (and (not (and (not ?op1) (not ?op2))) (not (and ?op1 ?op2))) (not ?op3))))" => "(maj (not ?op1) (not ?op2) ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op1 ?op2)) (not (and (and (not (and (not ?op1) (not ?op2))) (not (and ?op1 ?op2))) ?op3)))" => "(maj (not ?op1) (not ?op2) (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op1 ?op2)) (not (and (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2)))) (not ?op3))))" => "(maj (not ?op1) (not ?op2) ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op1 ?op2)) (not (and (not (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2)))) (not ?op3))))" => "(maj (not ?op1) (not ?op2) ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op1 ?op2)) (not (and (not (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2)))) ?op3)))" => "(maj (not ?op1) (not ?op2) (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op1 ?op2)) (not (and (not (and (not ?op1) (not ?op2))) (not ?op3))))" => "(maj (not ?op1) (not ?op2) ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op1 ?op2)) (not (and (not ?op3) (not (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2)))))))" => "(maj (not ?op1) (not ?op2) ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op1 ?op2)) (not (and (not ?op3) (not (and (not ?op1) (not ?op2))))))" => "(maj (not ?op1) (not ?op2) ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op1 ?op3)) (not (and (not ?op2) (and (not (and ?op1 ?op3)) (not (and (not ?op1) (not ?op3)))))))" => "(maj (not ?op1) ?op2 (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op2 (not (and (not ?op1) (not ?op3))))) (not (and ?op1 ?op3)))" => "(maj (not ?op1) (not ?op2) (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op2 (not ?op3))) (not (and (not ?op1) (not (and (not ?op2) ?op3)))))" => "(maj ?op1 (not ?op2) ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op2 (not ?op3))) (not (and ?op1 (not (and (not ?op2) ?op3)))))" => "(maj (not ?op1) (not ?op2) ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op2 ?op3)) (not (and (not ?op1) (not (and (not ?op2) (not ?op3))))))" => "(maj ?op1 (not ?op2) (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op2 ?op3)) (not (and ?op1 (and (not (and ?op2 ?op3)) (not (and (not ?op2) (not ?op3)))))))" => "(maj (not ?op1) (not ?op2) (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op2 ?op3)) (not (and ?op1 (not (and (not ?op2) (not ?op3))))))" => "(maj (not ?op1) (not ?op2) (not ?op3))"),

//         rewrite!("CUSTOM_RULE"; "(and (not (and (and (not (and (not ?op1) (not ?op2))) (not (and ?op1 ?op2))) (not ?op3))) (not (and (not (and (not (and (not ?op1) (not ?op2))) (not (and ?op1 ?op2)))) ?op3)))" => "(xor3 (not ?op1) (not ?op2) (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (and (not (and (not ?op1) (not ?op2))) (not (and ?op1 ?op2))) (not ?op3))) (not (and (not (and (not (and (not ?op1) (not ?op2))) (not (and ?op1 ?op2)))) ?op3)))" => "(xor3 (not ?op1) ?op2 ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (and (not (and (not ?op1) (not ?op2))) (not (and ?op1 ?op2))) (not ?op3))) (not (and (not (and (not (and (not ?op1) (not ?op2))) (not (and ?op1 ?op2)))) ?op3)))" => "(xor3 ?op1 (not ?op2) ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (and (not (and (not ?op1) (not ?op2))) (not (and ?op1 ?op2))) (not ?op3))) (not (and (not (and (not (and (not ?op1) (not ?op2))) (not (and ?op1 ?op2)))) ?op3)))" => "(xor3 ?op1 ?op2 (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (and (not (and (not ?op1) (not ?op2))) (not (and ?op1 ?op2))) ?op3)) (not (and (not (and (not (and (not ?op1) (not ?op2))) (not (and ?op1 ?op2)))) (not ?op3))))" => "(xor3 (not ?op1) (not ?op2) ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (and (not (and (not ?op1) (not ?op2))) (not (and ?op1 ?op2))) ?op3)) (not (and (not (and (not (and (not ?op1) (not ?op2))) (not (and ?op1 ?op2)))) (not ?op3))))" => "(xor3 (not ?op1) ?op2 (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (and (not (and (not ?op1) (not ?op2))) (not (and ?op1 ?op2))) ?op3)) (not (and (not (and (not (and (not ?op1) (not ?op2))) (not (and ?op1 ?op2)))) (not ?op3))))" => "(xor3 ?op1 (not ?op2) (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (and (not (and (not ?op1) (not ?op2))) (not (and ?op1 ?op2))) ?op3)) (not (and (not (and (not (and (not ?op1) (not ?op2))) (not (and ?op1 ?op2)))) (not ?op3))))" => "(xor3 ?op1 ?op2 ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2)))) (not ?op3))) (not (and (not (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2))))) ?op3)))" => "(xor3 (not ?op1) (not ?op2) ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2)))) (not ?op3))) (not (and (not (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2))))) ?op3)))" => "(xor3 (not ?op1) ?op2 (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2)))) (not ?op3))) (not (and (not (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2))))) ?op3)))" => "(xor3 ?op1 (not ?op2) (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2)))) (not ?op3))) (not (and (not (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2))))) ?op3)))" => "(xor3 ?op1 ?op2 ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2)))) ?op3)) (not (and (not (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2))))) (not ?op3))))" => "(xor3 (not ?op1) (not ?op2) (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2)))) ?op3)) (not (and (not (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2))))) (not ?op3))))" => "(xor3 (not ?op1) ?op2 ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2)))) ?op3)) (not (and (not (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2))))) (not ?op3))))" => "(xor3 ?op1 (not ?op2) ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2)))) ?op3)) (not (and (not (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2))))) (not ?op3))))" => "(xor3 ?op1 ?op2 (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2))) (not ?op3))) (not (and (not (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2)))) ?op3)))" => "(xor3 (not ?op1) (not ?op2) ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2))) (not ?op3))) (not (and (not (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2)))) ?op3)))" => "(xor3 (not ?op1) ?op2 (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2))) (not ?op3))) (not (and (not (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2)))) ?op3)))" => "(xor3 ?op1 (not ?op2) (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2))) (not ?op3))) (not (and (not (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2)))) ?op3)))" => "(xor3 ?op1 ?op2 ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2))) ?op3)) (not (and (not (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2)))) (not ?op3))))" => "(xor3 (not ?op1) (not ?op2) (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2))) ?op3)) (not (and (not (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2)))) (not ?op3))))" => "(xor3 (not ?op1) ?op2 ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2))) ?op3)) (not (and (not (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2)))) (not ?op3))))" => "(xor3 ?op1 (not ?op2) ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2))) ?op3)) (not (and (not (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2)))) (not ?op3))))" => "(xor3 ?op1 ?op2 (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2)))) (not ?op3))) (not (and (not (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2))))) ?op3)))" => "(xor3 (not ?op1) (not ?op2) (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2)))) (not ?op3))) (not (and (not (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2))))) ?op3)))" => "(xor3 (not ?op1) ?op2 ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2)))) (not ?op3))) (not (and (not (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2))))) ?op3)))" => "(xor3 ?op1 (not ?op2) ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2)))) (not ?op3))) (not (and (not (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2))))) ?op3)))" => "(xor3 ?op1 ?op2 (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2)))) ?op3)) (not (and (not (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2))))) (not ?op3))))" => "(xor3 (not ?op1) (not ?op2) ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2)))) ?op3)) (not (and (not (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2))))) (not ?op3))))" => "(xor3 (not ?op1) ?op2 (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2)))) ?op3)) (not (and (not (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2))))) (not ?op3))))" => "(xor3 ?op1 (not ?op2) (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2)))) ?op3)) (not (and (not (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2))))) (not ?op3))))" => "(xor3 ?op1 ?op2 ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not (and (not (and (not ?op1) (not ?op2))) (not (and ?op1 ?op2)))) (not ?op3))) (not (and (and (not (and (not ?op1) (not ?op2))) (not (and ?op1 ?op2))) ?op3)))" => "(xor3 (not ?op1) (not ?op2) ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not (and (not (and (not ?op1) (not ?op2))) (not (and ?op1 ?op2)))) (not ?op3))) (not (and (and (not (and (not ?op1) (not ?op2))) (not (and ?op1 ?op2))) ?op3)))" => "(xor3 (not ?op1) ?op2 (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not (and (not (and (not ?op1) (not ?op2))) (not (and ?op1 ?op2)))) (not ?op3))) (not (and (and (not (and (not ?op1) (not ?op2))) (not (and ?op1 ?op2))) ?op3)))" => "(xor3 ?op1 (not ?op2) (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not (and (not (and (not ?op1) (not ?op2))) (not (and ?op1 ?op2)))) (not ?op3))) (not (and (and (not (and (not ?op1) (not ?op2))) (not (and ?op1 ?op2))) ?op3)))" => "(xor3 ?op1 ?op2 ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not (and (not (and (not ?op1) (not ?op2))) (not (and ?op1 ?op2)))) ?op3)) (not (and (and (not (and (not ?op1) (not ?op2))) (not (and ?op1 ?op2))) (not ?op3))))" => "(xor3 (not ?op1) (not ?op2) (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not (and (not (and (not ?op1) (not ?op2))) (not (and ?op1 ?op2)))) ?op3)) (not (and (and (not (and (not ?op1) (not ?op2))) (not (and ?op1 ?op2))) (not ?op3))))" => "(xor3 (not ?op1) ?op2 ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not (and (not (and (not ?op1) (not ?op2))) (not (and ?op1 ?op2)))) ?op3)) (not (and (and (not (and (not ?op1) (not ?op2))) (not (and ?op1 ?op2))) (not ?op3))))" => "(xor3 ?op1 (not ?op2) ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not (and (not (and (not ?op1) (not ?op2))) (not (and ?op1 ?op2)))) ?op3)) (not (and (and (not (and (not ?op1) (not ?op2))) (not (and ?op1 ?op2))) (not ?op3))))" => "(xor3 ?op1 ?op2 (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2))))) (not ?op3))) (not (and (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2)))) ?op3)))" => "(xor3 (not ?op1) (not ?op2) (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2))))) (not ?op3))) (not (and (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2)))) ?op3)))" => "(xor3 (not ?op1) ?op2 ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2))))) (not ?op3))) (not (and (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2)))) ?op3)))" => "(xor3 ?op1 (not ?op2) ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2))))) (not ?op3))) (not (and (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2)))) ?op3)))" => "(xor3 ?op1 ?op2 (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2))))) ?op3)) (not (and (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2)))) (not ?op3))))" => "(xor3 (not ?op1) (not ?op2) ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2))))) ?op3)) (not (and (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2)))) (not ?op3))))" => "(xor3 (not ?op1) ?op2 (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2))))) ?op3)) (not (and (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2)))) (not ?op3))))" => "(xor3 ?op1 (not ?op2) (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2))))) ?op3)) (not (and (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2)))) (not ?op3))))" => "(xor3 ?op1 ?op2 ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2)))) (not ?op3))) (not (and (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2))) ?op3)))" => "(xor3 (not ?op1) (not ?op2) (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2)))) (not ?op3))) (not (and (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2))) ?op3)))" => "(xor3 (not ?op1) ?op2 ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2)))) (not ?op3))) (not (and (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2))) ?op3)))" => "(xor3 ?op1 (not ?op2) ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2)))) (not ?op3))) (not (and (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2))) ?op3)))" => "(xor3 ?op1 ?op2 (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2)))) ?op3)) (not (and (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2))) (not ?op3))))" => "(xor3 (not ?op1) (not ?op2) ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2)))) ?op3)) (not (and (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2))) (not ?op3))))" => "(xor3 (not ?op1) ?op2 (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2)))) ?op3)) (not (and (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2))) (not ?op3))))" => "(xor3 ?op1 (not ?op2) (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2)))) ?op3)) (not (and (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2))) (not ?op3))))" => "(xor3 ?op1 ?op2 ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2))))) (not ?op3))) (not (and (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2)))) ?op3)))" => "(xor3 (not ?op1) (not ?op2) ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2))))) (not ?op3))) (not (and (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2)))) ?op3)))" => "(xor3 (not ?op1) ?op2 (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2))))) (not ?op3))) (not (and (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2)))) ?op3)))" => "(xor3 ?op1 (not ?op2) (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2))))) (not ?op3))) (not (and (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2)))) ?op3)))" => "(xor3 ?op1 ?op2 ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2))))) ?op3)) (not (and (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2)))) (not ?op3))))" => "(xor3 (not ?op1) (not ?op2) (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2))))) ?op3)) (not (and (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2)))) (not ?op3))))" => "(xor3 (not ?op1) ?op2 ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2))))) ?op3)) (not (and (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2)))) (not ?op3))))" => "(xor3 ?op1 (not ?op2) ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2))))) ?op3)) (not (and (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2)))) (not ?op3))))" => "(xor3 ?op1 ?op2 (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not ?op1) (and (not (and (not ?op2) (not ?op3))) (not (and ?op2 ?op3))))) (not (and ?op1 (not (and (not (and (not ?op2) (not ?op3))) (not (and ?op2 ?op3)))))))" => "(xor3 (not ?op1) (not ?op2) (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not ?op1) (and (not (and (not ?op2) (not ?op3))) (not (and ?op2 ?op3))))) (not (and ?op1 (not (and (not (and (not ?op2) (not ?op3))) (not (and ?op2 ?op3)))))))" => "(xor3 (not ?op1) ?op2 ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not ?op1) (and (not (and (not ?op2) (not ?op3))) (not (and ?op2 ?op3))))) (not (and ?op1 (not (and (not (and (not ?op2) (not ?op3))) (not (and ?op2 ?op3)))))))" => "(xor3 ?op1 (not ?op2) ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not ?op1) (and (not (and (not ?op2) (not ?op3))) (not (and ?op2 ?op3))))) (not (and ?op1 (not (and (not (and (not ?op2) (not ?op3))) (not (and ?op2 ?op3)))))))" => "(xor3 ?op1 ?op2 (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not ?op1) (not (and (not (and (not ?op2) (not ?op3))) (not (and ?op2 ?op3)))))) (not (and ?op1 (and (not (and (not ?op2) (not ?op3))) (not (and ?op2 ?op3))))))" => "(xor3 (not ?op1) (not ?op2) ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not ?op1) (not (and (not (and (not ?op2) (not ?op3))) (not (and ?op2 ?op3)))))) (not (and ?op1 (and (not (and (not ?op2) (not ?op3))) (not (and ?op2 ?op3))))))" => "(xor3 (not ?op1) ?op2 (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not ?op1) (not (and (not (and (not ?op2) (not ?op3))) (not (and ?op2 ?op3)))))) (not (and ?op1 (and (not (and (not ?op2) (not ?op3))) (not (and ?op2 ?op3))))))" => "(xor3 ?op1 (not ?op2) (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not ?op1) (not (and (not (and (not ?op2) (not ?op3))) (not (and ?op2 ?op3)))))) (not (and ?op1 (and (not (and (not ?op2) (not ?op3))) (not (and ?op2 ?op3))))))" => "(xor3 ?op1 ?op2 ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not ?op2) (and (not (and ?op1 ?op3)) (not (and (not ?op1) (not ?op3)))))) (not (and ?op2 (not (and (not (and ?op1 ?op3)) (not (and (not ?op1) (not ?op3))))))))" => "(xor3 (not ?op1) (not ?op2) (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not ?op2) (and (not (and ?op1 ?op3)) (not (and (not ?op1) (not ?op3)))))) (not (and ?op2 (not (and (not (and ?op1 ?op3)) (not (and (not ?op1) (not ?op3))))))))" => "(xor3 (not ?op1) ?op2 ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not ?op2) (and (not (and ?op1 ?op3)) (not (and (not ?op1) (not ?op3)))))) (not (and ?op2 (not (and (not (and ?op1 ?op3)) (not (and (not ?op1) (not ?op3))))))))" => "(xor3 ?op1 (not ?op2) ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not ?op2) (and (not (and ?op1 ?op3)) (not (and (not ?op1) (not ?op3)))))) (not (and ?op2 (not (and (not (and ?op1 ?op3)) (not (and (not ?op1) (not ?op3))))))))" => "(xor3 ?op1 ?op2 (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not ?op3) (not (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2))))))) (not (and ?op3 (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2)))))))" => "(xor3 (not ?op1) (not ?op2) (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not ?op3) (not (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2))))))) (not (and ?op3 (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2)))))))" => "(xor3 (not ?op1) ?op2 ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not ?op3) (not (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2))))))) (not (and ?op3 (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2)))))))" => "(xor3 ?op1 (not ?op2) ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not ?op3) (not (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2))))))) (not (and ?op3 (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2)))))))" => "(xor3 ?op1 ?op2 (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not ?op3) (not (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2)))))) (not (and ?op3 (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2))))))" => "(xor3 (not ?op1) (not ?op2) (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not ?op3) (not (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2)))))) (not (and ?op3 (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2))))))" => "(xor3 (not ?op1) ?op2 ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not ?op3) (not (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2)))))) (not (and ?op3 (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2))))))" => "(xor3 ?op1 (not ?op2) ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not ?op3) (not (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2)))))) (not (and ?op3 (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2))))))" => "(xor3 ?op1 ?op2 (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not ?op3) (not (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2))))))) (not (and ?op3 (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2)))))))" => "(xor3 (not ?op1) (not ?op2) ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not ?op3) (not (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2))))))) (not (and ?op3 (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2)))))))" => "(xor3 (not ?op1) ?op2 (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not ?op3) (not (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2))))))) (not (and ?op3 (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2)))))))" => "(xor3 ?op1 (not ?op2) (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and (not ?op3) (not (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2))))))) (not (and ?op3 (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2)))))))" => "(xor3 ?op1 ?op2 ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op1 (and (not (and (not ?op2) (not ?op3))) (not (and ?op2 ?op3))))) (not (and (not ?op1) (not (and (not (and (not ?op2) (not ?op3))) (not (and ?op2 ?op3)))))))" => "(xor3 (not ?op1) (not ?op2) ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op1 (and (not (and (not ?op2) (not ?op3))) (not (and ?op2 ?op3))))) (not (and (not ?op1) (not (and (not (and (not ?op2) (not ?op3))) (not (and ?op2 ?op3)))))))" => "(xor3 (not ?op1) ?op2 (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op1 (and (not (and (not ?op2) (not ?op3))) (not (and ?op2 ?op3))))) (not (and (not ?op1) (not (and (not (and (not ?op2) (not ?op3))) (not (and ?op2 ?op3)))))))" => "(xor3 ?op1 (not ?op2) (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op1 (and (not (and (not ?op2) (not ?op3))) (not (and ?op2 ?op3))))) (not (and (not ?op1) (not (and (not (and (not ?op2) (not ?op3))) (not (and ?op2 ?op3)))))))" => "(xor3 ?op1 ?op2 ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op1 (and (not (and ?op2 (not ?op3))) (not (and (not ?op2) ?op3))))) (not (and (not ?op1) (not (and (not (and ?op2 (not ?op3))) (not (and (not ?op2) ?op3)))))))" => "(xor3 (not ?op1) (not ?op2) (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op1 (and (not (and ?op2 (not ?op3))) (not (and (not ?op2) ?op3))))) (not (and (not ?op1) (not (and (not (and ?op2 (not ?op3))) (not (and (not ?op2) ?op3)))))))" => "(xor3 (not ?op1) ?op2 ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op1 (and (not (and ?op2 (not ?op3))) (not (and (not ?op2) ?op3))))) (not (and (not ?op1) (not (and (not (and ?op2 (not ?op3))) (not (and (not ?op2) ?op3)))))))" => "(xor3 ?op1 (not ?op2) ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op1 (and (not (and ?op2 (not ?op3))) (not (and (not ?op2) ?op3))))) (not (and (not ?op1) (not (and (not (and ?op2 (not ?op3))) (not (and (not ?op2) ?op3)))))))" => "(xor3 ?op1 ?op2 (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op1 (and (not (and ?op2 ?op3)) (not (and (not ?op2) (not ?op3)))))) (not (and (not ?op1) (not (and (not (and ?op2 ?op3)) (not (and (not ?op2) (not ?op3))))))))" => "(xor3 (not ?op1) (not ?op2) ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op1 (and (not (and ?op2 ?op3)) (not (and (not ?op2) (not ?op3)))))) (not (and (not ?op1) (not (and (not (and ?op2 ?op3)) (not (and (not ?op2) (not ?op3))))))))" => "(xor3 (not ?op1) ?op2 (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op1 (and (not (and ?op2 ?op3)) (not (and (not ?op2) (not ?op3)))))) (not (and (not ?op1) (not (and (not (and ?op2 ?op3)) (not (and (not ?op2) (not ?op3))))))))" => "(xor3 ?op1 (not ?op2) (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op1 (and (not (and ?op2 ?op3)) (not (and (not ?op2) (not ?op3)))))) (not (and (not ?op1) (not (and (not (and ?op2 ?op3)) (not (and (not ?op2) (not ?op3))))))))" => "(xor3 ?op1 ?op2 ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op3 (not (and (not (and (not ?op1) (not ?op2))) (not (and ?op1 ?op2)))))) (not (and (not ?op3) (and (not (and (not ?op1) (not ?op2))) (not (and ?op1 ?op2))))))" => "(xor3 (not ?op1) (not ?op2) (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op3 (not (and (not (and (not ?op1) (not ?op2))) (not (and ?op1 ?op2)))))) (not (and (not ?op3) (and (not (and (not ?op1) (not ?op2))) (not (and ?op1 ?op2))))))" => "(xor3 (not ?op1) ?op2 ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op3 (not (and (not (and (not ?op1) (not ?op2))) (not (and ?op1 ?op2)))))) (not (and (not ?op3) (and (not (and (not ?op1) (not ?op2))) (not (and ?op1 ?op2))))))" => "(xor3 ?op1 (not ?op2) ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op3 (not (and (not (and (not ?op1) (not ?op2))) (not (and ?op1 ?op2)))))) (not (and (not ?op3) (and (not (and (not ?op1) (not ?op2))) (not (and ?op1 ?op2))))))" => "(xor3 ?op1 ?op2 (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op3 (not (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2))))))) (not (and (not ?op3) (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2)))))))" => "(xor3 (not ?op1) (not ?op2) ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op3 (not (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2))))))) (not (and (not ?op3) (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2)))))))" => "(xor3 (not ?op1) ?op2 (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op3 (not (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2))))))) (not (and (not ?op3) (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2)))))))" => "(xor3 ?op1 (not ?op2) (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op3 (not (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2))))))) (not (and (not ?op3) (and (not (and (not ?op1) ?op2)) (not (and ?op1 (not ?op2)))))))" => "(xor3 ?op1 ?op2 ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op3 (not (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2)))))) (not (and (not ?op3) (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2))))))" => "(xor3 (not ?op1) (not ?op2) ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op3 (not (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2)))))) (not (and (not ?op3) (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2))))))" => "(xor3 (not ?op1) ?op2 (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op3 (not (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2)))))) (not (and (not ?op3) (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2))))))" => "(xor3 ?op1 (not ?op2) (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op3 (not (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2)))))) (not (and (not ?op3) (and (not (and ?op1 (not ?op2))) (not (and (not ?op1) ?op2))))))" => "(xor3 ?op1 ?op2 ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op3 (not (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2))))))) (not (and (not ?op3) (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2)))))))" => "(xor3 (not ?op1) (not ?op2) (not ?op3))"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op3 (not (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2))))))) (not (and (not ?op3) (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2)))))))" => "(xor3 (not ?op1) ?op2 ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op3 (not (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2))))))) (not (and (not ?op3) (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2)))))))" => "(xor3 ?op1 (not ?op2) ?op3)"),
//         rewrite!("CUSTOM_RULE"; "(and (not (and ?op3 (not (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2))))))) (not (and (not ?op3) (and (not (and ?op1 ?op2)) (not (and (not ?op1) (not ?op2)))))))" => "(xor3 ?op1 ?op2 (not ?op3))"),

//     ];

//     rules

// }