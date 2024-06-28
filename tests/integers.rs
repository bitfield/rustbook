//! Given a list of integers, use a vector and return the median (when sorted,
//! the value in the middle position) and mode (the value that occurs most
//! often; a hash map will be helpful here) of the list.

use std::collections::HashMap;

fn median(nums: &[i32]) -> Option<f64> {
    if nums.is_empty() {
        return None;
    }
    let mut v = nums.to_vec();
    v.sort_unstable();
    if v.len() % 2 == 0 {
        let a = v[(v.len() / 2) - 1];
        let b = v[v.len() / 2];
        Some(f64::from(a + b) / 2.0)
    } else {
        Some(f64::from(v[v.len() / 2]))
    }
}

fn mode(nums: &[i32]) -> Option<i32> {
    let v = nums.to_vec();
    let mut freqs: HashMap<i32, usize> = HashMap::new();
    for n in &v {
        *freqs.entry(*n).or_insert(0) += 1;
    }
    let (mode, max) = freqs.iter().max_by_key(|(_, v)| *v)?;
    if freqs.iter().any(|(k, v)| v == max && k != mode) {
        None
    } else {
        Some(*mode)
    }
}

#[test]
fn test_median() {
    let cases: Vec<(&[i32], Option<f64>)> = vec![
        (&[1, 6, 2, 3, 4, 5, 5, 5, 2], Some(4.0)),
        (&[7, 8, 9, 10], Some(8.5)),
        (&[11, 10, 9], Some(10.0)),
        (&[], None),
    ];

    for (input, want) in cases {
        let got = median(input);
        assert_eq!(want, got, "median({input:?}): want {want:?}, got {got:?}");
    }
}

#[test]
fn test_mode() {
    let cases: Vec<(&[i32], Option<i32>)> = vec![
        (&[1, 6, 2, 3, 4, 5, 5, 5, 2], Some(5)),
        (&[7, 8, 9, 10], None),
        (&[1, 1, 1], Some(1)),
        (&[-1, 0, 0, 1], Some(0)),
        (&[], None),
    ];

    for (input, want) in cases {
        let got = mode(input);
        assert_eq!(want, got, "mode({input:?}): want {want:?}, got {got:?}");
    }
}
