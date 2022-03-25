pub mod staging;

fn main() {
    let mut random_numbers: Vec<u32> = vec![0u32; 10usize];
    staging::fill_array(&mut random_numbers, Some((0u32, 100u32)));
    let random_array_value: u32 = staging::select_random_array_value(&random_numbers);
    random_numbers.sort();
    staging::print_vector_array(&random_numbers);
    println!("Random number: {random_array_value}");
    let search_index_result: usize = staging::binary_search(&random_numbers, &random_array_value);
    println!("Base binary search index result: {search_index_result}");
    let search_index_result: usize =
        staging::binary_search_compare(&random_numbers, &random_array_value);
    println!("std::cmp binary search index result: {search_index_result}");
}

#[cfg(test)]
mod tests {

    #[test]
    fn default() {}
}
