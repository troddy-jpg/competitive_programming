use text_io::*;
fn main() {
    let input_sets:u8 = read!();
    for _x in 0..input_sets {
        let tol_cap: usize = read!();
        let kids: usize = read!();
        let mut dyn_prog: Vec<Vec<usize>> = vec![vec![0;(tol_cap+1).into()];(kids+1).into()];
        for k in 0..kids {
            let annoyance:usize = read!(); 
            let cuteness:usize = read!();
            for curr_tol in 0..tol_cap+1{
                dyn_prog[(k+1) ][curr_tol ] = dyn_prog[(k) ][curr_tol ];
                if annoyance <= curr_tol {
                    dyn_prog [(k+1) ][curr_tol ] = std::cmp::max(
                        dyn_prog[(k+1) ][curr_tol ], 
                        dyn_prog[(k) ][(curr_tol - annoyance) ] + cuteness);
                }  
            }
        }
        println!("{:?}", dyn_prog);
    }
}
