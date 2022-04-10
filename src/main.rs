pub mod algorithm;
pub mod data_structure;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::algorithm::sorting;
    use crate::data_structure::LinkedList;

    #[test]
    fn test_bubble_sort() {
        let mut v = vec![38, 32, 1, 8, 819, 122];
        sorting::bubble_sort(&mut v);
        assert_eq!(v, vec![1, 8, 32, 38, 122, 819])
    }

    #[test]
    fn test_has_linkedlist_is_empty() {
        let ll = LinkedList::<i64>::empty();

        assert_eq!(ll.size(), 0)
    }

    #[test]
    fn test_linkedlist_has_one_item() {
        let mut ll = LinkedList::<i64>::empty();
        ll.push(1);
        assert_eq!(ll.size(), 1)
    }
}
