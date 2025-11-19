pub fn pairs(pair_list: Vec<String>) -> Vec<(String, String)> {
    let mut p: Vec<(String, String)> = Vec::new();

    let len = pair_list.len();
    for i in 0..len {
        println!("{} ", pair_list[i]);
        for j in i + 1..len {
            println!("{}", pair_list[j]);
            println!("{:?}", (pair_list[i].clone(), pair_list[j].clone()));

            p.push((pair_list[i].clone(), pair_list[j].clone()));
        }
    }
    p
}

#[test]
fn test_pairs() {
    let case_1: Vec<_> = vec![
        ("a".to_string(), "b".to_string()),
        ("a".to_string(), "c".to_string()),
        ("b".to_string(), "c".to_string()),
    ];

    assert_eq!(
        pairs(vec!["a".to_string(), "b".to_string(), "c".to_string()]),
        case_1
    );

    let case_2 = vec![
        ("a".to_string(), "b".to_string()),
        ("a".to_string(), "c".to_string()),
        ("a".to_string(), "d".to_string()),
        ("b".to_string(), "c".to_string()),
        ("b".to_string(), "d".to_string()),
        ("c".to_string(), "d".to_string()),
    ];

    assert_eq!(
        pairs(vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string()
        ]),
        case_2
    );

    let case_3 = vec![
        ("cherry".to_string(), "cranberry".to_string()),
        ("cherry".to_string(), "banana".to_string()),
        ("cherry".to_string(), "blueberry".to_string()),
        ("cherry".to_string(), "lime".to_string()),
        ("cherry".to_string(), "papaya".to_string()),
        ("cranberry".to_string(), "banana".to_string()),
        ("cranberry".to_string(), "blueberry".to_string()),
        ("cranberry".to_string(), "lime".to_string()),
        ("cranberry".to_string(), "papaya".to_string()),
        ("banana".to_string(), "blueberry".to_string()),
        ("banana".to_string(), "lime".to_string()),
        ("banana".to_string(), "papaya".to_string()),
        ("blueberry".to_string(), "lime".to_string()),
        ("blueberry".to_string(), "papaya".to_string()),
        ("lime".to_string(), "papaya".to_string()),
    ];

    assert_eq!(
        pairs(vec![
            "cherry".to_string(),
            "cranberry".to_string(),
            "banana".to_string(),
            "blueberry".to_string(),
            "lime".to_string(),
            "papaya".to_string()
        ]),
        case_3
    );
}
