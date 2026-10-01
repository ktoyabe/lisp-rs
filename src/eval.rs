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
            _ => Err(format!("Invalid binary operator: {}", s)),
        }
        _ => Err(format!("Operator must be symbol. op={:?}", op)),
    }
}

fn eval_list(list: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    let head = &list[0];
    match head {
        Object::Symbol(s) => match s.as_str() {
            "+" => {
                return eval_binary_op(&list, env);
            }
            _ => {
                return Err(format!("eval_list: Unsupported operator. op={}", head))
            }
        }
        _ => {
            return Err(format!("eval_list: Unsupported. {:?}", list));
        }
    }
}

fn eval_obj(obj: &Object, env: &mut Rc<RefCell<Env>>) -> Result<Object, String> {
    match obj {
        Object::List(list) => eval_list(list, env),
        Object::Void => Ok(Object::Void),
        // Object::Lambda(_params, _body) => Ok(Object::Void),
        // Object::Bool(_) => Ok(obj.clone()),
        Object::Integer(n) => Ok(Object::Integer(*n)),
        // Object::Symbol(s) => eval_symbol(s, env),
        _ => Err(format!("eval_obj: unsupported object. obj={:?}", obj)),
    }
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
    fn test_simple_add() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let result = eval("(+ 1 2)", &mut env).unwrap();
        assert_eq!(result, Object::Integer(3));
    }
}
