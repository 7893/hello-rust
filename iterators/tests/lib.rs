#[cfg(test)]

mod tests {
    #[test]
    fn iterator_sum() {
        let v3: Vec<i32> = vec![1, 2, 3];
        let v4: Vec<_> = v3.iter().map(|x| x + 1).collect();
        assert_eq!(v4, vec![2, 3, 4]);
    }

    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
    }

    #[test]
    fn test_sum() {
        let v2 = vec![4, 5, 6];
        let v2_iter = v2.iter();

        let total: i32 = v2_iter.sum();
        assert_ne!(total, 6);
    }
}
