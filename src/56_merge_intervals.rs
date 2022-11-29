fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut arr: Vec<Vec<i32>> = [].to_vec();

    let mut sorted_intervals = intervals;

    sorted_intervals.sort();

    for value in sorted_intervals.iter() {
        let left = value[0];

        let right = value[1];

        if arr.is_empty() {
            arr.push([left, right].to_vec());
        } else if let Some(last) = arr.last_mut() {
            if last[1] < left {
                arr.push([left, right].to_vec());
            } else {
                last[1] = right.max(last[1]);
            }
        }
    }

    return arr;
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let intervals_one: Vec<Vec<i32>> = vec![[1, 4].to_vec(), [0, 0].to_vec()];
        let arr_one = merge(intervals_one);
        assert_eq!(arr_one, vec![[0, 0].to_vec(), [1, 4].to_vec()]);
    }

    #[test]
    fn test_two() {
        let intervals_two = vec![
            [1, 3].to_vec(),
            [2, 6].to_vec(),
            [8, 10].to_vec(),
            [15, 18].to_vec(),
        ];

        let arr_two = merge(intervals_two);

        assert_eq!(
            arr_two,
            vec![[1, 6].to_vec(), [8, 10].to_vec(), [15, 18].to_vec()]
        );
    }
    #[test]
    fn test_three() {
        let intervals = vec![[1, 4].to_vec(), [2, 3].to_vec()];

        let arr = merge(intervals);

        assert_eq!(arr, vec![[1, 4].to_vec()]);
    }
    #[test]
    fn test_four() {
        let intervals = vec![[1, 4].to_vec(), [4, 5].to_vec()];

        let arr = merge(intervals);

        assert_eq!(arr, vec![[1, 5].to_vec()]);
    }
}
