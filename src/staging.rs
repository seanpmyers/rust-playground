use rand::{prelude::ThreadRng, Rng};

pub fn fill_array(array: &mut Vec<u32>, random_number_range: Option<(u32, u32)>) {
    let mut random_number_generator: ThreadRng = rand::thread_rng();
    match random_number_range {
        Some(range) => {
            (0..array.len()).for_each(|index| {
                array[index] = random_number_generator.gen_range(range.0..range.1);
            });
        }
        None => {
            (0..array.len()).for_each(|index| {
                array[index] = random_number_generator.gen();
            });
        }
    }
}

pub fn select_random_array_value(array: &Vec<u32>) -> u32 {
    let mut random_number_generator: ThreadRng = rand::thread_rng();

    let random_index = random_number_generator.gen_range(0usize..array.len());

    array[random_index]
}

pub fn binary_search(array: &Vec<u32>, target: &u32) -> usize {
    if array.is_empty() {
        return 0usize;
    }

    let mut minimum: usize = 0usize;
    let mut maximum: usize = array.len() - 1;

    let calculate_middle = |min: usize, max: usize| min + (max - min) / 2;

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
        match array[middle].cmp(target) {
            Ordering::Equal => return middle,
            Ordering::Less => minimum = middle + 1,
            Ordering::Greater => maximum = middle,
        };
    }
    0usize
}

pub fn print_vector_array(array: &Vec<u32>) {
    print!("Array Values: ");
    for number in array {
        print!(" {number} ");
    }
    println!();
}

pub fn two_sum(array: &[i32], target: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let map: HashMap<&i32, i32> = array.iter().zip(0..).collect();
    let s = array
        .iter()
        .map(|b| target - b)
        .zip(0..)
        .filter_map(|(a, n)| {
            map.get(&a)
                .and_then(|&idx| if idx != n { Some((n, idx)) } else { None })
        })
        .next()
        .unwrap_or_else(|| panic!("No two sum results to {target}"));
    vec![s.0, s.1]
}

// pub fn test(array: &Vec<i32>, target: i32) -> Vec<i32> {
//     use std::collections::HashMap;
//     let map: HashMap<i32,i32> = array.iter().zip(0..).collect();

//     let result = array
//         .iter()
//         .map(|value| target - value)
//         .zip(0..)
//         .filter_map(|&value| )

//     result[]
// }
