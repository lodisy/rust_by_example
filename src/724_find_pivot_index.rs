fn find_pivot_index(nums: Vec<i32>) -> i32 {
    let mut left_sum = 0;

    let sum: i32 = nums.iter().sum();

    for (key, value) in nums.iter().enumerate() {
        if 2 * left_sum + nums[key] == sum {
            return key as i32;
        }

        left_sum += value;
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let arr = vec![-1, -1, -1, -1, -1, 0];
        let index = find_pivot_index(arr);

        assert_eq!(2, index);
    }
}
