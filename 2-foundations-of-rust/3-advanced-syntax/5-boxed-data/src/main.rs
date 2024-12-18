/// Below you find a small start of a data type modelling the abstract syntax tree for an expression,
/// and a small evaluator function.
///
/// Please extend this evaluator in the following ways:
///
/// - Add support for multiplication and division
///
/// - We have added the form "Summation(Vec<Expr>)", representing the sum of a list of expressions.
/// Question: why can we get away with Vec<Expr> enough in that case, instead of Box<Vec<Expr>> ?
///
/// - EXTRA: Since division can fail, the function eval needs to return an Option<i64>, where None indicates that a division by
///   zero has occurred. Can you change the code so that that errors are propagated correctly? (hint: use the ? syntax).

#[derive(PartialEq, Debug)]
enum Expr {
    Const(i64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    
    Var,
    Summation(Vec<Expr>),
}

use std::result;

// inject these two identifiers directly into the current namespace
use Expr::Const;
use Expr::Summation;
use Expr::Var;

// These are convenience functions, so you don't have to type "Box::new" as often
// when building test-data types
fn add(x: Expr, y: Expr) -> Expr {
    Expr::Add(Box::new(x), Box::new(y))
}

fn sub(x: Expr, y: Expr) -> Expr {
    Expr::Sub(Box::new(x), Box::new(y))
}

fn mul(x: Expr, y: Expr) -> Expr {
    // todo!()
    Expr::Mul(Box::new(x), Box::new(y))
}

fn div(x: Expr, y: Expr) -> Expr {
    // todo!()
    Expr::Div(Box::new(x), Box::new(y))
}

// ...

fn eval(expr: &Expr, var: i64) -> Option<i64> {
    // this should return an Option<i64>
    use Expr::*;
    match expr {
        Const(k) => return Some(*k),
        Var => return Some(var),
        Add(lhs, rhs) => return Some(eval(lhs, var)? + eval(rhs, var)?),
        Sub(lhs, rhs) => return Some(eval(lhs, var)? - eval(rhs, var)?),
        Div(lhs, rhs) =>  {
            let x = eval(rhs, var)?;
            if x == 0 {
                return None
            } else  {
                return Some(eval(lhs, var)? / x)
            }
        }
        Mul(lhs, rhs) => return Some(eval(lhs, var)? * eval(rhs, var)?),

        Summation(exprs) => {
            let mut acc = 0;
            for e in exprs {
                acc += eval(e, var)?;
            }
            return Some(acc)
        }
    }
    return None
}

fn main() {
    let test = |expr| {
        let value = rand::random::<i8>() as i64;
        let rez = eval(&expr, value);

        match rez {
            Some(v) => println!(
                "{:?} with Var = {} ==> {}",
                &expr,
                value,
                v,
            ),
            None => print!("no div by 0\n")
        }

        // println!(
        //     "{:?} with Var = {} ==> {}",
        //     &expr,
        //     value,
        // );
    };

    test(Const(5));
    test(Var);
    test(sub(Var, Const(5)));
    test(sub(Var, Var));
    test(add(sub(Var, Const(5)), Const(5)));
    test(Summation(vec![Var, Const(1)]));
    test(div(Var, Const(0)));
    test(mul(Var, Const(1)));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cases() {
        let x = 42;
        assert_eq!(eval(&Const(5), x), Some(5));
        assert_eq!(eval(&Var, x), Some(42));
        assert_eq!(eval(&sub(Var, Const(5)), x), Some(37));
        assert_eq!(eval(&sub(Var, Var), x), Some(0));
        assert_eq!(eval(&add(sub(Var, Const(5)), Const(5)), x), Some(42));
        assert_eq!(eval(&Summation(vec![Var, Const(1)]), x), Some(43));
    }
}

// If you have time left and want to code more Rust: you can extend this exercise endlessly; one idea would be adding a Sigma(from,to,expr)
// constructor to Expr which computes the equivalent of (in LaTeX notation) \sum_{Var = from}^{to} expr; i.e. Sigma(Const(1), Const(5), Var) should be
// equivalent to Summation(vec![Const(1), Const(2), Const(3), Const(4), Const(5)]).
