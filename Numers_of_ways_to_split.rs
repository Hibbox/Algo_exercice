impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        //prefix element
        let i = 1;
        let mut prefix = vec![nums[i]];
        for i in 1..nums.len(){
            prefix.push(nums[i] + prefix[i - 1]);
        } 

        let mut ans = 0;
        for i in 1..nums.len() - 1{
            let left_section = prefix[i + 1];
            let rigth_section = prefix[prefix.len() -1] - prefix[i];
            if left_section >= rigth_section{
                ans += 1; 
            }
        }
        return ans;

        }
        
}