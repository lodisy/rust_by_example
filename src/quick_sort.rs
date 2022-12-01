#[allow(dead_code)]
fn partition(arr: &mut Vec<i32>, left: usize, right: usize) -> usize {
    // right is the last index in the array
    let pivot = right;

    let mut index = left;
    for i in left..right {
        if arr[i] < arr[pivot] {
            // move all elements that are less than pivot to the range of [0,index]
            arr.swap(i, index);
            index += 1;
        }
    }
    // center the pivot
    arr.swap(index, pivot);

    index
}
#[allow(dead_code)]
fn quick_sort(arr: &mut Vec<i32>, left: usize, right: usize) {
    if left >= right || arr.len() <= 1 {
        println!("Right must be greater than left");

        return;
    }

    let pivot_index = partition(arr, left, right);

    if pivot_index < 1 {
        return; // make sure that the left and right is usize ( >= 0)
    }

    // sort left
    quick_sort(arr, left, pivot_index - 1);
    // sort right
    quick_sort(arr, pivot_index + 1, right);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut arr = vec![2, 3, 1, 5, 2];

        quick_sort(&mut arr, 0, 4);

        assert_eq!(arr, [1, 2, 2, 3, 5].to_vec());
    }

    #[test]
    fn test_quick_sort_again() {
        let mut arr = vec![2, 5, 6, 1, 9, 10, 5];

        quick_sort(&mut arr, 0, 6);

        assert_eq!(arr, [1, 2, 5, 5, 6, 9, 10].to_vec());
    }
}
