

pub fn closure_time (loops: u32) -> i32 {
    let mut x = 0; 
    let mut increment = || { x += 1};

    let mut triple = || {
        increment();
        increment();
        increment();
    };

    let mut count = 0;
    while count < loops {
        triple();
        count += 1;
    }

    x.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count() {
        assert_eq!(6, closure_time(2));
    }
}