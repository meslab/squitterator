mod dfs;
mod extended;
mod mode_s;
mod short;

pub use dfs::*;
pub(crate) use extended::*;
pub(crate) use mode_s::*;
pub(crate) use short::*;

use crate::decoder::range_value;

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
/// use squitterator::decoder::{message, df};
/// let squitter = "8D40621D58C382D690C8AC2863A7";
/// if let Some(message) = message(squitter) {
///     if let Some(df) = df(&message) {
///         assert_eq!(df, 17);
///     }
/// }
/// ```
pub fn df(message: &[u32]) -> Option<u32> {
    range_value(message, 1, 5)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decoder::message;

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

    #[test]
    fn test_df_22() {
        let squitter = "A020100A10020A80F000004F24AF";
        if let Some(message) = message(squitter) {
            let result = df(&message).unwrap_or(0);
            assert_eq!(result, 20);
        }
    }
}
