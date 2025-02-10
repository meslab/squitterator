use log::debug;

use crate::decoder::{flag_and_range_value, get_hex_message};

pub(crate) fn mcp_selected_altitude(message: &[u32]) -> Option<u32> {
    debug!("MCP, {}", get_hex_message(message));
    flag_and_range_value(message, 33, 34, 45)
        .filter(|&f| f.0 == 1)
        .map(|v| v.1 << 4)
}

pub(crate) fn fms_selected_altitude(message: &[u32]) -> Option<u32> {
    flag_and_range_value(message, 46, 47, 58)
        .filter(|&f| f.0 == 1)
        .map(|v| v.1 << 4)
}

pub(crate) fn barometric_pressure_setting(message: &[u32]) -> Option<u32> {
    if let Some((status, value)) = flag_and_range_value(message, 59, 60, 71) {
        if status == 1 {
            Some(value / 10 + 800)
        } else {
            Some(value / 10)
        }
    } else {
        None
    }
}

pub(crate) fn target_altitude_source(message: &[u32]) -> Option<u32> {
    flag_and_range_value(message, 86, 87, 88)
        .filter(|&f| f.0 == 1)
        .map(|v| v.1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decoder::get_message;
    #[test]
    fn test_mcp_selected_altitude_value() {
        let s = [
            ("A80004BAACF6427180000078379E", 23008),
            ("A80004BAACF64270A800007814EC", 23008),
            ("A80004BAB8AE4270A80000823C66", 29008),
        ];
        for (squitter, value) in s.iter() {
            if let Some(message) = get_message(squitter) {
                assert_eq!(mcp_selected_altitude(&message), Some(*value as u32));
            }
        }
    }

    #[test]
    fn test_fsm_selected_altitude_value() {
        let s = [
            ("A80004BAACF6427180000078379E", 37008),
            ("A80004BAACF64270A800007814EC", 37008),
            ("A80004BAB8AE4270A80000823C66", 37008),
        ];
        for (squitter, value) in s.iter() {
            if let Some(message) = get_message(squitter) {
                assert_eq!(fms_selected_altitude(&message), Some(*value as u32));
            }
        }
    }
}
