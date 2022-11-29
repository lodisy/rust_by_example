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

fn main() {
    let nums_one = vec![1, 3, 5, 6];

    let nums_two = vec![1, 3, 5, 6, 7];

    let target_one = 5;

    let target_two = 8;

    let index_one = search_insert(nums_one, target_one);

    let index_two = search_insert(nums_two, target_two);

    assert_eq!(index_one, 2);

    assert_eq!(index_two, 5);
}
