impl Solution {
    pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if k == 0{
            return nums;
        }
        let mut prefix: Vec<i64> = vec![nums[0] as i64];
        for i in 1..nums.len(){
            prefix.push(prefix[i-1] + nums[i] as i64)
        }
         let n = nums.len();
         let mut result = vec![-1; n]; //  on initialise le résultat avec la valeur par défaut, puis on remplace ce qui doit être remplacé.
        for i in 0..n{
            let mut sum = 0;
            let mut avrg = 0;
            let left_i32 = i as i32 - k;
            let right_i32 = i as i32 + k;
            
            if left_i32 < 0 || right_i32 >= n as i32{ 
                continue; 
            } 
            let left = left_i32 as usize;
            let right = right_i32 as usize;
            if left == 0 {
                sum = prefix[right];
            } else {
                sum = prefix[right] - prefix[left -1] ; // une somme d'interval de soustableau se calcul avec sous traction
            }
            let window =  (2 * k + 1) as i64;
            avrg = (sum / window) as i32;
            result[i] = avrg;
        }
        return result;
        
    }
}