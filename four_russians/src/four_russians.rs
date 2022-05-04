#[derive(Debug)]
pub struct Matrix<T, const M: usize, const N: usize> {
    pub rows: [[T; N]; M]
}

pub fn print_mat<T: std::fmt::Debug, const M: usize, const N: usize>(mat: Matrix<T,{M},{N}>) {
    for row in mat.rows {
        println!("{:?}", row);
    }
}

/* 
     This diagram summarises the method. We use "4 Russian" tables
     of width S bits (S=3 below); current block starts at column s;
     to find pivot for row r, we start at j=r and reduce to the
     left using pivots s .. r-1; then check whether row j has a
     pivot for row r -- if it does, (possibly) xor it onto row r.
     Once the block is complete, form the table of size 2^S and
     reduce below...
         +-----+--------------------------------+
         |1 * *|* * * * * * * * * * * * * * * * |
         |  1 *|* * * * * * * * * * * * * * * * |
         |    1|* * * * * * * * * * * * * * * * |
         +-----+-----+--------------------------+
     s-> |     |1 * *|* * * * * * * * * * * * * |
         |     |0 0 *|* * * * * * * * * * * * * | <- r
         |     |0 0 *|* * * * * * * * * * * * * |
         |     |* * *|* * * * * * * * * * * * * | <- j
         |     |* * *|* * * * * * * * * * * * * |
         |     |* * *|* * * * * * * * * * * * * |
         +--------------------------------------+
      On failure to find a pivot, break out for a simpler loop
      that finishes it off.


fn four_russians_mat_print(m: u32, n: u32, wds: u32, &[]) -> u32 {
    chunk_size = m.log2();


}
*/