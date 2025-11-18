#[derive(Debug, PartialEq)]
pub enum Item {
    N(usize),
    S(String),
}

pub fn fizz_buzz(n: usize) -> Vec<Item> {
    let mut results: Vec<Item> = vec![];

    for i in 1..=n {
        if i % 5 == 0 && i % 3 == 0 {
            results.push(Item::S("fizzbuzz".to_string()));
        } else if i % 5 == 0 {
            results.push(Item::S("buzz".to_string()));
        } else if i % 3 == 0 {
            results.push(Item::S("fizz".to_string()));
        } else {
            results.push(Item::N(i));
        }
    }
    results
}
#[test]
fn test_fizz_buzz() {
    let case_1: Vec<Item> = vec![
        Item::N(1),
        Item::N(2),
        Item::S(String::from("fizz")),
        Item::N(4),
        Item::S(String::from("buzz")),
        Item::S(String::from("fizz")),
        Item::N(7),
        Item::N(8),
        Item::S(String::from("fizz")),
        Item::S(String::from("buzz")),
        Item::N(11),
    ];
    assert_eq!(fizz_buzz(11), case_1);

    let case_2: Vec<Item> = vec![Item::N(1), Item::N(2)];
    assert_eq!(fizz_buzz(2), case_2);

    let case_3: Vec<Item> = vec![
        Item::N(1),
        Item::N(2),
        Item::S(String::from("fizz")),
        Item::N(4),
        Item::S(String::from("buzz")),
        Item::S(String::from("fizz")),
        Item::N(7),
        Item::N(8),
        Item::S(String::from("fizz")),
        Item::S(String::from("buzz")),
        Item::N(11),
        Item::S(String::from("fizz")),
        Item::N(13),
        Item::N(14),
        Item::S(String::from("fizzbuzz")),
        Item::N(16),
    ];
    assert_eq!(fizz_buzz(16), case_3);
}
