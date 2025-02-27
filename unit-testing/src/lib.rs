// pub fn add(left: u64, right: u64) -> u64 {
//     left + right + right - right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }

//     #[test]
//     fn adding_equal_numbers_works() {
//         assert_eq!(add(2, 2), 4);
//     }

//     #[test]
//     fn adding_unequal_numbers_works() {
//         assert_eq!(add(2, 3), 5);
//     }
// }

#[derive(Default)]
pub struct Database {
    count: u32,
}
impl Database {
    // Implement this function for testing purposes within the current crate.
    // If you need this function outside of the current crate (like in for example integration tests), use `pub` instead of `pub(crate)`.
    #[cfg(test)]
    pub(crate) fn at_10() -> Database {
        Self { count: 10 }
    }
    pub fn operate(&mut self) {
        self.count += 1;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operate_once() {
        let mut database = Database::at_10();
        database.operate();
        assert_eq!(database.count, 11);
    }

    #[test]
    fn operating_twice() {
        let mut database = Database::at_10();
        database.operate();
        database.operate();
        assert_eq!(database.count, 12);
    }
}
