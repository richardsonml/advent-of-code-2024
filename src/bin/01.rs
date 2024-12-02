use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut left_values: Vec<u32> = Vec::new();
    let mut right_values: Vec<u32> = Vec::new();

    // Get the left and right side values into separate vectors (lists)
    for line in input.lines() {
        let mut pairs = line.split_whitespace();
        if let (Some(first), Some(second)) = (pairs.next(), pairs.next()) {
            if let (Ok(a), Ok(b)) = (first.parse::<u32>(), second.parse::<u32>()) {
                left_values.push(a);
                right_values.push(b);
            }
        }
    }

    // Sort the sides
    left_values.sort_unstable();
    right_values.sort_unstable();

    // Get the difference from each pair and add it to the total distance
    let mut total_distance = 0;

    for i in 0..left_values.len() {
        total_distance += left_values[i].abs_diff(right_values[i]);
    }

    Some(total_distance)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left_values: Vec<u32> = Vec::new();
    let mut right_values: Vec<u32> = Vec::new();

    // Get the left and right side values into separate vectors (lists)
    for line in input.lines() {
        let mut pairs = line.split_whitespace();
        if let (Some(first), Some(second)) = (pairs.next(), pairs.next()) {
            if let (Ok(a), Ok(b)) = (first.parse::<u32>(), second.parse::<u32>()) {
                left_values.push(a);
                right_values.push(b);
            }
        }
    }

    // Generate a frequency map of each number from the right side values
    let mut frequency_map = HashMap::new();

    for &value in &right_values {
        *frequency_map.entry(value).or_insert(0) += 1;
    }

    // Iterate the left side values and update the similarity if the value from the left side is found in the frequency map
    let mut total_similarity = 0;

    for value in &left_values {
        match frequency_map.get(value) {
            Some(&frequency) => total_similarity += value * frequency,
            None => total_similarity += 0,
        }
    }

    Some(total_similarity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
