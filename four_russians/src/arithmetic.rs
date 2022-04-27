/// Calculates the dot product of two slices of equal length.
///
/// # Examples
///
/// ```
/// let dot1 = dot(&[1,2], &[3,4])
/// let dot2 = dot(&vec![1,2], &vec![3,4])
/// assert_eq!(11, dot1)
/// assert_eq!(dot1, dot2)
/// ```

fn dot(a: &[u64], b: &[u64]) -> u64 {
    assert_eq!(a.len(), b.len());
    let mut product = 0;
    for i in 0..a.len() {
        product += a[i] * b[i]
    }
    product
}

#[test]
fn dot_test() {
    let dot1 = dot(&[1,2], &[3,4]);
    let dot2 = dot(&vec![1,2], &vec![3,4]);
    assert_eq!(11, dot1);
    assert_eq!(dot1, dot2);
}

fn dot_gf2(a: &[u64], b: &[u64]) -> u64 {
    assert_eq!(a.len(), b.len());
    let mut product = 0;
    for i in 0..a.len() {
        product ^= a[i] & b[i]
    }
    product
}

#[test]
fn dotgf2_test() {
    let dot1 = dot_gf2(&[1,2], &[3,4]);
    let dot2 = dot_gf2(&vec![1,2], &vec![3,4]);
    assert_eq!(1, dot1);
    assert_eq!(dot1, dot2);
}