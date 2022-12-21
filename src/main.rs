fn main() {
}

pub fn nget(stack: &mut Vec<i32>) -> Result<(), String> {
    if stack.len() < 2 {
        return Err("Not enough values to nget".to_string());
    }
    let v1 = stack.pop().unwrap();

    if let Some(val) = stack.get(stack.len() - v1 as usize) {
        stack.push(*val);
    } else {
        return Err(format!("in nget indexing is not valid: {}", v1).to_string());
    };

    Ok(())
}

pub fn eq(stack: &mut Vec<i32>) -> Result<(), String> {
    if stack.len() < 2 {
        return Err("Not enough values to eq".to_string());
    }
    let v1 = stack.pop().unwrap();
    let v2 = stack.pop().unwrap();
    if v2 == v1 {
        stack.push(1);
    } else {
        stack.push(0);
    }

    Ok(())
}

pub fn gt(stack: &mut Vec<i32>) -> Result<(), String> {
    if stack.len() < 2 {
        return Err("Not enough values to gt".to_string());
    }
    let v1 = stack.pop().unwrap();
    let v2 = stack.pop().unwrap();
    if v2 > v1 {
        stack.push(1);
    } else {
        stack.push(0);
    }

    Ok(())
}

pub fn lt(stack: &mut Vec<i32>) -> Result<(), String> {
    if stack.len() < 2 {
        return Err("Not enough values to lt".to_string());
    }
    let v1 = stack.pop().unwrap();
    let v2 = stack.pop().unwrap();
    if v2 < v1 {
        stack.push(1);
    } else {
        stack.push(0);
    }

    Ok(())
}

pub fn sel(stack: &mut Vec<i32>) -> Result<(), String> {
    if stack.len() < 3 {
        return Err("Not enough values to swap".to_string());
    }
    let v1 = stack.pop().unwrap();
    let v2 = stack.pop().unwrap();
    let v3 = stack.pop().unwrap();
    if v3 == 0 {
        stack.push(v1);
    } else {
        stack.push(v2);
    }
    Ok(())
}

pub fn swap(stack: &mut Vec<i32>) -> Result<(), String> {
    if stack.len() < 2 {
        return Err("Not enough values to swap".to_string());
    }
    let last = stack.len() - 1;
    let last2 = stack.len() - 2;
    stack.swap(last, last2);
    Ok(())
}

pub fn pop(stack: &mut Vec<i32>) -> Result<(), String> {
    if stack.len() < 1 {
        return Err("Not enough values to pop".to_string());
    }
    stack.pop();
    Ok(())
}

pub fn add(stack: &mut Vec<i32>) -> Result<(), String> {
    if stack.len() < 2 {
        return Err("Not enough values to add".to_string());
    }
    let v1 = stack.pop().unwrap();
    let v2 = stack.pop().unwrap();
    stack.push(v2 + v1);
    Ok(())
}

pub fn sub(stack: &mut Vec<i32>) -> Result<(), String> {
    if stack.len() < 2 {
        return Err("Not enough values to sub".to_string());
    }
    let v1 = stack.pop().unwrap();
    let v2 = stack.pop().unwrap();
    stack.push(v2 - v1);
    Ok(())
}

pub fn mul(stack: &mut Vec<i32>) -> Result<(), String> {
    if stack.len() < 2 {
        return Err("Not enough values to mul".to_string());
    }
    let v1 = stack.pop().unwrap();
    let v2 = stack.pop().unwrap();
    stack.push(v2 * v1);
    Ok(())
}

pub fn div(stack: &mut Vec<i32>) -> Result<(), String> {
    if stack.len() < 2 {
        return Err("Not enough values to div".to_string());
    }
    let v1 = stack.pop().unwrap();
    let v2 = stack.pop().unwrap();
    stack.push(v2 / v1);
    Ok(())
}

pub fn rem(stack: &mut Vec<i32>) -> Result<(), String> {
    if stack.len() < 2 {
        return Err("Not enough values to rem".to_string());
    }
    let v1 = stack.pop().unwrap();
    let v2 = stack.pop().unwrap();
    stack.push(v2 % v1);
    Ok(())
}

pub fn parse(command: String, mut args: Vec<i32>) -> Result<i32, String> {
    let mut stack: Vec<i32> = vec![];
    let mut splitted = command
        .split(" ")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    if splitted[0] != "postfix".to_string() {
        return Err("invalid syntax".to_string());
    }
    if splitted[1].parse::<usize>().unwrap() != args.len() {
        return Err("invalid syntax".to_string());
    }
    splitted.remove(0);
    splitted.remove(0);
    for _ in 0..args.len() {
        stack.push(args.pop().unwrap());
    }
    for elem in splitted.iter() {
        if let Ok(val) = elem.parse::<i32>() {
            stack.push(val);
        } else {
            match elem.as_str() {
                "add" => add(&mut stack)?,
                "sub" => sub(&mut stack)?,
                "mul" => mul(&mut stack)?,
                "div" => div(&mut stack)?,
                "rem" => rem(&mut stack)?,
                "swap" => swap(&mut stack)?,
                "pop" => pop(&mut stack)?,
                "eq" => eq(&mut stack)?,
                "lt" => lt(&mut stack)?,
                "gt" => gt(&mut stack)?,
                "nget" => nget(&mut stack)?,
                _ => panic!("{}", elem),
            }
        }
    }
    Ok(stack.last().unwrap().clone())
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
        let res = parse("postfix 1 4 add 5 mul 6 sub 7 div".to_string(), vec![3]);
        assert_eq!(res, Ok(4));
        let res = parse(
            "postfix 5 add mul sub swap div".to_string(),
            vec![7, 6, 5, 4, 3],
        );
        assert_eq!(res, Ok(-20));
        let res = parse("postfix 3 4000 swap pop add".to_string(), vec![300, 20, 1]);
        assert_eq!(res, Ok(4020));
        let res = parse("postfix 1 4 lt 10 add".to_string(), vec![3]);
        assert_eq!(res, Ok(11));
    }

    #[test]
    fn test_nget() {
        let res = parse("postfix 1 1 nget mul".to_string(), vec![5]);
        assert_eq!(res, Ok(25));
        let res = parse("postfix 2 1 nget".to_string(), vec![4, 5]);
        assert_eq!(res, Ok(4));
        let res = parse("postfix 4 4 nget 5 nget mul mul swap 4 nget mul add add".to_string(), vec![3, 4, 5, 2]);
        assert_eq!(res, Ok(25));
    }

    #[test]
    fn test_add_without_args() {
        let res = parse("postfix 0 1 3 add".to_string(), vec![]);
        assert_eq!(res, Ok(4));
    }

    #[test]
    fn test_add_with_args() {
        let res = parse("postfix 0 1 3 add".to_string(), vec![]);
        assert_eq!(res, Ok(4));
    }

    #[test]
    fn test_sub_without_args() {
        let res = parse("postfix 0 1 4 sub".to_string(), vec![]);
        assert_eq!(res, Ok(-3));
    }
}
