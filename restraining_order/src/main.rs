use text_io::read;
fn main() {
    let input_sets:usize = read!();
    for _ in 0..input_sets {
        let (people_sizes, seat_sizes): (String, String) = (read!(), read!());
        fn digitized(num:String) -> Vec<usize>  { 
        num.chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect()
        }
        let ( mut people_digits, mut seat_digits) = (digitized(people_sizes), digitized(seat_sizes));
        seat_digits.sort();
        people_digits.sort();
        // println!("{:?}, {:?}",people_digits, seat_digits);

        fn can_fit_people(people: Vec<usize>, mut seats: Vec<usize>) -> bool {        
            for person in people {
                let mut fit = false;
                for seat in seats.iter_mut() {
                    if *seat >= person {
                        *seat -= person;
                        fit = true;
                        break;
                    }
                }
                if !fit {
                    return false; // Person couldn't fit in any seat
                }
            }
            true
        }
        if can_fit_people(people_digits, seat_digits) {
            println!("This looks safe.");
        }
        else {println!("Not gonna work.");}
    }
}

//sort seats l->g
//fill smallest seat
//go through persons l->g
//