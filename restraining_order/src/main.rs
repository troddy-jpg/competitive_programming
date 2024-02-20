use text_io::read;
fn main() {
    let input_sets:usize = read!();
    for _ in 0..input_sets {
        let (people_sizes, seat_sizes): (isize, isize) = (read!(), read!());
        let (individual_people, mut individual_seats) = (usize_to_vec_of_isize(people_sizes), usize_to_vec_of_isize(seat_sizes));
        
        individual_seats.sort();
        individual_seats.push(0);

        for person in individual_people {
            let next_seat = get_available_seat(&individual_seats, person);
            individual_seats[next_seat] = individual_seats[next_seat] - person;
        }
        if individual_seats[individual_seats.len()-1] < 0 
            {println!("Not gonna work.");}
        else
            {println!("This looks safe.");}
    }
}

fn usize_to_vec_of_isize(n: isize) -> Vec<isize> {
    n.to_string()
     .chars()
     .map(|c| c.to_digit(9).unwrap() as isize)
     .collect()
}

fn get_available_seat(seats: &Vec<isize>, person: isize) -> usize {
    for curr_seat in 0..seats.len(){
        if person <= seats[curr_seat]{ 
            return curr_seat;
        }
    }
    seats.len()-1
}