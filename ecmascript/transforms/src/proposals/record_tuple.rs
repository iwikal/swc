use crate::perf::Check;
use swc_common::{Span, DUMMY_SP};
use swc_ecma_ast::{
    CallExpr, Expr, ExprOrSpread, ExprOrSuper, Ident, ObjectLit, RecordLit, TupleLit,
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

        fn call_expr(span: Span, args: Vec<ExprOrSpread>, callee: Ident) -> Expr {
            Expr::Call(CallExpr {
                span,
                callee: ExprOrSuper::Expr(Box::new(Expr::Ident(callee))),
                args,
                type_args: None,
            })
        }

        match e {
            Expr::Record(RecordLit { span, props }) => call_expr(
                span,
                vec![ExprOrSpread {
                    expr: Box::new(Expr::Object(ObjectLit { span, props })),
                    spread: None,
                }],
                quote_ident!("Record"),
            ),
            Expr::Tuple(TupleLit { elems, span }) => call_expr(
                span,
                elems
                    .into_iter()
                    .map(|option| match option {
                        Some(expr) => expr,
                        None => ExprOrSpread {
                            expr: Box::new(Expr::Ident(Ident {
                                sym: js_word!("undefined"),
                                span: DUMMY_SP,
                                optional: false,
                                type_ann: None,
                            })),
                            spread: None,
                        },
                    })
                    .collect(),
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
