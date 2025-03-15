use std::cmp::max;

fn cut_rod(p: &Vec<i32>, n: usize) -> i32 {
    if n == 0 {
        return 0;
    }
    
    let mut q = i32::MIN;
    
    for i in 0..n {
        let new_val = p[i] + cut_rod(&p, n - i - 1);
        // dbg!(new_val);
        q = max(q, new_val);
    }
    return q;
}

fn main() {

    let p = vec![
        1, 5, 8, 
        10, 17, 17, 
        20, 24, 30,
        30, 33, 34, 
        40, 42, 45, 
        50, 50, 54, 
        60, 61, 62,  
        70, 72, 73,
        80, 81, 82,
        90, 91, 92
    ];
    
    let n = p.len();
    
    let res = cut_rod(&p, n);
    println!("result : {}", res)
}
