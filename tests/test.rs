mod test {
    //
    #[test]
    fn test_size_of() {
        assert_eq!(std::mem::size_of::<memx::RangeError>(), 0);
        //
        #[cfg(target_pointer_width = "64")]
        {
            assert_eq!(std::mem::size_of::<memx::iter::MemchrIter>(), 32);
            assert_eq!(std::mem::size_of::<memx::iter::MemrchrIter>(), 32);
        }
        #[cfg(target_pointer_width = "32")]
        {
            assert_eq!(std::mem::size_of::<memx::iter::MemchrIter>(), 16);
            assert_eq!(std::mem::size_of::<memx::iter::MemrchrIter>(), 16);
        }
    }
}
