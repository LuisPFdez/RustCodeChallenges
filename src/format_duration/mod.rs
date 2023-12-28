#![allow(dead_code)]
fn format_duration(seconds: u64) -> String {
    if seconds == 0 {
        return String::from("now");
    }

    let mut result = String::new();

    let operations: [(u16, (&str, &str)); 4] = [
        (60, ("second", "seconds")),
        (60, ("minute", "minutes")),
        (24, ("hour", "hours")),
        (365, ("day", "days")),
    ];
    let last_name = ("year", "years");
    let mut current_value: u64 = seconds;

    let mut remainder: u64;

    for (divider, name) in operations {
        remainder = current_value % divider as u64;
        current_value = current_value / divider as u64;

        if remainder > 0 {
            if result.len() != 0 {
                result.insert_str(0, ", ");
            }

            if remainder == 1 {
                result.insert_str(0, &format!("{} {}", remainder, name.0))
            } else {
                result.insert_str(0, &format!("{} {}", remainder, name.1))
            }
        }
    }

    if current_value > 0 {
        if result.len() != 0 {
            result.insert_str(0, ", ");
        }

        if current_value == 1 {
            result.insert_str(0, &format!("{} {}", current_value, last_name.0))
        } else {
            result.insert_str(0, &format!("{} {}", current_value, last_name.1))
        }
    }

    return match result.rfind(",") {
        Some(index) => {
            result.replace_range(index..index + 1, " and");
            result
        }
        None => result,
    };
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::format_duration;

    #[test]
    fn test_basic() {
        assert_eq!(format_duration(1), "1 second");
        assert_eq!(format_duration(62), "1 minute and 2 seconds");
        assert_eq!(format_duration(120), "2 minutes");
        assert_eq!(format_duration(3600), "1 hour");
        assert_eq!(format_duration(3662), "1 hour, 1 minute and 2 seconds");
    }
}
