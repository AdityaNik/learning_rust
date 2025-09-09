pub fn build_proverb(list: &[&str]) -> String {
    let mut ans: String = String::from("");
    if list.len() == 0 {
        return ans;
    }
    
    for i in 0..list.len() - 1 {
        let formated_string = format!("For want of a {} the {} was lost.\n", list[i], list[i+1]);
        ans.push_str(&formated_string);
    }
    let formated_str = format!("And all for the want of a {}.", list[0]);
    ans.push_str(&formated_str);
    ans
}
