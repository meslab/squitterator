/// Retrieves the Downlink Format (DF) from a message.
///
/// # Arguments
///
/// * `message` - The message to extract the DF from.
///
/// # Returns
///
/// The Downlink Format (DF) value.
/// # Examples
///
/// ```
/// use squitterator::adsb::{message, df};
/// let squitter = "8D40621D58C382D690C8AC2863A7";
/// if let Some(message) = message(squitter) {
///     if let Some(df) = df(&message) {
///         assert_eq!(df, 17);
///     }
/// }
/// ```
pub fn df(message: &[u32]) -> Option<u32> {
    crate::adsb::range_value(message, 1, 5)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adsb::message;

    #[test]
    fn test_df_17() {
        let squitter = "8D40621D58C382D690C8AC2863A7";
        if let Some(message) = message(squitter) {
            let result = df(&message).unwrap_or(0);
            assert_eq!(result, 17);
        }
    }

    #[test]
    fn test_df_21() {
        let squitter = "A8281200200464B3CF7820CD194C";
        if let Some(message) = message(squitter) {
            let result = df(&message).unwrap_or(0);
            assert_eq!(result, 21);
        }
    }
}
