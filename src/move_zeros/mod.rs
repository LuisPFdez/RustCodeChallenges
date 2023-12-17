#![allow(dead_code)]
fn move_zeros(arr: &[u8]) -> Vec<u8> {
    let mut new_vector: Vec<u8> = Vec::new();
    let mut num_of_zeros: i32 = 0;

    for element in arr  {
        if *element == 0{
            num_of_zeros+=1;
            continue;
        }

        new_vector.push(*element);
    }

    for _ in 0..num_of_zeros{
        new_vector.push(0);
    }

    return new_vector;
}

//Second implementation with one loop
fn move_zeros_2(arr: &[u8]) -> Vec<u8> {
    let mut new_vector: Vec<u8> = Vec::new();
    let mut num_of_zeros: usize = 0;
    
    
    for (index,element) in arr.iter().enumerate()  {
        if *element == 0{
            new_vector.push(0);
            num_of_zeros += 1;
        } else {
            new_vector.insert(index - num_of_zeros, *element);
        }
    }

    new_vector
}

#[cfg(test)]
mod tests {
    use super::move_zeros;
    
    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
    
    fn dotest(a: &[u8], expected: &[u8]) {
        let actual = move_zeros(a);
        assert!(actual == expected, "With arr = {a:?}\nExpected {expected:?} but got {actual:?}")   
    }
    
    #[test]
    fn sample_tests() {
        dotest(&[1, 2, 0, 1, 0, 1, 0, 3, 0, 1], &[1, 2, 1, 1, 3, 1, 0, 0, 0, 0]);
        dotest(&[9, 0, 0, 9, 1, 2, 0, 1, 0, 1, 0, 3, 0, 1, 9, 0, 0, 0, 0, 9], &[9, 9, 1, 2, 1, 1, 3, 1, 9, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        dotest(&[0, 0], &[0, 0]);
        dotest(&[0], &[0]);
        dotest(&[], &[]);
    }
}