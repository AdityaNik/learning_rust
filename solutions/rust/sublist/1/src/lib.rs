#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn is_sublist(f: &[i32], s: &[i32]) -> bool {
    if f.is_empty() {
        return true
    }

    s.windows(f.len()).any(|window| window == f)
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    let m = first_list.len();
    let n = second_list.len();
    
    if first_list == second_list {
        Comparison::Equal
    } else if is_sublist(first_list, second_list) {
        Comparison::Sublist
    } else if is_sublist(second_list, first_list) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
    
}
