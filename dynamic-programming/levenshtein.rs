/*
status: in-progress
notes:
    Dynamic Programming table for levenshtein algorithm example

    //      r   s   y   u   s   q   t
    //  r   1   1   1   1   1   1   1
    //  u   1   1   1   2   2   2   2
    //  s   1   1   1   2   3   3   3
    //  t   1   1   1   2   3   3   4

    Distance : 3
    Explanation: This two strings has an embedded longest sequence of 4 elements "rust" that we obtained using the levenshtein algorithm, so to compute the distance subtract this value with the length of the longest string compared, in this case is "rsyusqt" wich has 7 elements, so 7 - 4 = 3.
*/

fn levenshtein(src: &str, target: &str) -> u32 {
    return 3;
}

fn main() {
    println!("Here is my Levenshtein Implementation");

    let src = "rust";
    let target = "rsyusqt";

    levenshtein(src, target);

}