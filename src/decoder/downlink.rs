mod dfs;
mod extended;
mod mode_s;
mod short;

pub use dfs::*;
pub(crate) use extended::*;
pub(crate) use mode_s::*;
pub(crate) use short::*;

use crate::range_value;

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
/// use squitterator::{get_message, get_downlink_format};
/// let squitter = "8D40621D58C382D690C8AC2863A7";
/// if let Some(message) = get_message(squitter) {
///     if let Some(df) = get_downlink_format(&message) {
///         assert_eq!(df, 17);
///     }
/// }
/// ```
pub fn get_downlink_format(message: &[u32]) -> Option<u32> {
    range_value(message, 1, 5)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decoder::get_message;

    #[test]
    fn test_df_17() {
        let squitter = "8D40621D58C382D690C8AC2863A7";
        if let Some(message) = get_message(squitter) {
            let result = get_downlink_format(&message).unwrap_or(0);
            assert_eq!(result, 17);
        }
    }

    #[test]
    fn test_df_21() {
        let squitter = "A8281200200464B3CF7820CD194C";
        if let Some(message) = get_message(squitter) {
            let result = get_downlink_format(&message).unwrap_or(0);
            assert_eq!(result, 21);
        }
    }

    #[test]
    fn test_df_22() {
        let squitter = "A020100A10020A80F000004F24AF";
        if let Some(message) = get_message(squitter) {
            let result = get_downlink_format(&message).unwrap_or(0);
            assert_eq!(result, 20);
        }
    }
}
