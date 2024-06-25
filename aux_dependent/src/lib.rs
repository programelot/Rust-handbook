pub fn foo(i: i32) -> i32 {
    i + 10
}

// It is typical to add test for libraries.
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(11, super::foo(1));
    }
}
