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

pub fn print_vector_array(array: &Vec<u32>) -> () {
    for number in array {
        println!("{number}");
    }
}
