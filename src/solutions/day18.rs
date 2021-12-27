use serde_json::{Result, Value};

#[derive(Clone, Eq, PartialEq)]
enum SnailNumber {
    Num(i64),
    List(Vec<SnailNumber>),
}

impl std::fmt::Debug for SnailNumber {

fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
     match self {
         SnailNumber::Num(n) => write!(f, "{:?}", n),
         SnailNumber::List(l) => write!(f, "{:?}", l)
     }
    }
}

impl SnailNumber {
    fn add(a: &SnailNumber, b: &SnailNumber) -> SnailNumber {
        SnailNumber::List(vec![a.clone(), b.clone()])
    }
    
    fn reduce(&mut self, depth: i64) {
        match self {
            SnailNumber::Num() => (),
            SnailNumber::List(l) => {
                if 
            }
        }
    }
    
    fn reduced(snail: SnailNumber) -> SnailNumber {
        let mut snail = snail;
        Self::reduce(&mut snail, 0);
        snail
    }

    fn is_number(&self) -> bool {
        match self {
            SnailNumber::Num(_) => true,
            _ => false
        }
    }
}



fn json_to_snail(json: &Value) -> SnailNumber {
    match json.as_i64() {
        Some(v) => SnailNumber::Num(v),
        None => match json.as_array() {
            Some(a) => SnailNumber::List(a.iter().map(json_to_snail).collect()),
            None => panic!("")
        }
    }
}

fn parse(line: &str) -> SnailNumber {
    json_to_snail(&serde_json::from_str(line).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        assert_eq!(SnailNumber::Num(1), parse("1"));
        assert_eq!(SnailNumber::List(vec![SnailNumber::Num(1), SnailNumber::Num(2)]), parse("[1, 2]"));
    }

    #[test]
    fn example_1() {
        assert_eq!( reduced(parse("[[[[[9,8],1],2],3],4]")), parse("[[[[0,9],2],3],4]"));
        assert_eq!( reduced(parse("[7,[6,[5,[4,[3,2]]]]]")), parse("[7,[6,[5,[7,0]]]]"));
        assert_eq!( reduced(parse("[[6,[5,[4,[3,2]]]],1]")), parse("[[6,[5,[7,0]]],3]"));
        assert_eq!( reduced(parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]")), parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"));
        assert_eq!( reduced(parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")), parse("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"));
    }
}
