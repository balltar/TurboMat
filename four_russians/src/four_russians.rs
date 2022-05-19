// Use usize for indexing and u32 for numbers...
// Complications really come up when you are doing arithmetic
// operations with your indices... seems bad.

use crate::arithmetic::Word;
use crate::arithmetic::Index;

pub fn print_type_of<T>(_: &T) { println!("{}", std::any::type_name::<T>()) }
pub fn ceil_div(x: Index, y: Index) -> Index { x / y + (x % y != 0) as Index }
pub fn ceil_rem(x: Index, y: Index) -> Index { if x % y != 0 {x % y} else {y} }

pub const fn log_2(x: usize) -> usize {
    assert!(x > 0);
    WORD_SIZE -  (x.leading_zeros() as usize) - 1
}

#[derive(Debug)]
pub struct Matrix<T, const M: usize, const N: usize> {
    pub rows: [[T; N]; M]
}

impl<T, const M: Index, const N: Index> Matrix<T, M, N> {
    pub fn num_rows(&self)   -> u32 { M.try_into().unwrap() }
    pub fn num_cols(&self)   -> u32 { N.try_into().unwrap() }
    pub fn dimensions(&self) -> (u32, u32) { (M.try_into().unwrap(), N.try_into().unwrap()) }
}

impl<const M: Index, const N: Index> Matrix<Word, M, N> {
    const N_words: usize = N * WORD_SIZE;

    fn add_rows(&mut self, r1: Index, r2: Index, i0: Index, i1: Index) {
        assert!(r1 < M && r2 < M);
        for i in i0..i1 {
            self.rows[r1][i] ^= self.rows[r2][i]
        }
    }

    fn update_xor_row(&mut self, r1: Index, row: &Row<N>, i0: Index, i1: Index) {
        assert!(r1 < M);
        for i in i0..i1 {
            self.rows[r1][i] ^= row[i]
        }
    }
}

type Row<const N: Index> = [Word; N];

pub trait Bit {
    fn bit(&self, i: Index) -> bool;
    fn bits(&self, start: Index, end: Index) -> Word;
}

trait UpdateRow<const N: Index> {
    fn update_add_rows(&mut self, r1: &Row<N>, r2: &Row<N>, i0: Index, i1: Index);
}

impl<const N: Index> Bit for Row<N> {
    /// Determine if a row has nonzero entry at a given bit index.
    /// We assume bit indexing follows little endian representation of a word.
    //  NOTE: bit shift operations are endianess agnostic in languages
    //  like C and Rust. This means >> shifts towards the LEAST SIGNIFICANT
    //  bit and << shifts towards the MOST SIGNIFICANT bit regardless of
    //  endianess convention used (e.g., little endianess used for all Intel
    //  processors and big endian often used for web applications).
    fn bit(&self, bit_index: Index) -> bool {
        assert!(bit_index / WORD_SIZE < N);
        let probe: Index = 1 << bit_index % WORD_SIZE;
        self[bit_index / WORD_SIZE] & probe != 0 
    }

    /// Return the bits from bit_start to bit_start + width
    /// We assume bit indexing follows little endian representation of a word.
    //  NOTE: bit shift operations are endianess agnostic in languages
    //  like C and Rust. This means >> shifts towards the LEAST SIGNIFICANT
    //  bit and << shifts towards the MOST SIGNIFICANT bit regardless of
    //  endianess convention used (e.g., little endianess used for all Intel
    //  processors and big endian often used for web applications).
    fn bits(&self, bit_row_start: Index, width: Index) -> Word {
        let w0: Index = bit_row_start / WORD_SIZE;
        let w1: Index = (bit_row_start + width - 1) / WORD_SIZE;
        let bit_word_start: Index = bit_row_start % WORD_SIZE;

        let mut result: Word = self[w0] >> bit_word_start;
        
        if w0 < w1 { result  ^= self[w1] << WORD_SIZE - bit_word_start; }
        result & ((1 << width) - 1)
    }
}

