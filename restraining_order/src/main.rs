use text_io::read;
fn main() {
    let input_sets:u8 = read!();
    for _ in 0..input_sets {
        println!("Input set!");
        let (people_sizes, seat_sizes): (usize, usize) = (read!(), read!());
        let (individual_people, mut individual_seats) = (usize_to_vec_of_u8(people_sizes), usize_to_vec_of_u8(seat_sizes));
        individual_seats.sort();
        let mut filled_seats = vec![0;individual_seats.len()];

        let mut seats_to_fill: Vec<(usize, usize)> = individual_seats.iter()
        .map(|&value| (value as usize, 0))
        .collect();
        println!("ppls:{:?}, seats:{:?}", individual_people, seats_to_fill);

        for person in individual_people {
            for (pos, seat) in individual_seats.iter().enumerate() {
                if person <= *seat && filled_seats[pos] == 0
                    {filled_seats[pos] = 1;}
            }
            //go through the bins in order of smallest to greatest
            //put person in the first bin which is big enough to fit it.
            println!("{}", person);
        }
    }
    println!("This looks safe.");
    println!("Not gonna work.");
}

fn usize_to_vec_of_u8(n: usize) -> Vec<u8> {
    n.to_string()
     .chars()
     .map(|c| c.to_digit(10).unwrap() as u8)
     .collect()
}


// fn main() {
//     let (tol_cap, num_kids): (usize, usize) = (read!(), read!());
//         let mut dp_table: Vec<Vec<usize>> = vec![vec![0;(tol_cap+1).into()];(num_kids+1).into()];
//         (0..num_kids).for_each(|kids| {
//             let (annoyance, cuteness): (usize, usize) = (read!(), read!());
//             (0..=tol_cap).for_each(|curr_tol| {
//                     dp_table[kids+1 ][curr_tol ] = dp_table[kids ][curr_tol ];
//                     if annoyance <= curr_tol {
//                         dp_table [kids+1][curr_tol] = std::cmp::max(
//                             dp_table[kids+1][curr_tol], 
//                             dp_table[kids][curr_tol - annoyance] + cuteness);
//                     }  
//                 });
//         });
//         println!("{}", dp_table[num_kids][tol_cap]);
//     }
// }