pub fn raindrops(n: u32) -> String {
    let mut ans = String::from("");
    
    if n % 3 == 0 {
        ans.push_str("Pling");
    } 
    if n % 5 == 0 {
        ans.push_str("Plang");
    } 
    if n % 7 == 0 {
        ans.push_str("Plong");
    }
    if n % 3 != 0 && n % 5 != 0 && n % 7 != 0 {
        return n.to_string();
    }
    ans
}
