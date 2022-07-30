fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut res: Vec<i32> = vec![0, 0];

    let mut h = 0;
    while h < nums.len() {
        let mut l = nums.len() - 1;
        while h < l {
            if (nums[h] + nums[l]) == target {
                res = vec![h, l];
                break;
            }
            l -= 1
        }
        h += 1
    }
    return res;
}
fn main() {
    let res = two_sum([2, 7, 11, 15].to_vec(), 9);
    println!("{:?}", res);
}
