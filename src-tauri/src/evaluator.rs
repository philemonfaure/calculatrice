use std::collections::{VecDeque, HashMap};
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref VARIABLES: Mutex<HashMap<String, f64>> = Mutex::new(HashMap::new());
}

pub fn compute(content: &str) -> String
{
    let uniform_content = content.replace("</div><div>", "<div>").replace("</div>", "<div>");
    let line_array: Vec<String> = uniform_content.split("<div>").map(|s| s.to_string()).collect();

    let mut output= String::from("");
    let mut value: String;
    for line in line_array
    {
        value = compute_line(&line);
        output.push_str(&value);
        output.push_str("<br>");
    }
    format!("{}", output)
}
fn compute_line(expression: &str) -> String
{
    if let Some((var_name, expr)) = parse_variable_declaration(expression)
    {
        let tokens = tokenize(expr);
        match tokens
        {
            Ok(tokens) => {
                let rpn = shunting_yard(&tokens);
                match rpn
                {
                    Ok(rpn) => {
                        let result = evaluate_rpn(&rpn);
                        match result
                        {
                            Ok(v) => {
                                let mut vars = VARIABLES.lock().unwrap();
                                vars.insert(var_name.to_string(), v);
                                v.to_string()
                            },
                            Err(_) => "".to_string(),
                        }
                    },
                    Err(_) => "".to_string(),
                }
            },
            Err(_) => "".to_string(),
        }
    }
    else
    {
        let tokens = tokenize(expression);
        match tokens
        {
            Ok(tokens) => {
                let rpn = shunting_yard(&tokens);
                match rpn
                {
                    Ok(rpn) => {
                        let result = evaluate_rpn(&rpn);
                        match result
                        {
                            Ok(v) => v.to_string(),
                            Err(_) => "".to_string(),
                        }
                    },
                    Err(_) => "".to_string(),
                }
            },
            Err(_) => "".to_string(),
        }
    }
}

fn parse_variable_declaration(expression: &str) -> Option<(&str, &str)>
{
    if let Some(pos) = expression.find('=')
    {
        let (var_name, expr) = expression.split_at(pos);
        let var_name = var_name.trim();
        let expr = &expr[1..].trim();
        Some((var_name, expr))
    }
    else
    {
        None
    }
}

fn tokenize(expression: &str) -> Result<Vec<String>, String>
{
    let constants: HashMap<String, f64> = HashMap::from([
        ("e".to_string(), std::f64::consts::E),
        ("pi".to_string(), std::f64::consts::PI),
        ("E".to_string(), std::f64::consts::E),
        ("PI".to_string(), std::f64::consts::PI),
        ("phi".to_string(), (1.0 + 5.0_f64.sqrt()) / 2.0),
        ("PHI".to_string(), (1.0 + 5.0_f64.sqrt()) / 2.0),
    ]);
    let mut tokens = Vec::new();
    let mut num_buf = String::new();
    let mut i = 0;
    let chars: Vec<char> = expression.chars().collect();

    while i < chars.len()
    {
        let c = chars[i];
        if c.is_whitespace()
        {
            i += 1;
            continue;
        }
        else if c.is_digit(10) || c == '.'
        {
            num_buf.push(c);
            i += 1;
        }
        else
        {
            if !num_buf.is_empty()
            {
                tokens.push(num_buf.clone());
                num_buf.clear();
            }

            let mut found_constant = false;
            for (key, &value) in &constants
            {
                if expression[i..].starts_with(key)
                {
                    tokens.push(value.to_string());
                    i += key.len();
                    found_constant = true;
                    break;
                }
            }

            if !found_constant
            {
                let var_name = read_variable_name(&chars, i);
                if let Some(var_name) = var_name
                {
                    let vars = VARIABLES.lock().unwrap();
                    if let Some(&value) = vars.get(&var_name)
                    {
                        tokens.push(value.to_string());
                    }
                    else
                    {
                        tokens.push(var_name.clone());
                    }
                    i += var_name.len();
                    continue;
                }

                tokens.push(c.to_string());
                i += 1;
            }
        }
    }

    if !num_buf.is_empty()
    {
        tokens.push(num_buf);
    }

    Ok(tokens)
}

fn read_variable_name(chars: &[char], start: usize) -> Option<String>
{
    let mut end = start;
    while end < chars.len() && chars[end].is_alphabetic()
    {
        end += 1;
    }
    if end > start
    {
        Some(chars[start..end].iter().collect())
    }
    else
    {
        None
    }
}

fn shunting_yard(tokens: &[String]) -> Result<Vec<String>, String>
{
    let mut output: Vec<String> = Vec::new();
    let mut operators: Vec<String> = Vec::new();

    let precedence = |op: &str| -> i32{
        match op
        {
            "+" | "-" => 1,
            "*" | "/" => 2,
            _ => 0,
        }
    };

    for token in tokens
    {
        if token.parse::<f64>().is_ok()
        {
            output.push(token.clone());
        }
        else if ["+", "-", "*", "/"].contains(&token.as_str())
        {
            while let Some(top_op) = operators.last()
            {
                if precedence(top_op) >= precedence(token)
                {
                    output.push(operators.pop().unwrap());
                }
                else
                {
                    break;
                }
            }
            operators.push(token.clone());
        }
        else if token == "("
        {
            operators.push(token.clone());
        }
        else if token == ")"
        {
            while let Some(top_op) = operators.pop()
            {
                if top_op == "("
                {
                    break;
                }
                else
                {
                    output.push(top_op);
                }
            }
        }
        else
        {
            return Err(format!("Unknown token: {}", token));
        }
    }

    while let Some(op) = operators.pop()
    {
        output.push(op);
    }

    Ok(output)
}

fn evaluate_rpn(rpn: &[String]) -> Result<f64, String>
{
    let mut stack = VecDeque::new();

    for token in rpn
    {
        if let Ok(num) = token.parse::<f64>()
        {
            stack.push_back(num);
        }
        else
        {
            let b = stack.pop_back().ok_or("Missing operand")?;
            let a = stack.pop_back().ok_or("Missing operand")?;
            let result = match token.as_str()
            {
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "/" => a / b,
                _ => return Err(format!("Unknown operator: {}", token)),
            };
            stack.push_back(result);
        }
    }

    if stack.len() == 1
    {
        Ok(stack.pop_back().unwrap())
    }
    else
    {
        Err("Invalid expression".to_string())
    }
}