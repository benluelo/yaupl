use crate::{
    expression::expression,
    expression::Expression,
    parse_error::ParseError,
    pointer::Pointer,
    tokens::{
        binary_operator_add, binary_operator_div, binary_operator_eq, binary_operator_gt,
        binary_operator_gte, binary_operator_lt, binary_operator_lte, binary_operator_mul,
        binary_operator_neq, binary_operator_sub, BinaryOperatorAdd, BinaryOperatorDiv,
        BinaryOperatorEq, BinaryOperatorGt, BinaryOperatorGte, BinaryOperatorLt, BinaryOperatorLte,
        BinaryOperatorMul, BinaryOperatorNeq, BinaryOperatorSub,
    },
};

pub(crate) fn binary_operation(
    i: &str,
    ptr: Pointer,
) -> Result<(&str, Pointer, Expression), ParseError> {
    let (i, ptr, operand) = binary_operator(i, ptr)?;
    let (i, ptr, first) = expression(i, ptr)?;
    let (i, ptr, second) = expression(i, ptr)?;
    Ok((
        i,
        ptr,
        Expression::BinaryOperation(BinaryOperation {
            operand,
            first: Box::new(first),
            second: Box::new(second),
        }),
    ))
}

fn binary_operator(i: &str, ptr: Pointer) -> Result<(&str, Pointer, BinaryOperator), ParseError> {
    binary_operator_add(i.clone(), ptr.clone())
        .map(op_to_op_enum)
        .or(binary_operator_sub(i.clone(), ptr.clone()).map(op_to_op_enum))
        .or(binary_operator_mul(i.clone(), ptr.clone()).map(op_to_op_enum))
        .or(binary_operator_div(i.clone(), ptr.clone()).map(op_to_op_enum))
        .or(binary_operator_gt(i.clone(), ptr.clone()).map(op_to_op_enum))
        .or(binary_operator_lt(i.clone(), ptr.clone()).map(op_to_op_enum))
        .or(binary_operator_gte(i.clone(), ptr.clone()).map(op_to_op_enum))
        .or(binary_operator_lte(i.clone(), ptr.clone()).map(op_to_op_enum))
        .or(binary_operator_eq(i.clone(), ptr.clone()).map(op_to_op_enum))
        .or(binary_operator_neq(i.clone(), ptr.clone()).map(op_to_op_enum))
}

fn op_to_op_enum(
    res: (&str, Pointer, impl Into<BinaryOperator>),
) -> (&str, Pointer, BinaryOperator) {
    (res.0, res.1, res.2.into())
}

pub struct BinaryOperation {
    operand: BinaryOperator,
    first: Box<Expression>,
    second: Box<Expression>,
}

pub(crate) enum BinaryOperator {
    Add(BinaryOperatorAdd),
    Sub(BinaryOperatorSub),
    Mul(BinaryOperatorMul),
    Div(BinaryOperatorDiv),
    Gt(BinaryOperatorGt),
    Lt(BinaryOperatorLt),
    Gte(BinaryOperatorGte),
    Lte(BinaryOperatorLte),
    Eq(BinaryOperatorEq),
    Neq(BinaryOperatorNeq),
}

// impl<T: Into<BinaryOperator>> From<(&str, Pointer, T)> for (&str, Pointer, BinaryOperator) {
//     fn from(_: (&str, Pointer, T)) -> Self {
//         ()
//     }
// }

impl From<BinaryOperatorAdd> for BinaryOperator {
    fn from(op: BinaryOperatorAdd) -> Self {
        BinaryOperator::Add(op)
    }
}

impl From<BinaryOperatorSub> for BinaryOperator {
    fn from(op: BinaryOperatorSub) -> Self {
        BinaryOperator::Sub(op)
    }
}

impl From<BinaryOperatorMul> for BinaryOperator {
    fn from(op: BinaryOperatorMul) -> Self {
        BinaryOperator::Mul(op)
    }
}

impl From<BinaryOperatorDiv> for BinaryOperator {
    fn from(op: BinaryOperatorDiv) -> Self {
        BinaryOperator::Div(op)
    }
}

impl From<BinaryOperatorGt> for BinaryOperator {
    fn from(op: BinaryOperatorGt) -> Self {
        BinaryOperator::Gt(op)
    }
}

impl From<BinaryOperatorLt> for BinaryOperator {
    fn from(op: BinaryOperatorLt) -> Self {
        BinaryOperator::Lt(op)
    }
}

impl From<BinaryOperatorGte> for BinaryOperator {
    fn from(op: BinaryOperatorGte) -> Self {
        BinaryOperator::Gte(op)
    }
}

impl From<BinaryOperatorLte> for BinaryOperator {
    fn from(op: BinaryOperatorLte) -> Self {
        BinaryOperator::Lte(op)
    }
}

impl From<BinaryOperatorEq> for BinaryOperator {
    fn from(op: BinaryOperatorEq) -> Self {
        BinaryOperator::Eq(op)
    }
}

impl From<BinaryOperatorNeq> for BinaryOperator {
    fn from(op: BinaryOperatorNeq) -> Self {
        BinaryOperator::Neq(op)
    }
}
