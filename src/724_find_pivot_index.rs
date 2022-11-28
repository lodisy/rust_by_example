fn calculate(nums: Vec<i32>) -> i32 {
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

fn main() {
    let arr = vec![-1, -1, -1, -1, -1, 0];

    let result = calculate(arr);

    println!("The index is {}", result);
}
