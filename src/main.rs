use rostfix::parse;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents =
        fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file");
    let res = parse(
        contents,
        args[2..]
            .to_vec()
            .iter()
            .map(|a| a.parse::<i32>().unwrap())
            .collect(),
    );
    match res {
        Ok(a) => println!("{}", a),
        Err(b) => println!("{}", b),
    }
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn test_err() {
        let res = parse("postfi 0 1 2 3".to_string(), vec![]);
        assert_eq!(
            res,
            Err("A code must start with the constant 'rostfix'.".to_string())
        );
    }

    #[test]
    fn test_calc() {
        let res = parse("rostfix 1 4 add 5 mul 6 sub 7 div".to_string(), vec![3]);
        assert_eq!(res, Ok(4));
        let res = parse(
            "rostfix 5 add mul sub swap div".to_string(),
            vec![7, 6, 5, 4, 3],
        );
        assert_eq!(res, Ok(-20));
        let res = parse("rostfix 3 4000 swap pop add".to_string(), vec![300, 20, 1]);
        assert_eq!(res, Ok(4020));
        let res = parse("rostfix 1 4 lt 10 add".to_string(), vec![3]);
        assert_eq!(res, Ok(11));
    }

    #[test]
    fn test_nget() {
        let res = parse("rostfix 1 1 nget mul".to_string(), vec![5]);
        assert_eq!(res, Ok(25));
        let res = parse("rostfix 2 1 nget".to_string(), vec![4, 5]);
        assert_eq!(res, Ok(4));
        let res = parse(
            "rostfix 4 4 nget 5 nget mul mul swap 4 nget mul add add".to_string(),
            vec![3, 4, 5, 2],
        );
        assert_eq!(res, Ok(25));
    }

    #[test]
    fn test_add_without_args() {
        let res = parse("rostfix 0 1 3 add".to_string(), vec![]);
        assert_eq!(res, Ok(4));
    }

    #[test]
    fn test_no_calc() {
        let res = parse("rostfix 0 1 3 7".to_string(), vec![]);
        assert_eq!(res, Ok(7));
    }

    #[test]
    fn test_add_with_args() {
        let res = parse("rostfix 1 3 add".to_string(), vec![1]);
        assert_eq!(res, Ok(4));
    }

    #[test]
    fn test_sub_without_args() {
        let res = parse("rostfix 0 1 4 sub".to_string(), vec![]);
        assert_eq!(res, Ok(-3));
    }
}
