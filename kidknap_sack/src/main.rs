use text_io::read;
fn main() {
    let input_sets:u8 = read!();
    for _x in 0..input_sets {
        let (tol_cap, num_kids): (usize, usize) = (read!(), read!());
        let mut dp_table: Vec<Vec<usize>> = vec![vec![0;(tol_cap+1).into()];(num_kids+1).into()];
        (0..num_kids).for_each(|kids| {
            let (annoyance, cuteness): (usize, usize) = (read!(), read!());
            (0..=tol_cap).for_each(|curr_tol| {
                    dp_table[kids+1 ][curr_tol ] = dp_table[kids ][curr_tol ];
                    if annoyance <= curr_tol {
                        dp_table [kids+1][curr_tol] = std::cmp::max(
                            dp_table[kids+1][curr_tol], 
                            dp_table[kids][curr_tol - annoyance] + cuteness);
                    }  
                });
        });
        println!("{}", dp_table[num_kids][tol_cap]);
    }
}


//use text_io::read;
//
// BELOW: CHATGPT SOLUTION USING A FOLD TO MAKE AN IMMUTABLE DP_TABLE. PROBS NOT EVEN A BETTER SOLUTION BUT IDK...
//

// fn main() {
//     let input_sets: u8 = read!();
//     for _ in 0..input_sets {
//         let (tol_cap, num_kids): (usize, usize) = (read!(), read!());
//         let dp_table = (0..=num_kids)
//         .fold(
//             vec![vec![0; tol_cap + 1]; num_kids + 1], 
//             |mut acc, kid| {
//             if kid > 0 {
//                 let (annoyance, cuteness): (usize, usize) = (read!(), read!());
//                 (0..=tol_cap).for_each(|curr_tol| {
//                     acc[kid][curr_tol] = acc[kid - 1][curr_tol];
//                     if annoyance <= curr_tol {
//                         acc[kid][curr_tol] = std::cmp::max(
//                             acc[kid][curr_tol],
//                             acc[kid - 1][curr_tol - annoyance] + cuteness,
//                         );
//                     }
//                 });
//             }
//             acc
//         });
//         println!("{}", dp_table[num_kids][tol_cap]);
//     }
// }
