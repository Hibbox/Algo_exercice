impl Solution {
    pub fn running_sum(mut nums: Vec<i32>) -> Vec<i32> {
        
        for i in 1..nums.len(){
            nums[i] += nums[i - 1]; // accumule les indexe et les aditionne jusqu'a la fin.
        }
        return nums;
        }
        
}