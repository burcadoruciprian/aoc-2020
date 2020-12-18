use rhai::{Engine, RegisterFn};

fn evaluate_expr1(expr: &str) -> Result<i64, String> {
    const MY_PROD: &str = "my_prod";
    let mut engine = Engine::new();
    engine
        .register_custom_operator(MY_PROD, 150)?
        .register_fn(MY_PROD, |x: i64, y: i64| x * y);
    let result = engine
        .eval_expression::<i64>(expr.replace("*", MY_PROD).as_str())
        .unwrap();

    Ok(result)
}

fn evaluate_expr2(expr: &str) -> Result<i64, String> {
const MY_SUM: &str = "my_sum";
    let mut engine = Engine::new();
    engine
        .register_custom_operator(MY_SUM, 181)?
        .register_fn(MY_SUM, |x: i64, y: i64| x + y);
    let result = engine
        .eval_expression::<i64>(expr.replace("+", MY_SUM).as_str())
        .unwrap();

    Ok(result)
}

fn main() {
    let raw_input = std::fs::read_to_string("src/input.txt").expect("Error reading the file!");
    println!("Part1: {}", raw_input.lines().map(|l| evaluate_expr1(l).unwrap()).sum::<i64>());
    println!("Part1: {}", raw_input.lines().map(|l| evaluate_expr2(l).unwrap()).sum::<i64>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_input1() {
        let input = "1 + 2 * 3 + 4 * 5 + 6";
        assert_eq!(evaluate_expr1(input), Ok(71));

        let input = "1 + (2 * 3) + (4 * (5 + 6))";
        assert_eq!(evaluate_expr1(input), Ok(51));

        let input = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        assert_eq!(evaluate_expr1(input), Ok(13632));
    }

    #[test]
    fn test_evaluate_input2() {
        let input = "1 + 2 * 3 + 4 * 5 + 6";
        assert_eq!(evaluate_expr2(input), Ok(231));

        let input = "1 + (2 * 3) + (4 * (5 + 6))";
        assert_eq!(evaluate_expr2(input), Ok(51));

        let input = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        assert_eq!(evaluate_expr2(input), Ok(23340));
    }
}
