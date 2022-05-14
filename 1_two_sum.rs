impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
		// Vector of indexes sorted by values of nums
        let mut index_vec:Vec<usize> = (0..nums.len()).collect();
		index_vec.sort_by_key(|k| nums[*k]);
        // Larger number searched
        let mut upper_index:usize = nums.len()-1;
        for i in 0..nums.len() {
            // Eliminate numbers that are too large to be used in sum
            while (nums[index_vec[i]] + nums[index_vec[upper_index]]) > target {
                upper_index -= 1;
            }
            // Return indexes if 2 numbers add uo to target
            if nums[index_vec[i]] + nums[index_vec[upper_index]] == target {
                let res:Vec<i32> = vec![index_vec[i] as i32, index_vec[upper_index] as i32];
                return res;
            }
        }
        // Should never run, here to avoid compiler errors
        return vec![0,0];
    }
}
