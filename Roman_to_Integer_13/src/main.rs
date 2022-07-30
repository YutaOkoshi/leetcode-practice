use std::env;

// use std::collections::BinaryHeap;
// let Roman = BinaryHeap::new().0ush;

fn roman_to_int(s: String) -> i32 {
    use std::collections::HashMap;

    let hash = vec![
        ("I".to_owned(), 1),
        ("IV".to_owned(), 4),
        ("V".to_owned(), 5),
        ("IX".to_owned(), 9),
        ("X".to_owned(), 10),
        ("XL".to_owned(), 40),
        ("L".to_owned(), 50),
        ("XC".to_owned(), 90),
        ("C".to_owned(), 100),
        ("CD".to_owned(), 400),
        ("D".to_owned(), 500),
        ("CM".to_owned(), 900),
        ("M".to_owned(), 1000),
    ]
    .into_iter()
    .collect::<HashMap<String, i32>>();

    let v: Vec<char> = s.chars().collect();

    let mut n = 0;
    let mut res: i32 = 0;
    while n < v.len() {
        if n < v.len() - 1 {
            let s = v[n].to_string() + &v[n + 1].to_string();
            if hash.contains_key(&s) {
                res += hash[&s];
                n += 2;
                continue;
            }
        }
        let s = v[n].to_string();
        res += hash[&s];
        n += 1;
    }
    return res;
}
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let s: String = args[1].to_string();
    let res = roman_to_int(s);
    println!("{}", res);
}
