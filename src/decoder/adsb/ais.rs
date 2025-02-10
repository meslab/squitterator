/// Converts an IA5 character code to a `char`.
///
/// # Arguments
///
/// * `ch` - The IA5 character code.
///
/// # Returns
///
/// The corresponding `char` value.
fn ia5(ch: u32) -> char {
    match ch {
        48..=57 => char::from_u32(ch).unwrap_or(' '),
        1..=26 => char::from_u32(ch | 64).unwrap_or(' '),
        _ => ' ',
    }
}

/// Extracts AIS (Aeronautical Information Service) data from a message.
///
/// # Arguments
///
/// * `message` - The message to extract AIS data from.
///
/// # Returns
///
/// An `Option` containing the AIS data as a `String`, or `None` if the message does not contain AIS data.
pub(crate) fn ais(message: &[u32]) -> Option<String> {
    Some(
        [
            ((message[10] << 2) | (message[11] >> 2)),
            (((message[11] & 3) << 4) | message[12]),
            ((message[13] << 2) | (message[14] >> 2)),
            (((message[14] & 3) << 4) | message[15]),
            ((message[16] << 2) | (message[17] >> 2)),
            (((message[17] & 3) << 4) | message[18]),
            ((message[19] << 2) | (message[20] >> 2)),
            (((message[20] & 3) << 4) | message[21]),
        ]
        .iter()
        .map(|&c| ia5(c))
        .filter(|&c| c != ' ')
        .collect::<String>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decoder::get_message;

    #[test]
    fn test_ais() {
        let squitter = "8DAAAA9225041331DF3820CAC7A4";
        if let Some(message) = get_message(squitter) {
            if let Some(result) = ais(&message) {
                assert_eq!(result, "AAL173");
            }
        }
    }

    #[test]
    fn test_ais_batch() {
        let squitters = [
            ("8D406F7C250815F2CB4560C85DCA", "BAW224U"),
            ("8DA1CB0523282573D74820E76A8D", "JBU354"),
        ];

        for (squitter, value) in squitters.iter() {
            if let Some(message) = get_message(squitter) {
                let result = ais(&message);
                match result {
                    Some(r) => assert_eq!(r.as_str(), *value, "{}", squitter),
                    None => assert_eq!(result, None),
                }
            }
        }
    }
}
