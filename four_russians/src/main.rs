mod arithmetic;
mod four_russians;

use crate::four_russians::print_mat;
use crate::four_russians::Matrix;


fn main() {
    let mat = Matrix::<u32, 2, 3>{
        rows : [[1,2,3], [4,5,6]]
    };
    print_mat(mat);
}
