use log::warn;

/// Clean the squitter from any non-hexadecimal characters
///
/// # Arguments
///
/// * `line` - A string slice that holds the squitter
///
/// # Returns
///
/// * `Option<Vec<u32>>` - A cleaned squitter
///
pub(crate) fn clean_squitter(line: &str) -> Option<Vec<u32>> {
    let trimmed_line: Vec<u32> = line
        .chars()
        .filter(|c| c.is_ascii_hexdigit())
        .map(|c| c.to_digit(16).unwrap())
        .collect();
    match trimmed_line.len() {
        14 | 28 => Some(trimmed_line),
        26 | 40 => Some(trimmed_line[12..].to_vec()),
        _ => {
            warn!("Invalid squitter: {}", line);
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_squitter() {
        let line = "009736E3E0B75D406890DB5905";
        // let format = None;
        if let Some(result) = clean_squitter(line) {
            assert_eq!(result, [5, 13, 4, 0, 6, 8, 9, 0, 13, 11, 5, 9, 0, 5]);
        }
    }

    #[test]
    fn test_sbs_squitter() {
        let line = "@009736E2736B8D40717EF82100020049B8A8887A;";
        if let Some(result) = clean_squitter(line) {
            assert_eq!(
                result,
                [
                    8, 13, 4, 0, 7, 1, 7, 14, 15, 8, 2, 1, 0, 0, 0, 2, 0, 0, 4, 9, 11, 8, 10, 8, 8,
                    8, 7, 10
                ]
            );
        }
    }
}
