use ArithCmpOp::*;
use ArithExpr::*;
use BinArithOp::*;
use BinLogicOp::*;
use BoolExpr::*;
use Expr::*;
use Value::*;

pub enum Expr {
    ArithExpr(ArithExpr),
    BoolExpr(BoolExpr),
}

pub enum ArithExpr {
    BinArithExpr {
        left: Box<ArithExpr>,
        right: Box<ArithExpr>,
        op: BinArithOp,
    },
    IntLit(i64),
}

pub enum BoolExpr {
    ArithCmpExpr {
        left: Box<ArithExpr>,
        right: Box<ArithExpr>,
        op: ArithCmpOp,
    },
    BinBoolExpr {
        left: Box<BoolExpr>,
        right: Box<BoolExpr>,
        op: BinLogicOp,
    },
    NotExpr(Box<BoolExpr>),
    BoolLit(bool),
}

pub enum BinArithOp {
    AddOp,
    SubOp,
    MulOp,
    IntDivOp,
}

pub enum ArithCmpOp {
    LtOp,
    LteOp,
    GtOp,
    GteOp,
    ArithEqOp,
    ArithNeqOp,
}

pub enum BinLogicOp {
    AndOp,
    OrOp,
    BoolEqOp,
    BoolNeqOp,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    BoolValue(bool),
    IntValue(i64),
}

pub fn eval(expr: Expr) -> Value {
    match expr {
        ArithExpr(expr) => Value::IntValue(eval_arith_expr(expr)),
        BoolExpr(expr) => Value::BoolValue(eval_bool_expr(expr)),
    }
}

pub fn eval_arith_expr(arith_expr: ArithExpr) -> i64 {
    match arith_expr {
        BinArithExpr {left, right, op} => {
            let left = eval_arith_expr(*left); //make variable
            let right = eval_arith_expr(*right); //make variable
            match op {
                AddOp => left + right,
                SubOp => left - right,
                MulOp => left * right,
                IntDivOp => left / right,
            }
        }
        IntLit(num) => num, 
    }
}


pub fn eval_bool_expr(bool_expr: BoolExpr) -> bool {
    match bool_expr {
        ArithCmpExpr {left, right, op} =>
        {
            let left = eval_arith_expr(*left); //make variable
            let right = eval_arith_expr(*right); //make variable
            match op {
                LtOp => left < right,
                LteOp => left <= right,
                GtOp => left > right,
                GteOp => left >= right,
                ArithEqOp => left == right,
                ArithNeqOp => left != right,
            }

        }
        BinBoolExpr{left, right, op} =>
        {
            let left = eval_bool_expr(*left); //make variable
            let right = eval_bool_expr(*right); //make variable
            match op {
                AndOp => left && right,
                OrOp => left || right,
                BoolEqOp => left == right,
                BoolNeqOp => left != right
            }
            
        }
        NotExpr(bool_expr) => !eval_bool_expr(*bool_expr),
        BoolLit(boolean) => boolean
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let expr = Expr::BoolExpr(BoolLit(true));
        let answer = BoolValue(true);

        assert_eq!(eval(expr), answer);  // eval(BoolExpr(BoolLit(true))) == BoolValue(true)
    }

    #[test]
    fn test_1(){
        let expr = ArithExpr(BinArithExpr{
            left: Box::new(BinArithExpr
                {left: Box::new(IntLit(4)), 
                right: Box::new(IntLit(9)), 
                op: AddOp}), 
            right: Box::new(BinArithExpr
                {left: Box::new(IntLit(4)), 
                right: Box::new(IntLit(9)), 
                op: SubOp}), 
            op: MulOp});
        let answer = IntValue(-65);

        assert_eq!(eval(expr), answer);  // eval(BoolExpr(BoolLit(true))) == BoolValue(true)
    }

    #[test]
    fn test_2(){
        let expr = ArithExpr(BinArithExpr{left: Box::new(IntLit(4)), right: Box::new(IntLit(2)), op: IntDivOp});
        let answer = IntValue(2);

        assert_eq!(eval(expr), answer);  // eval(BoolExpr(BoolLit(true))) == BoolValue(true)        
    }

    #[test]
    fn test_3(){
        let expr = BoolExpr(BinBoolExpr{
            left: Box::new(BinBoolExpr
                {left: Box::new(BoolLit(true)), 
                right: Box::new(BoolLit(true)), 
                op: AndOp}), 
            right: Box::new(BinBoolExpr
                {left: Box::new(BoolLit(false)), 
                right: Box::new(BoolLit(false)), 
                op: OrOp}), 
            op: BoolEqOp});
        let answer = BoolValue(false);

        assert_eq!(eval(expr), answer);  // eval(BoolExpr(BoolLit(true))) == BoolValue(true)
    }

    #[test]
    fn test_4(){
        let expr = BoolExpr(BinBoolExpr{left: Box::new(BoolLit(true)), right: Box::new(BoolLit(false)), op: BoolNeqOp});
        let answer = BoolValue(true);

        assert_eq!(eval(expr), answer);  // eval(BoolExpr(BoolLit(true))) == BoolValue(true)        
    }

    #[test]
    fn test_5(){
        let expr = BoolExpr(BinBoolExpr{
            left: Box::new(ArithCmpExpr
                {left: Box::new(IntLit(4)), 
                right: Box::new(IntLit(3)), 
                op: LtOp}), 
            right: Box::new(ArithCmpExpr
                {left: Box::new(IntLit(4)), 
                right: Box::new(IntLit(4)), 
                op: LteOp}), 
            op: BoolEqOp});
    
        let answer = BoolValue(false);

        assert_eq!(eval(expr), answer);  // eval(BoolExpr(BoolLit(true))) == BoolValue(true)
    }

    #[test]
    fn test_6(){
        let expr = BoolExpr(BinBoolExpr{
            left: Box::new(ArithCmpExpr
                {left: Box::new(IntLit(4)), 
                right: Box::new(IntLit(3)), 
                op: ArithNeqOp}), 
            right: Box::new(ArithCmpExpr
                {left: Box::new(IntLit(4)), 
                right: Box::new(IntLit(4)), 
                op: ArithEqOp}), 
            op: BoolEqOp});
    
        let answer = BoolValue(true);

        assert_eq!(eval(expr), answer);  // eval(BoolExpr(BoolLit(true))) == BoolValue(true)
    }

    #[test]
    fn test_7(){
        let expr = BoolExpr(BinBoolExpr{
            left: Box::new(ArithCmpExpr
                {left: Box::new(IntLit(4)), 
                right: Box::new(IntLit(3)), 
                op: GtOp}), 
            right: Box::new(ArithCmpExpr
                {left: Box::new(IntLit(4)), 
                right: Box::new(IntLit(4)), 
                op: GteOp}), 
            op: BoolEqOp});
    
        let answer = BoolValue(true);

        assert_eq!(eval(expr), answer);  // eval(BoolExpr(BoolLit(true))) == BoolValue(true)
    }
    #[test]
    fn test_8() {
        let expr = NotExpr(Box::new(BoolLit(true)));
        assert_eq!(eval_bool_expr(expr), false);
    }
    #[test]
    fn test_others() {
        main();
        println!("{:?}", BoolValue(true));
    }
}
