use crate::perf::Check;
use swc_common::Span;
use swc_ecma_ast::{
    ArrayLit, CallExpr, Expr, ExprOrSpread, ExprOrSuper, Ident, ObjectLit, RecordLit, TupleLit,
};
use swc_ecma_transforms_macros::fast_path;
use swc_ecma_utils::quote_ident;
use swc_ecma_visit::{noop_fold_type, Fold, FoldWith, Node, Visit};

pub fn record_tuple() -> impl Fold {
    RecordTuple
}

struct RecordTuple;

#[fast_path(RecordTupleVisitor)]
impl Fold for RecordTuple {
    noop_fold_type!();

    fn fold_expr(&mut self, e: Expr) -> Expr {
        let e = validate!(e);
        let e = e.fold_children_with(self);

        fn call_expr(span: Span, arg: Expr, callee: Ident) -> Expr {
            Expr::Call(CallExpr {
                span,
                callee: ExprOrSuper::Expr(Box::new(Expr::Ident(callee))),
                args: vec![ExprOrSpread {
                    expr: Box::new(arg),
                    spread: None,
                }],
                type_args: None,
            })
        }

        match e {
            Expr::Record(RecordLit { span, props }) => call_expr(
                span,
                Expr::Object(ObjectLit { span, props }),
                quote_ident!("Record"),
            ),
            Expr::Tuple(TupleLit { elems, span }) => call_expr(
                span,
                Expr::Array(ArrayLit { span, elems }),
                quote_ident!("Tuple"),
            ),
            _ => e,
        }
    }
}

#[derive(Default)]
struct RecordTupleVisitor {
    found: bool,
}

impl Visit for RecordTupleVisitor {
    noop_visit_type!();

    fn visit_record_lit(&mut self, _: &RecordLit, _: &dyn Node) {
        self.found = true;
    }

    fn visit_tuple_lit(&mut self, _: &TupleLit, _: &dyn Node) {
        self.found = true;
    }
}

impl Check for RecordTupleVisitor {
    fn should_handle(&self) -> bool {
        self.found
    }
}
