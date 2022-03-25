use rand::{prelude::ThreadRng, Rng};

pub fn fill_array(array: &mut Vec<u32>, random_number_range: Option<(u32, u32)>) -> () {
    let mut random_number_generator: ThreadRng = rand::thread_rng();
    match random_number_range {
        Some(range) => {
            for index in 0..array.len() {
                array[index] = random_number_generator.gen_range(range.0..range.1);
            }
        }
        None => {
            for index in 0..array.len() {
                array[index] = random_number_generator.gen();
            }
        }
    }
}

pub fn select_random_array_value(array: &Vec<u32>) -> u32 {
    let mut random_number_generator: ThreadRng = rand::thread_rng();

    let random_index = random_number_generator.gen_range(0usize..array.len());

    return array[random_index].clone();
}

pub fn binary_search(array: &Vec<u32>, target: &u32) -> usize {
    if array.len() == 0 {
        return 0usize;
    }

    let mut minimum: usize = 0usize;
    let mut maximum: usize = array.len() - 1;

    let calculate_middle = |min: usize, max: usize| {
        return min + (max - min) / 2;
    };

    while minimum <= maximum {
        let middle: usize = calculate_middle(minimum, maximum);
        if &array[middle] == target {
            return middle;
        } else if &array[middle] < target {
            minimum = middle + 1;
        } else {
            maximum = middle - 1;
        }
    }
    0usize
}

pub fn binary_search_compare(array: &Vec<u32>, target: &u32) -> usize {
    use std::cmp::Ordering;

    let (mut minimum, mut maximum): (usize, usize) = (0usize, array.len());

    while minimum < maximum {
        let middle = minimum + (maximum - minimum) / 2;
        match array[middle].cmp(&target) {
            Ordering::Equal => return middle,
            Ordering::Less => minimum = middle + 1,
            Ordering::Greater => maximum = middle,
        };
    }
    0usize
}

pub fn print_vector_array(array: &Vec<u32>) -> () {
    print!("Array Values: ");
    for number in array {
        print!(" {number} ");
    }
    println!("");
}
