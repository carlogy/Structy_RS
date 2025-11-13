fn longest_word(sentence: &str) -> String {
    let words = sentence.split(" ");
    let mut longest = String::new();

    words.into_iter().for_each(|x| {
        if x.len() >= longest.len() {
            longest = x.to_string()
        }
    });
    longest
}

#[test]
fn test_longest_word() {
    assert_eq!(longest_word("what a wonderful world"), "wonderful");
    assert_eq!(longest_word("have a nice day"), "nice");
    assert_eq!(
        longest_word("the quick brown fox jumped over the lazy dog"),
        "jumped"
    );
    assert_eq!(longest_word("who did eat the ham"), "ham");
    assert_eq!(longest_word("potato"), "potato");
}
