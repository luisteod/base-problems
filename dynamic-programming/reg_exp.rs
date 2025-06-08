/*
status: done
notes:
*/

fn is_match(s: String, p: String) -> bool {
    let s_vec: Vec<_> = s.chars().collect();
    let p_vec: Vec<_> = p.chars().collect();
    let s_len = s_vec.len();
    let p_len = p_vec.len();

    // +1 for empty string
    let mut dp = vec![vec![false; p_len + 1]; s_len + 1];

    dp[0][0] = true;

    // Handle the case for empty string
    for j in 1..=p_len {
        if p_vec[j - 1] == '*' {
            if j >= 2 {
                dp[0][j] = dp[0][j - 2];
            }
        }
    }

    for i in 1..=s_len {
        for j in 1..=p_len {
            if s_vec[i - 1] == p_vec[j - 1] || p_vec[j - 1] == '.' {
                dp[i][j] = dp[i - 1][j - 1];
            }
            else if p_vec[j - 1] == '*' {
                dp[i][j] = dp[i][j - 2];
                if s_vec[i - 1] == p_vec[j - 2] || p_vec[j - 2] == '.' {
                    dp[i][j] = dp[i][j] || dp[i - 1][j];
                }
            }
        }
    }

    return dp[s_len][p_len];
}

fn main() {
    let s = "aa".to_string();
    let p = "a*".to_string();
    let result = is_match(s, p);
    println!(
        "
        \r#######################################
        \rResult : {}
        \r#######################################
        ", result
    );
}