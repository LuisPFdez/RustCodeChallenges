#![allow(dead_code)]
fn remove_smallest(numbers: &[u32]) -> Vec<u32> {
    let mut new_vector: Vec<u32> = Vec::new();
    if numbers.len() >= 1 {
        let mut min_index: (u32, usize) = match numbers.iter().next() {
            Some(num) => (*num, 0),
            None => (0, 0),
        };

        for (index, number) in numbers.iter().enumerate() {
            if *number < min_index.0 {
                min_index.0 = *number;
                min_index.1 = index;
            }
        }

        new_vector = numbers.to_vec();
        new_vector.remove(min_index.1);

        return new_vector;
    } else {
        return new_vector;
    }
}

#[cfg(test)]
mod tests {
    use super::remove_smallest;
    
    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
    
    fn dotest(a: &[u32], expected: &[u32]) {
        assert_eq!(remove_smallest(a), expected, "{ERR_MSG} with numbers = {a:?}")
    }

    #[test]
    fn fixed_tests() {
        dotest(&[1, 2, 3, 4, 5], &[2, 3, 4, 5]);
        dotest(&[1, 2, 3, 4], &[2, 3, 4]);
        dotest(&[5, 3, 2, 1, 4], &[5, 3, 2, 4]);
        dotest(&[1, 2, 3, 1, 1], &[2, 3, 1, 1]);
    }
}