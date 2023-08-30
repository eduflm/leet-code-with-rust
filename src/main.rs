
fn main() {
    let nums = vec![2,7,11,15];
    let target: i32 = 9;
    let result = two_sum(nums, target);
    println!("{:?}", result);
}

// pub fn two_sum_naive(nums: Vec<i32>, target: i32) -> Vec<i32> {
//     let length : usize = nums.len();
//     for i in 0..length {
//         for j in i+1..length {
//             let var1 = nums[i];
//             let var2 = nums[j];
//             if var1 + var2 == target {
//                 return vec![i,j]
//             }
//         } 
//     }
//     return vec![];
// }

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut subtractions : std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
    for i  in 0..nums.len() {
        let subtraction : i32 = target - nums[i];
        if subtractions.contains_key(&subtraction) {
            let first_sub_index : i32 = subtractions[&subtraction];
            return vec![first_sub_index, i as i32]
        } else {
            subtractions.insert(nums[i], i as i32);
        }
    }
    return vec![];   
}


#[cfg(test)]  // This attribute tells Rust to compile the tests only when running tests.
mod tests {
    use super::*;  // Import symbols from the outer module.

    #[test]
    fn test_sum_two_first() {
        assert_eq!(two_sum(vec![2,7,11,15], 9), vec![0,1]);
    }

    #[test]
    fn test_sum_two_second() {
        assert_eq!(two_sum(vec![3,2,4], 6), vec![1,2]);
    }

    #[test]
    fn test_sum_two_third() {
        assert_eq!(two_sum(vec![3,3], 6), vec![0,1]);
    }
}
