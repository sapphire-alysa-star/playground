use std::collections::btree_map::ValuesMut;

pub fn append_string(vec: Vec<String>, add_on: String) -> Vec<String> {
    vec.iter().map(|x| format!("{}{}", x, add_on)).collect()
}

pub fn boolean_map() {
    let i = 10;
    let val = 0 <= i;
    println!("{}", val);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_test() {
        let vec = vec!["Yolo".to_string(), "Wizzy".to_string(), "Lotus".to_string()];
        let new_vec = append_string(vec, " Cash Money".to_string());

        assert!(new_vec[0] == "Yolo Cash Money");
    }
}