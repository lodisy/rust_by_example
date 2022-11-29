fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut index: Vec<i32> = [].to_vec();

    for (k, v) in nums.iter().enumerate() {
        let d = target - v;

        let result: Vec<_> = nums
            .iter()
            .enumerate()
            .filter(|&x| *x.1 == d && x.0 > k)
            .collect();

        if !result.is_empty() {
            index.push(k as i32);
            index.push(result[0].0 as i32);
            break;
        }
    }

    index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = [2, 7, 11, 15].to_vec();

        let target = 9;

        let index = two_sum(nums, target);

        assert_eq!(index, [0, 1].to_vec());
    }
    #[test]
    fn test_again() {
        let nums = [3, 3].to_vec();

        let target = 6;

        let index = two_sum(nums, target);

        assert_eq!(index, [0, 1].to_vec());
    }
    #[test]
    fn test_third() {
        let nums = [3, 2, 4].to_vec();

        let target = 6;

        let index = two_sum(nums, target);

        assert_eq!(index, [1, 2].to_vec());
    }
}
