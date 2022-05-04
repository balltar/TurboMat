type Word  = usize;
type Index = usize;

/// Calculates the dot product of two slices of equal length 
/// over ZZ (integers).
///
/// # Examples
///
/// ```
/// let dot1 = dot(&[1,2], &[3,4])
/// let dot2 = dot(&vec![1,2], &vec![3,4])
/// assert_eq!(11, dot1)
/// assert_eq!(dot1, dot2)
/// ```

fn dot(a: &[Word], b: &[Word]) -> Word {
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

/// Calculates the dot product of two slices of equal length 
/// over ZZ / 2ZZ (bits).
///
/// # Examples
///
/// ```
/// let dot1 = dot(&[1,2], &[3,4])
/// let dot2 = dot(&vec![1,2], &vec![3,4])
/// assert_eq!(11, dot1)
/// assert_eq!(dot1, dot2)
/// ```

fn dot_gf2(a: &[Word], b: &[Word]) -> u32 {
    assert_eq!(a.len(), b.len());
    let mut product: Word = 0;
    for i in 0..a.len() {
        product ^= a[i] & b[i]
    }
    product.count_ones() % 2
}

#[test]
fn dotgf2_test() {
    let dot1 = dot_gf2(&[1,2], &[3,3]);
    let dot2 = dot_gf2(&vec![1,2], &vec![3,3]);
    assert_eq!(0, dot1);
    assert_eq!(dot1, dot2);
}

/// Calculates the dot product of two slices a and b 
/// of equal length over ZZ / 2ZZ with the knowledge
/// that the indices of the nonzero words are in 
/// the half open interval [w0, w1)
///
/// # Examples
///
/// ```
/// let dot1 = trunc_dot_gf2(&[7,1,2], &[7,3,3], 1, 2);
/// let dot2 = trunc_dot_gf2(&vec![7,1,2], &vec![7,3,3], 1, 2);
/// assert_eq!(0, dot1);
/// assert_eq!(dot1, dot2);
/// ```
#[inline]
fn trunc_dot_gf2(a: &[Word], b: &[Word], i0: Index, i1: Index) -> u32 {
    assert_eq!(a.len(), b.len());
    let mut product: Word = 0;
    for i in i0..i1 {
        product ^= a[i] & b[i]
    }
    product.count_ones() % 2
}

#[test]
fn trunc_dot_gf2_test() {
    let dot1 = trunc_dot_gf2(&[7,1,2], &[7,3,3], 1, 2);
    let dot2 = trunc_dot_gf2(&vec![7,1,2], &vec![7,3,3], 1, 3);
    assert_eq!(1, dot1);
    assert_eq!(0, dot2);
}

/// Adds two rows of a bit matrix together with
/// the knowledge that the indices of the nonzero bits
/// are in the half open interval [s, wds).
/// A row is represented as a list of words.
///
/// # Examples
///
/// ```
/// let row0 = &mut [0, 0, 0];
/// let row7 = &[7, 7, 7];
/// let row3 = &[3, 3, 3];
/// add_rows(row0, row7, row3, 1, 3);
/// assert_eq!(row0, &[0,4,4])
/// ```
#[inline]
fn add_rows(a: &mut [Word], b: &[Word], c: &[Word], i0: Index, i1: Index) {
    for i in i0..i1 {
        a[i] = b[i] ^ c[i]
    }
}

#[test]
fn add_rows_test() {
    let row0 = &mut [0, 0, 0];
    let row7 = &[7, 7, 7];
    let row3 = &[3, 3, 3];
    add_rows(row0, row7, row3, 1, 3);
    assert_eq!(row0, &[0,4,4])
}

