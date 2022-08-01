fn find_missing_ranges(nums: Vec<i32>, lower: i32, upper: i32) -> Vec<String> {

    let mut w = nums;
    let mut res: Vec<String> = Vec::new();
    w.insert(0, lower-1);
    w.insert(w.len(), upper+1);

    let mut i =0;
    while i < w.len()-1 {
        if w[i+1] - w[i] >= 2{
            if w[i]+1 == w[i+1]-1{
                res.push((w[i]+1).to_string());
            }else{
                res.push((w[i]+1).to_string()+"->"+&(w[i+1]-1).to_string())
            }
        }
        i+=1;
    }
    return res;
}
fn main() {

    let res = find_missing_ranges([0,1,3,50,75].to_vec(),0,99);
    println!("{:?}",res);
    let res = find_missing_ranges([].to_vec(),1,1);
    println!("{:?}",res);

}
