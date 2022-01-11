#[no_mangle]
pub extern "C" fn test() -> i32 {
    1337
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(test(), 1337)
    }
}
