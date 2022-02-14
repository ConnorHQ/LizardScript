#[cfg(test)]
mod tests {
    //use super::*;
    #[derive(Debug, PartialEq)]
    pub struct Number(pub i32);

    #[derive(Debug, PartialEq)]
    pub enum Op {
        Add,
        Sub,
        Mul,
        Div,
    }

    impl Op {
        pub fn new(s: &str) -> Self {
            match s {
                "+" => Self::Add,
                "-" => Self::Sub,
                "*" => Self::Mul,
                "/" => Self::Div,
                _ => panic!("ERROR: Bad Operator")
            }
        }
    }

    impl Number {
        pub fn new(s: &str) -> Self {
            Self(s.parse().unwrap())
        }
    }
    #[test]
    fn parse_num() {
        assert_eq!(Number::new("123"), Number(123))
    }
    #[test]
    fn parse_add_op() {
        assert_eq!(Op::new("+"), Op::Add);
    }

    #[test]
    fn parse_sub_op() {
        assert_eq!(Op::new("-"), Op::Sub);
    }
}
