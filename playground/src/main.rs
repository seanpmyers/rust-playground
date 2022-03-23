pub mod staging;

fn main() {
    let mut random_numbers: Vec<u32> = vec![0u32; 10usize];
    staging::fill_array(&mut random_numbers, Some((0u32, 100u32)));
    staging::print_vector_array(&random_numbers);
}

#[cfg(test)]
mod tests {

    #[test]
    fn default() {}
}
