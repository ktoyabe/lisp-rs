use std::{cell::RefCell, rc::Rc};

use crate::{env::Env, object::Object, parser::parse};

fn eval_binary_op(list: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    if list.len() != 3 {
        return Err(format!("Invalid number of arguments for binary operator"));
    }
    let op = list[0].clone();
    let left = eval_obj(&list[1].clone(), env)?;
    let right = eval_obj(&list[2].clone(), env)?;

    let left_val = match left {
        Object::Integer(n) => n,
        _ => return Err(format!("Left operand must be an integer {:?}", left)),
    };
    let right_val = match right {
        Object::Integer(n) => n,
        _ => return Err(format!("Right operand must be an integer {:?}", right)),
    };

    match op {
        Object::Symbol(s) => match s.as_str() {
            "+" => Ok(Object::Integer(left_val + right_val)),
            "-" => Ok(Object::Integer(left_val - right_val)),
            "*" => Ok(Object::Integer(left_val * right_val)),
            "/" => Ok(Object::Integer(left_val / right_val)),
            "=" => Ok(Object::Bool(left_val == right_val)),
            "!=" => Ok(Object::Bool(left_val != right_val)),
            "<" => Ok(Object::Bool(left_val < right_val)),
            ">" => Ok(Object::Bool(left_val > right_val)),
            _ => Err(format!("Invalid binary operator: {}", s)),
        }
        _ => Err(format!("Operator must be symbol. op={:?}", op)),
    }
}

fn eval_list(list: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    let head = &list[0];
    match head {
        Object::Symbol(s) => match s.as_str() {
            "+" | "-" | "*" | "/" | "=" | "!=" | "<" | ">" => {
                return eval_binary_op(&list, env);
            }
            "define" => {
                return eval_define(&list, env);
            }
            _ => {
                return Err(format!("eval_list: Unsupported operator. op={}", head))
            }
        }
        _ => {
            let mut new_list = Vec::new();
            for obj in list {
                let result = eval_obj(obj, env)?;
                match result {
                    Object::Void => {},
                    _ => new_list.push(result),
                }
            }
            return Ok(Object::List(new_list))
        }
    }
}

fn eval_define(list: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    if list.len() != 3 {
        return Err(format!("Invalid number of arguments for define"))
    }

    let symbol = match &list[1] {
        Object::Symbol(s) => s.clone(),
        _ => return Err(format!("Invalid define")),
    };

    let val = eval_obj(&list[2], env)?;
    env.borrow_mut().set(&symbol, val);
    Ok(Object::Void)
}

fn eval_obj(obj: &Object, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    match obj {
        Object::List(list) => eval_list(list, env),
        Object::Void => Ok(Object::Void),
        // Object::Lambda(_params, _body) => Ok(Object::Void),
        Object::Bool(_) => Ok(obj.clone()),
        Object::Integer(n) => Ok(Object::Integer(*n)),
        Object::Symbol(s) => eval_symbol(s, env),
        _ => Err(format!("eval_obj: unsupported object. obj={:?}", obj)),
    }
}

fn eval_symbol(symbol: &str, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    let val = env.borrow_mut().get(symbol);
    if val.is_none() {
        return Err(format!("Undefined symbol: {}", symbol));
    }

    Ok(val.unwrap().clone())
}

pub fn eval(program: &str, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    let parsed_list = parse(program);
    if parsed_list.is_err() {
        return Err(format!("{}", parsed_list.err().unwrap()));
    }
    eval_obj(&parsed_list.unwrap(), env)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_op_add() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let result = eval("(+ 1 2)", &mut env).unwrap();
        assert_eq!(result, Object::Integer(3));
    }

    #[test]
    fn test_binary_op_sub() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let result = eval("(- 1 2)", &mut env).unwrap();
        assert_eq!(result, Object::Integer(-1));
    }

    #[test]
    fn test_binary_op_mul() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let result = eval("(* 2 3)", &mut env).unwrap();
        assert_eq!(result, Object::Integer(6));
    }

    #[test]
    fn test_binary_op_div() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let result = eval("(/ 5 2)", &mut env).unwrap();
        assert_eq!(result, Object::Integer(2));
    }

    #[test]
    fn test_binary_op_equal() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        assert_eq!(eval("(= 5 2)", &mut env).unwrap(), Object::Bool(false));
        assert_eq!(eval("(= 2 2)", &mut env).unwrap(), Object::Bool(true));
    }

    #[test]
    fn test_binary_op_not_equal() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        assert_eq!(eval("(!= 5 2)", &mut env).unwrap(), Object::Bool(true));
        assert_eq!(eval("(!= 5 5)", &mut env).unwrap(), Object::Bool(false));
    }

    #[test]
    fn test_binary_op_less() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        assert_eq!(eval("(< 2 3)", &mut env).unwrap(), Object::Bool(true));
        assert_eq!(eval("(< 3 3)", &mut env).unwrap(), Object::Bool(false));
        assert_eq!(eval("(< 4 3)", &mut env).unwrap(), Object::Bool(false));
    }

    #[test]
    fn test_binary_op_greater() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let result = eval("(/ 5 2)", &mut env).unwrap();
        assert_eq!(eval("(> 2 3)", &mut env).unwrap(), Object::Bool(false));
        assert_eq!(eval("(> 3 3)", &mut env).unwrap(), Object::Bool(false));
        assert_eq!(eval("(> 4 3)", &mut env).unwrap(), Object::Bool(true));
    }


    #[test]
    fn test_define() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let program = "(
            (define a 3)
            (define b 5)
            (+ a b)
        )";

        let result = eval(program, &mut env).unwrap();
        assert_eq!(result, Object::List(vec![Object::Integer(8)]));
    }

}
