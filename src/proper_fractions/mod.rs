#![allow(dead_code)]
//First implementation
fn proper_fractions(n: u64) -> u64 {
    if n == 1 {
        return 0;
    }

    let mut result = 1;

    'outer: for i in 2..=n {
        if n % i == 0 {
            continue;
        }

        for j in 2..=(i / 2) {
            if i % j == 0 && n % j == 0 {
                continue 'outer;
            }
        }

        result += 1;
    }

    result
}

//Euler’s Totient Function implementation in Rust. Funtion refactored from Java example of https://www.geeksforgeeks.org/eulers-totient-function/
fn phi_totient(mut n: u64) -> u64 {
    if n == 1 {
        return 0;
    }

    let mut result: f64 = n as f64;

    let mut p: u64 = 2;

    while p * p <= n {
        if n % p == 0 {
            while n % p == 0 {
                n /= p;
            }
            result *= 1.0 - (1.0 / p as f64);
        }
        p += 1;
    }

    if n > 1 {
        result -= result / n as f64;
    }

    return result as u64;
}

#[cfg(test)]
mod tests {
    use super::proper_fractions;
    use super::phi_totient;

    #[test]
    fn test_tonient() {
        assert_eq!(
            phi_totient(1),
            0,
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            phi_totient(2),
            1,
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            phi_totient(5),
            4,
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            phi_totient(15),
            8,
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            phi_totient(25),
            20,
            "\nYour answer (left) is not the expected answer (right)."
        );
    }

    #[test]
    fn sample_tests_proper_fractions() {
        assert_eq!(
            proper_fractions(1),
            0,
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            proper_fractions(2),
            1,
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            proper_fractions(5),
            4,
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            proper_fractions(15),
            8,
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            proper_fractions(25),
            20,
            "\nYour answer (left) is not the expected answer (right)."
        );

        println!("Valor {}", proper_fractions(200000))
    }
}
