fn main() {
    println!("Hello, world!");
}

fn return_an_int(a: i32) -> i32 {
    a + 10
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn return_an_int_t() {
        assert_eq!(20, return_an_int(10));
    }
}
