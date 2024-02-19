use text_io::*;
fn main() {
    let mut goats: Vec<Vec<u32>> = vec![vec![], vec![]];
    let input_sets:u8 = read!();
    for _x in 0..input_sets {
        let tol_cap: u8 = read!();
        let kids: u8 = read!();
        let mut dyn_prog: Vec<Vec<u8>> = vec![vec![0;tol_cap.into()];kids.into()];
        println!("{:?}", dyn_prog);
        
        for k in 0..kids {
            let annoyance:u8 = read!(); 
            let cuteness:u8 = read!();
            for i in 0..tol_cap{
                println!("{}", dyn_prog[k as usize][i as usize]);   
            }
        }
        
        
        // weight_limit_tons = weight_limit_tons * 2000;
        // let family_len: u8 = read!();
        // let mut family_sum: u32 = 0;
        // for _y in 0..family_len {
        //     family_sum += {let readVar: u32=read!();readVar};
        // }
        // if family_sum > weight_limit_tons {println!("Time to clean the downstairs kitchen.");}
        // else {println!("All clear.");}
    }
}