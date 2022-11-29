fn final_prices(prices: Vec<i32>) -> Vec<i32> {
    let mut arr = [].to_vec();

    for (k, e) in prices.iter().enumerate() {
        let discount: Vec<_> = prices
            .iter()
            .enumerate()
            .filter(|&x| x.1 <= e && x.0 > k)
            .collect();

        if discount.is_empty() {
            arr.push(*e);
        } else {
            arr.push(e - discount[0].1);
        }
    }

    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let prices = vec![10, 1, 1, 6];

        let arr = final_prices(prices);

        assert_eq!(arr, vec![9, 0, 1, 6]);
    }
    #[test]
    fn test_two() {
        let prices = vec![1, 2, 3, 4, 5];

        let arr = final_prices(prices);

        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }
}
