use std::collections::VecDeque;
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

#[derive(Clone, Debug)]
pub enum Token {
    Val(i32),
    Seq(Vec<String>),
}

pub fn nget(stack: &mut Vec<Token>) -> Result<(), String> {
    if stack.len() < 2 {
        return Err("Not enough values to nget".to_string());
    }
    let v1_tok = stack.pop().unwrap();
    let v1 = match v1_tok {
        Token::Val(v) => v,
        Token::Seq(_) => {
            return Err("must not be seq".to_string());
        }
    };

    if let Some(val) = stack.get(stack.len() - v1 as usize) {
        stack.push(val.clone());
    } else {
        return Err(format!("in nget indexing is not valid: {}", v1).to_string());
    };

    Ok(())
}

pub fn eq(stack: &mut Vec<Token>) -> Result<(), String> {
    if stack.len() < 2 {
        return Err("Not enough values to eq".to_string());
    }
    let v1_tok = stack.pop().unwrap();
    let v1 = match v1_tok {
        Token::Val(v) => v,
        Token::Seq(_) => {
            return Err("must not be seq".to_string());
        }
    };
    let v2_tok = stack.pop().unwrap();
    let v2 = match v2_tok {
        Token::Val(v) => v,
        Token::Seq(_) => {
            return Err("must not be seq".to_string());
        }
    };
    if v2 == v1 {
        stack.push(Token::Val(1));
    } else {
        stack.push(Token::Val(0));
    }

    Ok(())
}

pub fn gt(stack: &mut Vec<Token>) -> Result<(), String> {
    if stack.len() < 2 {
        return Err("Not enough values to gt".to_string());
    }
    let v1_tok = stack.pop().unwrap();
    let v1 = match v1_tok {
        Token::Val(v) => v,
        Token::Seq(_) => {
            return Err("must not be seq".to_string());
        }
    };
    let v2_tok = stack.pop().unwrap();
    let v2 = match v2_tok {
        Token::Val(v) => v,
        Token::Seq(_) => {
            return Err("must not be seq".to_string());
        }
    };
    if v2 > v1 {
        stack.push(Token::Val(1));
    } else {
        stack.push(Token::Val(0));
    }

    Ok(())
}

pub fn lt(stack: &mut Vec<Token>) -> Result<(), String> {
    if stack.len() < 2 {
        return Err("Not enough values to lt".to_string());
    }
    let v1_tok = stack.pop().unwrap();
    let v1 = match v1_tok {
        Token::Val(v) => v,
        Token::Seq(_) => {
            return Err("must not be seq".to_string());
        }
    };
    let v2_tok = stack.pop().unwrap();
    let v2 = match v2_tok {
        Token::Val(v) => v,
        Token::Seq(_) => {
            return Err("must not be seq".to_string());
        }
    };
    if v2 < v1 {
        stack.push(Token::Val(1));
    } else {
        stack.push(Token::Val(0));
    }

    Ok(())
}

pub fn sel(stack: &mut Vec<Token>) -> Result<(), String> {
    if stack.len() < 3 {
        return Err("Not enough values to swap".to_string());
    }
    let v1_tok = stack.pop().unwrap();
    let v1 = match v1_tok {
        Token::Val(v) => v,
        Token::Seq(_) => {
            return Err("must not be seq".to_string());
        }
    };
    let v2_tok = stack.pop().unwrap();
    let v2 = match v2_tok {
        Token::Val(v) => v,
        Token::Seq(_) => {
            return Err("must not be seq".to_string());
        }
    };
    let v3_tok = stack.pop().unwrap();
    let v3 = match v3_tok {
        Token::Val(v) => v,
        Token::Seq(_) => {
            return Err("must not be seq".to_string());
        }
    };
    if v3 == 0 {
        stack.push(Token::Val(v1));
    } else {
        stack.push(Token::Val(v2));
    }
    Ok(())
}

pub fn swap(stack: &mut Vec<Token>) -> Result<(), String> {
    if stack.len() < 2 {
        return Err("Not enough values to swap".to_string());
    }
    let last = stack.len() - 1;
    let last2 = stack.len() - 2;
    stack.swap(last, last2);
    Ok(())
}

pub fn pop(stack: &mut Vec<Token>) -> Result<(), String> {
    if stack.len() < 1 {
        return Err("Not enough values to pop".to_string());
    }
    stack.pop();
    Ok(())
}

pub fn add(stack: &mut Vec<Token>) -> Result<(), String> {
    if stack.len() < 2 {
        return Err("Not enough values to add".to_string());
    }
    let v1_tok = stack.pop().unwrap();
    let v1 = match v1_tok {
        Token::Val(v) => v,
        Token::Seq(_) => {
            return Err("must not be seq".to_string());
        }
    };
    let v2_tok = stack.pop().unwrap();
    let v2 = match v2_tok {
        Token::Val(v) => v,
        Token::Seq(_) => {
            return Err("must not be seq".to_string());
        }
    };
    stack.push(Token::Val(v2 + v1));
    Ok(())
}

pub fn sub(stack: &mut Vec<Token>) -> Result<(), String> {
    if stack.len() < 2 {
        return Err("Not enough values to sub".to_string());
    }
    let v1_tok = stack.pop().unwrap();
    let v1 = match v1_tok {
        Token::Val(v) => v,
        Token::Seq(_) => {
            return Err("must not be seq".to_string());
        }
    };
    let v2_tok = stack.pop().unwrap();
    let v2 = match v2_tok {
        Token::Val(v) => v,
        Token::Seq(_) => {
            return Err("must not be seq".to_string());
        }
    };
    stack.push(Token::Val(v2 - v1));
    Ok(())
}