impl<const N: Index> UpdateRow<N> for Row<N> {
    fn update_add_rows(&mut self, r1: &Row<N>, r2: &Row<N>, i0: Index, i1: Index) {
        for i in i0..i1 {
            self[i] = r1[i] ^ r2[i]
        }
    }
}

pub fn print_mat<T: std::fmt::Debug + Copy, const M: Index, const N: Index>(mat: &Matrix<T, {M}, {N}>) {
    for row in mat.rows {
        println!("{:?}", row);
    }
}

pub fn print_gf2_mat<const M: Index, const N: Index>(mat: &Matrix<Word, {M}, {N}>) {
    for row in mat.rows {
        print!("[ ");
        for word in row {
            // :b displays words in big endian order
            let s = format!(" {:0>width$}", format!("{:b}", word), width = WORD_SIZE);
            // Display bits in little endian order
            print!("{}", &s.chars().rev().collect::<String>());
        }
        print!("]\n");
    }
}

pub fn four_russians_mat_print<const M: Index, const N: Index>(bits_per_chunk: Index, mat: &Matrix<Word, {M}, {N}>) {
    for row in mat.rows {
        let mut row_string = String::new();
        for word in row {
            // :b displays words in big endian order
            let zeropadded_bit_word = &format!("{:0>width$}", format!("{:b}", word), width = WORD_SIZE);
            // Display bits in little endian order
            row_string.push_str(&zeropadded_bit_word.chars().rev().collect::<String>());
        }
        let row_chars = row_string.chars();
        let mut i = 1;
        print!("[ ");
        for c in row_chars {
            print!("{}", c);
            if i % WORD_SIZE == 0 {
                print!(" ");
            }
            if i % bits_per_chunk == 0 {
                print!("|");
            }
            i += 1;
        }
        print!("]\n");
    }
}

const WORD_SIZE: usize = std::mem::size_of::<usize>() * 8;

/* 
     This diagram summarises the method. We use "4 Russian" tables
     of width bits_per_chunk bits (bits_per_chunk=3 below); current block starts at column chunk_start;
     to find pivot for row row_index, we start at j=row_index and reduce to the
     left using pivots chunk_start .. row_index-1; then check whether row j has a
     pivot for row row_index -- if it does, (possibly) xor it onto row row_index.
     Once the block is complete, form the table of size 2^bits_per_chunk and
     reduce below...
                   +-----+--------------------------------+
                   |1 * *|* * * * * * * * * * * * * * * * |
                   |  1 *|* * * * * * * * * * * * * * * * |
                   |    1|* * * * * * * * * * * * * * * * |
                   +-----+-----+--------------------------+
     chunk_start-> |     |1 * *|* * * * * * * * * * * * * |
                   |     |0 0 *|* * * * * * * * * * * * * | <- row_index
                   |     |0 0 *|* * * * * * * * * * * * * |
                   |     |* * *|* * * * * * * * * * * * * | <- j
                   |     |* * *|* * * * * * * * * * * * * |
                   |     |* * *|* * * * * * * * * * * * * |
                   +--------------------------------------+
      On failure to find a pivot, break out for a simpler loop
      that finishes it off.
*/

