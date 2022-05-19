mod arithmetic;
mod four_russians;

use std::cmp;

use crate::arithmetic::Word;
use crate::four_russians::{four_russians_mat_print, log_2, four_russians_rank, print_mat, print_gf2_mat, Matrix, Bit};


fn main() {
    let mut mat = Matrix::<Word, 32, 2>{
        rows : [[2,2], [4,5], [7, 28], [31, 13], [127, 5011], [3456, 255],
                [123456, 78910], [666, 666666],
                [11166744151617181920, 11121314847317181920],[11121314234517181920, 11123854151617181920],
                [15536314151617181920, 12342344151617181920], [11121314198647181920, 11121314151901281920],
                [17733314151617181920, 11121314151617181234], [11121334151617181920, 11958314151432081920],
                [11121314151617181234, 11121314151617183253], [11122344151617181920, 11121314151689321920],
                [11121314151617590420, 11123424151617181920], [11121314151695861920, 1112131415138581920],
                [11121319090161718120, 16221314151617181920], [18390314151617181920, 11112344151617181920],
                [11121312291617181920, 11121311230617181920], [12345313453617181920, 18341314151617181920],
                [11121312201617181920, 11121314151612312920], [17121314151617181920, 11121323451617181920], 
                [11121314151933381920, 11121314151349181920], [11125963151617181920, 11121314151619085920],
                [11121313859617181920, 11289534151617181920], [11121393485617181920, 11121314151937581920],
                [11132454151617181920, 11196744151617181920], [11121314169737181920, 11121314123487181920],
                [11188844151617181920, 11139774151617181920], [11121314128357181920, 11121314151698524920]]
    };
    let mut mat2 = Matrix::<Word, 32, 2>{
        rows : [[0,0], [0,0], [0,0], [0,0], [0,0], [0,0],
                [0,0], [0,0],
                [0,0], [0,0],
                [0,0], [0,0],
                [0,0], [0,0],
                [0,0], [0,0],
                [0,0], [0,11121312291617181920],
                [0,0], [0,11121312234598345340],
                [0,0], [0,0],
                [0,0], [0,0],
                [0,0], [0,0],
                [0,0], [0,0],
                [0,0], [0,0],
                [0,0], [0,0],]
    };
    println!("Matrix partitioned by word size:\n");
    print_gf2_mat(&mat);

    /*
    println!("0 bit of row 0 is nonzero: {:?}", mat.rows[0].bit(0));
    println!("2 bit of row 1 is nonzero: {:?}", mat.rows[1].bit(2));
    println!("1 bit of row 1 is nonzero: {:?}", mat.rows[1].bit(1));
    println!("0 bit of row 1 is nonzero: {:?}", mat.rows[1].bit(0));
    println!("63 bit of row 9 is nonzero: {:?}", mat.rows[9].bit(63));
    println!("62 bit of row 9 is nonzero: {:?}", mat.rows[9].bit(62));

    println!("First five bits of row 0 are: {:?}", mat.rows[0].bits(0, 7));
    println!("First five bits of row 1 are: {:?}", mat.rows[1].bits(0, 7));
    println!("First five bits of row 2 are: {:?}", mat.rows[2].bits(0, 7));
    println!("First five bits of row 9 are: {:?}", mat.rows[9].bits(0, 7));

    println!("Bits 62-65 of row 0 are: {:?}", mat.rows[0].bits(62, 4));
    println!("Bits 62-65 of row 1 are: {:?}", mat.rows[1].bits(62, 4));
    println!("Bits 62-65 of row 2 are: {:?}", mat.rows[2].bits(62, 4));
    println!("Bits 62-65 of row 9 are: {:?}", mat.rows[9].bits(62, 4));
    */

    let rank = four_russians_rank(&mut mat);
    println!("\nMatrix partitioned by four russian blocks has rank {:?}\n", rank);
    let c = log_2(mat.num_rows().try_into().unwrap());
    four_russians_mat_print(c, &mat);

    if rank == mat.num_rows().try_into().unwrap() {
        println!("\n The matrix is full rank!");
    }

    four_russians_mat_print(c, &mat2);
    let rank2 = four_russians_rank(&mut mat2);
    println!("\nMatrix partitioned by four russian blocks has rank {:?}\n", rank2);
    four_russians_mat_print(c, &mat2);

    if rank2 == mat2.num_rows().try_into().unwrap() {
        println!("\n The matrix is full rank!");
    }

    //println!("{}", format!("{:0>c$}", format!("{:b}", mat.rows[0].bits(12*c, c))));
    //println!("{}", format!("{:0>c$}", format!("{:b}", mat.rows[1].bits(12*c, c))));
    //println!("{}", mat.rows[0].bit(0));
}
