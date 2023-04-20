pub struct Solution;

impl Solution {
    pub fn find_swap_values(array1: Vec<i32>, array2: Vec<i32>) -> Vec<i32> {
        let sum1 = array1.iter().sum::<i32>();
        let sum2 = array2.iter().sum::<i32>();
        let diff = sum1 - sum2;
        if diff % 2 != 0 {
            return vec![];
        }
        let target = diff / 2;
        let set1 = array1.into_iter().collect::<std::collections::HashSet<i32>>();
        for x in array2 {
            let y = x + target;
            if set1.contains(&y) {
                return vec![y, x];
            }
        }
        vec![]
    }
}


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_find_swap_values() {
        let array1 = vec![4, 1, 2, 1, 1, 2];
        let array2 = vec![3, 6, 3, 3];
        let result = Solution::find_swap_values(array1.clone(), array2.clone());
        println!("Test case 1 result: {:?}", result);
        assert!(result == vec![1, 3]);

        
    }
}
