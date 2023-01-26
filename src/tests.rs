use proptest::{
    strategy::{Strategy, ValueTree},
    test_runner::TestRunner,
};
use quickcheck::quickcheck;

use super::*;

/*
    middle_item: "├", last_item: "└", item_indent: "── ",
    middle_skip: "│", last_skip: " ", skip_indent: "   ",
*/

#[test]
fn render_tree_root() {
    let tree = Tree::new("foo");
    assert_eq!(format!("{}", tree), "foo\n");
}

#[test]
fn render_tree_with_leaves() {
    let tree = Tree::new("foo").with_leaves([Tree::new("bar").with_leaves(["baz"])]);
    assert_eq!(
        format!("{}", tree),
        r#"foo
└── bar
    └── baz
"#
    );
}

#[test]
fn render_tree_with_mutltiple_leaves() {
    let tree = Tree::new("foo").with_leaves(["bar", "baz"]);
    assert_eq!(
        format!("{}", tree),
        r#"foo
├── bar
└── baz
"#
    );
}

#[test]
fn render_tree_with_mutltiline_leaves() {
    let tree = Tree::new("foo").with_leaves([
        Tree::new("hello\nworld").with_multiline(Multiline::True),
        Tree::new("goodbye\nworld").with_multiline(Multiline::True),
    ]);
    assert_eq!(
        format!("{}", tree),
        r#"foo
├── hello
│   world
└── goodbye
    world
"#
    );
}

// ---------------------------------------------------------
// https://github.com/BurntSushi/quickcheck/blob/master/examples/reverse.rs

fn reverse<T: Clone>(xs: &[T]) -> Vec<T> {
    let mut rev = vec![];
    for x in xs {
        rev.insert(0, x.clone())
    }
    rev
}

#[test]
fn test_quickcheck() {
    fn equality_after_applying_twice(xs: Vec<isize>) -> bool {
        xs == reverse(&reverse(&xs))
    }
    quickcheck(equality_after_applying_twice as fn(Vec<isize>) -> bool);
}

// ---------------------------------------------------------
// https://github.com/proptest-rs/proptest/blob/master/proptest/examples/tutorial-simplify-play.rs

// Shows how to pick values from a strategy and simplify them.
//
// This is *not* how proptest is normally used; it is simply used to play
// around with value generation.
#[test]
fn test_proptest() {
    let mut runner = TestRunner::default();
    let mut str_val = "[a-z]{1,4}\\p{Cyrillic}{1,4}\\p{Greek}{1,4}".new_tree(&mut runner).unwrap();
    println!("str_val = {}", str_val.current());
    while str_val.simplify() {
        println!("        = {}", str_val.current());
    }
}
