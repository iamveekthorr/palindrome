fn main() {
    let res = is_palindrome("race a car");
    println!("{res}")
}

fn is_palindrome(word: &str) -> bool {
    let mut start_index = 0;
    let mut end_index = word.len() - 1;

    while start_index < end_index {
        if word.as_bytes()[start_index] != word.as_bytes()[end_index] {
            return false;
        }

        start_index += 1;
        end_index -= 1;
    }

    return true;
}
