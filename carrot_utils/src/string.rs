use colored::*;

/// Search substring for string.
///
/// Example
/// string: "abcefghijklmn"
/// start_string: "efg"
/// end_char_list: ["m"]
/// -> return "hijkl"
pub fn search_substring(
    string: &str,
    start_string: &str,
    end_char_list: Vec<char>,
) -> Option<String> {
    if !string.contains(&start_string) {
        return None;
    }

    let chars: Vec<char> = string.chars().collect();
    let search_char: Vec<char> = start_string.chars().collect();

    for i in 0..chars.len() {
        if i + search_char.len() < chars.len() {
            let substring = &chars[i..(i + search_char.len())];
            if substring == search_char {
                for j in (i + search_char.len())..chars.len() {
                    if end_char_list.contains(&chars[j]) {
                        return Some(chars[(i + search_char.len())..j].iter().collect());
                    }
                }
            }
        }
    }
    None
}

/// Get colored string for checking.
/// This function is useful for CLI tools.
/// # Arguments
/// - source: source string
/// - target: target string
/// # Return
/// ColoredString
/// if source == target -> "OK" with green color
/// if source != target -> "NG" with red color
pub fn get_check_result_string(source: &str, target: &str) -> ColoredString {
    if source == target {
        "OK".green()
    } else {
        "NG".red()
    }
}
