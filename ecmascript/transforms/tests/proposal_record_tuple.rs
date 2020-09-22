#![feature(test)]
use swc_ecma_transforms::proposals::record_tuple;
use swc_ecma_visit::Fold;

#[macro_use]
mod common;

fn syntax() -> ::swc_ecma_parser::Syntax {
    Default::default()
}

fn tr() -> impl Fold {
    record_tuple()
}

test!(
    syntax(),
    |_| tr(),
    record_literal,
    "#{ foo: 0, bar, ...baz }",
    "Record({ foo: 0, bar, ...baz })"
);

test!(
    syntax(),
    |_| tr(),
    nested_records,
    "#{ foo: #{ bar } }",
    "Record({ foo: Record({ bar }) })"
);

test!(
    syntax(),
    |_| tr(),
    tuple_literal,
    "#[a, b,, ...d, e]",
    "Tuple(a, b, undefined, ...d, e)"
);

test!(
    syntax(),
    |_| tr(),
    nested_tuples,
    "#[ #[bar] ]",
    "Tuple(Tuple(bar))"
);

test!(
    syntax(),
    |_| tr(),
    nested_mixed,
    "#[ #{ foo: #[] } ]",
    "Tuple(Record({ foo: Tuple() }))"
);
