mod arithmetic;
mod four_russians;

use crate::arithmetic::Word;

use crate::four_russians::Matrix;
use crate::four_russians::Bit;
use crate::four_russians::print_mat;
use crate::four_russians::print_gf2_mat;
use crate::four_russians::four_russians_mat_print;
use crate::four_russians::four_russians_rank;


fn main() {
    let mut mat = Matrix::<Word, 2, 2>{
        rows : [[1,2], [4,5]]
    };
    print_mat(&mat);
    print_gf2_mat(&mat);
    four_russians_rank(&mut mat);
    four_russians_mat_print(7, &mat);

    println!("{}", format!("{:0>7}", format!("{:b}", mat.rows[0].bits(9*7, 7))));
}
