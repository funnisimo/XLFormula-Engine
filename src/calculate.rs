use crate::types;

fn calculate_divide_operator(num1: f32, num2: f32) -> f32 {
    num1 / num2
}

fn is_float_int(num: f32) -> bool {
    ((num as i32) as f32) == num
}

fn calculate_power_operator(num1: f32, num2: f32) -> f32 {
    if is_float_int(num2) {
        num1.powi(num2 as i32)
    } else {
        num1.powf(num2)
    }
}

fn cast_value_to_number(value: types::Value) -> Option<f32> {
    match value {
        types::Value::Number(number) => Some(number),
        types::Value::Text(_) => None,
        types::Value::Error(_) => None,
    }
}

fn option_map2<T, U, V>(a: Option<T>, b: Option<U>, f: fn(a1: T, b1: U) -> V) -> Option<V> {
    if let (Some(value_a), Some(value_b)) = (a, b) {
        Some(f(value_a, value_b))
    } else {
        None
    }
}
fn calculate_numeric_operator(
    lhs: types::Value,
    rhs: types::Value,
    f: fn(num1: f32, num2: f32) -> f32,
) -> types::Value {
    let l = cast_value_to_number(lhs);
    let r = cast_value_to_number(rhs);
    match option_map2(l, r, f) {
        Some(result) => types::Value::Number(result),
        None => types::Value::Error(String::from("Error")),
    }
}

pub fn calculate_formula(formula: types::Formula) -> types::Value {
    match formula {
        types::Formula::Operation(exp) => {
            let value1 = calculate_formula(*exp.lhs);
            let value2 = calculate_formula(*exp.rhs);

            match exp.op {
                types::Operator::Plus => {
                    calculate_numeric_operator(value1, value2, |n1, n2| n1 + n2)
                }
                types::Operator::Minus => {
                    calculate_numeric_operator(value1, value2, |n1, n2| n1 - n2)
                }
                types::Operator::Multiply => {
                    calculate_numeric_operator(value1, value2, |n1, n2| n1 * n2)
                }
                types::Operator::Divide => match value2 {
                    types::Value::Number(x) if x == 0.0 => {
                        types::Value::Error(String::from("#DIV/0!"))
                    }
                    _ => calculate_numeric_operator(value1, value2, calculate_divide_operator),
                },
                types::Operator::Power => {
                    calculate_numeric_operator(value1, value2, calculate_power_operator)
                }
                types::Operator::Null => types::Value::Error(String::from("Error")),
            }
        }
        types::Formula::Value(val) => val,
    }
}

pub fn result_to_string(_value: types::Value) -> String {
    match _value {
        types::Value::Number(number) => number.to_string(),
        types::Value::Text(text) => text,
        types::Value::Error(error) => error, // String::from("Error: "),
    }
}
