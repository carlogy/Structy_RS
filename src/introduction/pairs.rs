pub fn pairs(pair_list: Vec<String>) -> Vec<(String, String)> {
    let capacity = { pair_list.len() * (pair_list.len() - 1) / 2 };
    let mut result: Vec<(String, String)> = Vec::with_capacity(capacity);
    for (i, a) in pair_list.iter().enumerate() {
        for b in pair_list.iter().skip(i + 1) {
            result.push((a.to_string(), b.to_string()));
        }
    }
    result
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
