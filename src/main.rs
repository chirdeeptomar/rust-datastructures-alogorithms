pub mod algorithm;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_bubble_sort() {
        let mut v = vec![38, 32, 1, 8, 819, 122];
        algorithm::sorting::bubble_sort(&mut v);
        assert_eq!(v, vec![1, 8, 32, 38, 122, 819])
    }
}
