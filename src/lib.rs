pub fn add_two(a: i32) -> i32 {
    a + 2
}

fn sub_two(a: u32) -> u32 {
    a - 2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add_two(2), 4);
    }

    #[test]
    fn another() {
        assert_ne!(add_two(3), 4)
    }

    #[test]
    #[should_panic]
    fn other() {
        sub_two(1);
    }
}
