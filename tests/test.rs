mod test {
    //
    #[test]
    fn test_size_of() {
        assert_eq!(std::mem::size_of::<memx::RangeError>(), 0);
    }
}
