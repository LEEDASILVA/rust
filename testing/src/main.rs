struct Rectangle {
    width: u8,
    height: u8
}

impl Rectangle {
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}
// using cargo for testing
fn main() {}

fn get_two() -> i32 {
    2
}

// this will prevent the module tests to compile
// we do not need to compile the tests
#[cfg(test)]
mod tests {
    // saying cargo that the fnctions will be tests

    
    #[test]
    fn test_basic() {
        // using assert
        assert!(1 == 1); // OK
        // panic!("snap!!"); // will fail the hole test
    }

    // test should fail/panic
    #[test]
    #[should_panic]
    fn test_should_fail() {
        assert!(false);
        panic!("snapp!!");
    }

    #[test]
    // assert equal or assert not equal
    fn test_equals() {
        // the supper will access the outer scope
        assert_eq!(super::get_two(), 1 + 1);
        assert_ne!(super::get_two(), 1 + 2);
    }

    // #[ignore] //-> to ignore the test you dont want so use

    #[test]
    fn test_structs() {
        let r = super::Rectangle {
            width: 50,
            height: 50
        };
        // should fail
        assert!(r.is_square());
    }
}