pub fn mul(stack: &mut Vec<Token>) -> Result<(), String> {
    if stack.len() < 2 {
        return Err("Not enough values to mul".to_string());
    }
    let v1_tok = stack.pop().unwrap();
    let v1 = match v1_tok {
        Token::Val(v) => v,
        Token::Seq(_) => {
            return Err("must not be seq".to_string());
        }
    };
    let v2_tok = stack.pop().unwrap();
    let v2 = match v2_tok {
        Token::Val(v) => v,
        Token::Seq(_) => {
            return Err("must not be seq".to_string());
        }
    };
    stack.push(Token::Val(v2 * v1));
    Ok(())
}

pub fn div(stack: &mut Vec<Token>) -> Result<(), String> {
    if stack.len() < 2 {
        return Err("Not enough values to div".to_string());
    }
    let v1_tok = stack.pop().unwrap();
    let v1 = match v1_tok {
        Token::Val(v) => v,
        Token::Seq(_) => {
            return Err("must not be seq".to_string());
        }
    };
    let v2_tok = stack.pop().unwrap();
    let v2 = match v2_tok {
        Token::Val(v) => v,
        Token::Seq(_) => {
            return Err("must not be seq".to_string());
        }
    };
    stack.push(Token::Val(v2 / v1));
    Ok(())
}

pub fn rem(stack: &mut Vec<Token>) -> Result<(), String> {
    if stack.len() < 2 {
        return Err("Not enough values to rem".to_string());
    }
    let v1_tok = stack.pop().unwrap();
    let v1 = match v1_tok {
        Token::Val(v) => v,
        Token::Seq(_) => {
            return Err("must not be seq".to_string());
        }
    };
    let v2_tok = stack.pop().unwrap();
    let v2 = match v2_tok {
        Token::Val(v) => v,
        Token::Seq(_) => {
            return Err("must not be seq".to_string());
        }
    };
    stack.push(Token::Val(v2 % v1));
    Ok(())
}

pub fn exec(splitted: &mut VecDeque<String>, stack: &mut Vec<Token>) -> Result<(), String> {
    match stack.pop().unwrap() {
        Token::Val(_) => return Err("must be seq".to_string()),
        Token::Seq(seq) => splitted.append(&mut VecDeque::from(seq)),
    }
    Ok(())
}

fn create_seq(splitted: &mut VecDeque<String>) -> Result<Vec<String>, String> {
    let mut a: Vec<String> = Vec::new();
    loop {
        let b = splitted.pop_front().unwrap();
        if b == ")" {
            break;
        }
        a.push(b);
    }
    Ok(a)
}

fn _parse(splitted: &mut VecDeque<String>, stack: &mut Vec<Token>) -> Result<(), String> {
    while splitted.len() > 0 {
        let elem = match splitted.pop_front() {
            None => {
                break;
            }
            Some(el) => el,
        };
        if let Ok(val) = elem.parse::<i32>() {
            stack.push(Token::Val(val));
        } else {
            match elem.as_str() {
                "add" => add(stack)?,
                "sub" => sub(stack)?,
                "mul" => mul(stack)?,
                "div" => div(stack)?,
                "rem" => rem(stack)?,
                "swap" => swap(stack)?,
                "pop" => pop(stack)?,
                "eq" => eq(stack)?,
                "lt" => lt(stack)?,
                "gt" => gt(stack)?,
                "nget" => nget(stack)?,
                "exec" => exec(splitted, stack)?,
                "(" => {
                    let a = create_seq(splitted)?;
                    stack.push(Token::Seq(a));
                }
                _ => panic!("{}", elem),
            }
        }
    }
    Ok(())
}

pub fn parse(command: String, mut args: Vec<i32>) -> Result<i32, String> {
    let mut stack: Vec<Token> = vec![];
    let mut splitted = command
        .split(" ")
        .map(|x| x.to_string())
        .collect::<VecDeque<String>>();
    if splitted[0] != "rostfix".to_string() {
        return Err("A code must start with the constant 'rostfix'.".to_string());
    }
    if splitted[1].parse::<usize>().unwrap() != args.len() {
        return Err("The number of parameters is not correct.".to_string());
    }
    splitted.pop_front();
    splitted.pop_front();
    for _ in 0..args.len() {
        stack.push(Token::Val(args.pop().unwrap()));
    }
    _parse(&mut splitted, &mut stack)?;
    match stack.last().unwrap().clone() {
        Token::Val(v) => return Ok(v),
        Token::Seq(_) => return Err("error".to_string()),
    }
}

#[cfg(test)]
mod tests_with_brackets {
    use super::parse;

    #[test]
    fn test_bracket() {
        let res = parse("rostfix 1 ( 2 mul ) exec".to_string(), vec![7]);
        assert_eq!(res, Ok(14));

        let res = parse("rostfix 0 ( 0 swap sub ) 7 swap exec".to_string(), vec![]);
        assert_eq!(res, Ok(-7));
    }
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn test_err() {
        let res = parse("postfi 0 1 2 3".to_string(), vec![]);
        assert_eq!(res, Err("invalid syntax".to_string()));
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
