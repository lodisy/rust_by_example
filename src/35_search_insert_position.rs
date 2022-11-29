fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut index: usize = 0;

    for i in 0..nums.len() {
        if nums[i] >= target {
            index = i;
            break;
        }

        index = nums.len();
    }

    return index as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let nums = vec![1, 3, 5, 6];
        let target = 5;

        let index = search_insert(nums, target);

        assert_eq(index, 2);
    }
    #[test]
    fn test_two() {
        let nums = vec![1, 3, 5, 6, 7];
        let target = 8;

        let index = search_insert(nums, target);

        assert_eq(index, 5);
    }
}
