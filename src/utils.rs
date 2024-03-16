const SIBASE: f64 = 1000f64;
const IECBASE: f64 = 1024f64;
const SIUNITS: [&'static str; 8] = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB"];
const IECUNITS: [&'static str; 8] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB"];

pub(crate) fn convert_to_si_units(value: u64) -> String {
    convert_units(value, SIBASE, SIUNITS)
}

pub(crate) fn convert_to_iec_units(value: u64) -> String {
    convert_units(value, IECBASE, IECUNITS)
}

fn convert_units(val: u64, base: f64, units: [&'static str; 8]) -> String {
    let mut size = val as f64;
    let mut unit_index = 0;
    while size >= base && unit_index < units.len() - 1 {
        size /= base;
        unit_index += 1;
    }
    let precision = if size < 10f64 {
        2
    } else if size < 100f64 {
        1
    } else {
        0
    };
    format!("{:.*}{}", precision, size, units[unit_index])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_si_units() {
        assert_eq!(convert_to_si_units(11914873073), "11.9GB");
        assert_eq!(convert_to_si_units(53248), "53.2KB");
        assert_eq!(convert_to_si_units(1024), "1.02KB");
        assert_eq!(convert_to_si_units(500), "500B");
        assert_eq!(convert_to_si_units(10), "10.0B");
    }

    #[test]
    fn test_convert_to_iec_units() {
        assert_eq!(convert_to_iec_units(11914873073), "11.1GiB");
        assert_eq!(convert_to_iec_units(53248), "52.0KiB");
        assert_eq!(convert_to_iec_units(1024), "1.00KiB");
        assert_eq!(convert_to_iec_units(500), "500B");
        assert_eq!(convert_to_iec_units(10), "10.0B");
    }
}
