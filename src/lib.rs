use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub enum Token {
    Val(i32),
    Seq(Vec<String>),
}


pub enum Op {
    Add,
    Sub,
    Mul,
    Div
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

pub fn basic_operations(stack: &mut Vec<Token>, op: Op) -> Result<(), String> {
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
    match op {
        Op::Add => stack.push(Token::Val(v2 + v1)),
        Op::Sub => stack.push(Token::Val(v2 - v1)),
        Op::Mul => stack.push(Token::Val(v2 * v1)),
        Op::Div => stack.push(Token::Val(v2 / v1)),
    }
    Ok(())
}

pub fn add(stack: &mut Vec<Token>) -> Result<(), String> {
    basic_operations(stack, Op::Add)?;
    Ok(())
}

pub fn sub(stack: &mut Vec<Token>) -> Result<(), String> {
    basic_operations(stack, Op::Sub)?;
    Ok(())
}

pub fn mul(stack: &mut Vec<Token>) -> Result<(), String> {
    basic_operations(stack, Op::Mul)?;
    Ok(())
}

pub fn div(stack: &mut Vec<Token>) -> Result<(), String> {
    basic_operations(stack, Op::Div)?;
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
