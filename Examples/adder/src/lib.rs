pub fn adding(a:i32,b:i32) -> i32{
    return a+b;
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = adding(5,2);
        assert_eq!(result, 7    );
    }
}
