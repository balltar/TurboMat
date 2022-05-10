// Use usize for indexing and u32 for numbers...
// Complications really come up when you are doing arithmetic
// operations with your indices... seems bad.

use crate::arithmetic::Word;
use crate::arithmetic::Index;

pub fn print_type_of<T>(_: &T) { println!("{}", std::any::type_name::<T>()) }
pub fn ceil_div(x: Index, y: Index) -> Index { x / y + (x % y != 0) as Index }
pub fn ceil_rem(x: Index, y: Index) -> Index { if x % y != 0 {x % y} else {y} }

const fn log_2(x: usize) -> usize {
    assert!(x > 0);
    WORD_SIZE -  (x.leading_zeros() as usize) - 1
}

#[derive(Debug)]
pub struct Matrix<T, const M: usize, const N: usize> {
    pub rows: [[T; N]; M]
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
    fn bit(&self, bit_index: Index) -> bool {
        assert!(bit_index / WORD_SIZE < N);
        let probe: Index = 1 << (bit_index % WORD_SIZE);
        self[bit_index / WORD_SIZE] & probe != 0 
    }

    /// Return the bits from bit_start to bit_start + width
    fn bits(&self, bit_row_start: Index, width: Index) -> Word {
        assert!(if bit_row_start + width > WORD_SIZE {N > bit_row_start / WORD_SIZE} else {true});
        let w0: Index = bit_row_start / WORD_SIZE;
        let w1: Index = (bit_row_start + width - 1) / WORD_SIZE;
        let bit_word_start: Index = bit_row_start % WORD_SIZE;

        let mut result: Word = self[w0];
        
        if w0 < w1 {
            result <<= width - (WORD_SIZE - bit_word_start);
            result  ^= w1 >> (WORD_SIZE - (width - (WORD_SIZE - bit_word_start)));
        }
        else {
            result >>= WORD_SIZE - (bit_word_start + width);
        }

        
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
        print!("|");
        for word in row {
            //print!("{:0>width$}|", format!("{:b}", word), width = WORD_SIZE);
            let s = format!("{:0>width$}|", format!("{:b}", word), width = WORD_SIZE);
            print!("{}", s);
        }
        print!("\n");
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
*/

pub fn four_russians_mat_print<const M: Index, const N: Index>(bits_per_chunk: Index, mat: &Matrix<Word, {M}, {N}>) {
    for row in mat.rows {
        let mut row_string = String::new();
        for word in row {
            let zeropadded_bit_word = &format!("{:0>width$}", format!("{:b}", word), width = WORD_SIZE);
            row_string.push_str(zeropadded_bit_word);
        }
        let row_chars = row_string.chars();
        let mut i = 1;
        print!("[");
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
pub fn four_russians_rank<const M: Index, const N: Index>(mat: &mut Matrix<Word, {M}, {N}>) -> u32 {
    const a: usize = Matrix::<Word, {M}, {N}>::N_words;
    const b: usize = log_2(a);

    let mut Z = Matrix::<usize, b, N> {
        rows: [[0; N]; b]
    };

    return 5;
}*/


pub fn four_russians_rank<const M: Index, const N: Index>(mat: &mut Matrix<Word, {M}, {N}>) -> u32 {
    // Unfortunately, Rust is kind of dumb with the way it handles
    // constant generic types... you can't have bits_per_row below
    // be a const... this creates a headache for the lookup table!
    let bits_per_row: Index   = N * WORD_SIZE;
    let bits_per_chunk: usize = log_2(bits_per_row);

    // Initialize the Four Russians Lookup Table.
    // It will store all linear combinations of the pivots we have found
    // thus far. Since there are bits_per_chunk many pivots and each
    // can have 0 or 1 coefficient, there will be 2^bits_per_chunk many
    // entries needed for the ith chunk.
    // There are N chunks, so this is how we get Z below.
    let lookup_table_size: usize = 1 << bits_per_chunk;
    let mut z: Vec<Word>      = Vec::with_capacity(lookup_table_size);
    let mut Z: Vec<Row<N>> = Vec::with_capacity(lookup_table_size); 

    //println!("The number of bits per row is {:?}", bits_per_row);
    //println!("The number of bits per chunk is {:?}", bits_per_chunk);
    //println!("The number of chunks is {:?}", ceil_div(bits_per_row, bits_per_chunk));
    //println!("The number of bits in the last chunk is: {:?}", ceil_rem(bits_per_row, bits_per_chunk));
    
    // Variable rank will store the rank of mat.
    let row_index: Index = 0;

    // Variable chunk_start will store the starting index of the current chunk
    let mut chunk_start: Index = 0;

    while bits_per_chunk > 1 && chunk_start + bits_per_chunk <= M && row_index == chunk_start {
        // Seach for pivots for this chunk
        for _ in chunk_start..chunk_start + bits_per_chunk {
            // Search the remaining rows for a pivot
            let mut j: Index = row_index;
            while j < M {
                for k in chunk_start..row_index {
                    // Reduce relative to this chunk using pivots found so far
                    if mat.rows[j].bit(k) {
                        mat.add_rows(j, k, chunk_start / WORD_SIZE, N)
                    }
                }
                // Check to see if we found a pivot! If so, stop the search!
                if mat.rows[j].bit(row_index) { break; }

                // Otherwise, try the next row for a pivot!
                j += 1;
            }
            
            // We've reached the end
            if j == M { break; }

            // Create a new pivot
            if j != row_index { mat.add_rows(row_index, j, chunk_start / WORD_SIZE, N); }

            //if piv thing
        }

        // Couldn't find enough pivots
        if row_index != chunk_start + bits_per_chunk { break; }

        // End of matrix
        if chunk_start + bits_per_chunk == M { break; }

        //Clear z[0] to start over for this chunk.
        z[0] = 0;

        // For each pivot
        for i in 0..bits_per_chunk {
            // If we index the pivots 0..bits_per_chunk, then 
            // ii counts the number of linear combinations of 
            // pivots with the index of the maximal nonzero
            // coefficient being i, i.e., 2^i.
            let ii = 1 << i;

            // Let's grab the bits for the i^th pivot!
            let vv = mat.rows[chunk_start+i].bits(chunk_start, bits_per_chunk);

            // For each of our previously found linear combinations we 
            // xor with our new pivot.
            for j in 0..ii {
                let a = z[j];
                let b = a ^ vv;
                z[j+ii] = b;
                // is following line necessary?
                let Za  = Z[a];
                Z[b].update_add_rows(&Za, &mat.rows[chunk_start+i], bits_per_chunk / WORD_SIZE, N)
            }

            // Now reduce below the pivots!
            for i in bits_per_chunk + chunk_start .. M {
                let c = mat.rows[i].bits(chunk_start, bits_per_chunk);
                mat.update_xor_row(i, &mut Z[c], bits_per_chunk / WORD_SIZE, N)
            }
        }

        // NEXT TIME: WHY IS THIS NEVER REACHED??
        print_gf2_mat(&mat);

        // Next chunk!
        chunk_start += bits_per_chunk;
    }

    let rank = row_index as u32;

    return rank;
}