pub fn four_russians_rank<const M: Index, const N: Index>(mat: &mut Matrix<Word, {M}, {N}>) -> u32 {
    /*
     * Unfortunately, Rust is kind of dumb with the way it handles
     * constant generic types... you can't have bits_per_row below
     * be a const... this creates a headache for the lookup table!
     */
    let bits_per_row:   Index = N * WORD_SIZE;
    let bits_per_chunk: usize = log_2(M); //If bits_per_chunk <= 1 then error?

    /*
     * Initialize the Four Russians Lookup Table.
     * It will store all linear combinations of the pivots we have found
     * thus far. Since there are bits_per_chunk many pivots and each
     * can have 0 or 1 coefficient, there will be 2^bits_per_chunk many
     * entries needed for the ith chunk.
     * There are N chunks, so this is how we get Z below.
     */
    let lookup_table_size: usize = 1 << bits_per_chunk;
    let mut lookup_indices: Vec<Word>   = vec![     0; lookup_table_size];
    let mut lookup_table:   Vec<Row<N>> = vec![[0; N]; lookup_table_size]; 
    
    // Variable rank will store the rank of mat.
    let mut row_index: Index = 0;

    // Randomized Four Russian Method until not a full chunk or no pivot found
    'bigloop: for chunk_start in (0..M-M % bits_per_chunk).step_by(bits_per_chunk) {
        // Seach for pivots for this chunk
        for _ in chunk_start..chunk_start + bits_per_chunk {
            // Search the remaining rows for a pivot
            let mut found_pivot = None;
            for j in row_index..M {
                for k in chunk_start..row_index {
                    // Reduce relative to this chunk using pivots found so far
                    if mat.rows[j].bit(k) {
                        mat.add_rows(j, k, chunk_start / WORD_SIZE, N);
                    }
                }
                // Check to see if we found a pivot! If so, stop the search!
                // Otherwise, try the next row for a pivot!
                if mat.rows[j].bit(row_index) { 
                    found_pivot = Some(j);
                    break; 
                }
            }

            match found_pivot {
                None                      => break 'bigloop,
                Some(j) if j != row_index => mat.add_rows(row_index, j, chunk_start / WORD_SIZE, N),
                _                         => (),
            }
            row_index += 1;
        }
        // If not at end of matrix, then proceed with reduction
        if chunk_start + bits_per_chunk == M { break; }

        // Construct the four russians lookup table with pivots
        for i in 0..bits_per_chunk {
            // If we index the pivots 0..bits_per_chunk, then 
            // ii counts the number of linear combinations of 
            // pivots with the index of the maximal nonzero
            // coefficient being i, i.e., 2^i.
            let ii = 1 << i;

            // Grab the bits from the i^th pivot!
            let vv = mat.rows[chunk_start+i].bits(chunk_start, bits_per_chunk);

            // For each of our previously found linear combinations we xor with our new pivot.
            for j in 0..ii {
                let a   = lookup_indices[j];
                let b   = a ^ vv;
                lookup_indices[j+ii] = b;
                let Za  = lookup_table[a];
                lookup_table[b].update_add_rows(&Za, &mat.rows[chunk_start+i], bits_per_chunk / WORD_SIZE, N)
            }
        }
        // Now reduce below the pivots!
        for i in chunk_start + bits_per_chunk .. M {
            let c = mat.rows[i].bits(chunk_start, bits_per_chunk);
            mat.update_xor_row(i, &mut lookup_table[c], bits_per_chunk / WORD_SIZE, N)
        }
    }

    // Brute force the rest of the way!
    // Traverse columns one after another in search of a pivot!
    let mut col_index: Index = row_index; 
    while row_index < M && col_index < bits_per_row {
        let mut pivot = None;
        for j in row_index..M {
            if mat.rows[j].bit(col_index) { 
                pivot = Some(j);
                break;
            }
        }
        match pivot {
            None => {
                col_index += 1;
                continue;
            },
            Some(jj) => {
                if jj != row_index {
                    for w in col_index/64..N {
                        mat.rows[row_index][w] ^= mat.rows[jj][w];
                    }
                }
                assert!(mat.rows[row_index].bit(col_index));
                for j in row_index+1..M {
                    if mat.rows[j].bit(col_index) {
                        for w in col_index/64..N {
                            mat.rows[j][w] ^= mat.rows[row_index][w];
                        }
                    }
                }
                row_index += 1;
            }
        }
        col_index += 1;
    }
    let rank = row_index as u32;
    return rank;
}