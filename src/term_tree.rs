//! Code derived from [termtree](https://github.com/rust-cli/termtree/blob/main/src/lib.rs)
#![warn(rustdoc::missing_doc_code_examples)]

use std::{collections::VecDeque, fmt::Display, rc::Rc};

// ---------------------------------------------------------

pub struct Tree<D>
where
    D: Display,
{
    pub root: D,
    pub leaves: Vec<Tree<D>>,
    multiline: bool,
    glyphs: GlyphPalette,
}

pub trait TreeSetter {
    fn set_multiline(&mut self, is_multiline: Multiline) -> &mut Self;
    fn set_glyphs(&mut self, glyphs: GlyphPalette) -> &mut Self;
}

// ---------------------------------------------------------

impl<D> Tree<D>
where
    D: Display,
{
    pub fn new(root: D) -> Self {
        Self { root, leaves: Vec::new(), multiline: false, glyphs: GlyphPalette::new() }
    }

    pub fn push(&mut self, leaf: impl Into<Tree<D>>) -> &mut Self {
        self.leaves.push(leaf.into());

        self
    }

    /// Customize how this mode is rendered
    pub fn with_glyphs(mut self, glyphs: GlyphPalette) -> Self {
        self.glyphs = glyphs;
        self
    }

    pub fn with_leaves(mut self, leaves: impl IntoIterator<Item = impl Into<Tree<D>>>) -> Self {
        self.leaves = leaves.into_iter().map(Into::into).collect();
        self
    }

    // Ensure all lines for `root` are indented.
    pub fn with_multiline(mut self, is_multiline: Multiline) -> Self {
        self.multiline = matches!(is_multiline, Multiline::True);
        self
    }
}

impl<D> TreeSetter for Tree<D>
where
    D: Display,
{
    /// Ensure all lines for `root` are indented.
    fn set_multiline(&mut self, is_multiline: Multiline) -> &mut Self {
        self.multiline = matches!(is_multiline, Multiline::True);
        assert!(self.multiline);
        self
    }

    /// Customize the reneding of this node.
    fn set_glyphs(&mut self, glyphs: GlyphPalette) -> &mut Self {
        self.glyphs = glyphs;
        self
    }
}

// ---------------------------------------------------------

impl<D> From<D> for Tree<D>
where
    D: Display,
{
    fn from(inner: D) -> Self {
        Self::new(inner)
    }
}

impl<D> Extend<D> for Tree<D>
where
    D: Display,
{
    fn extend<T: IntoIterator<Item = D>>(&mut self, iter: T) {
        self.leaves.extend(iter.into_iter().map(Into::into))
    }
}

impl<D> Extend<Tree<D>> for Tree<D>
where
    D: Display,
{
    fn extend<T: IntoIterator<Item = Tree<D>>>(&mut self, iter: T) {
        self.leaves.extend(iter)
    }
}

// ---------------------------------------------------------

impl<D> Display for Tree<D>
where
    D: Display,
{
    fn fmt<'t>(&'t self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.root)?;
        let mut queue = DisplayQueue::<'t, D>::new();
        let no_space: Rc<Vec<bool>> = Rc::new(Vec::new());
        enqueue_leaves(&mut queue, self, no_space);

        // Loop while removing the first element and return it,
        // or break the loop if the deque is empty if `None`.
        while let Some((last, leaf, spaces)) = queue.pop_front() {
            let mut prefix: (&str, &str) = (
                if last { leaf.glyphs.last_item } else { leaf.glyphs.middle_item },
                leaf.glyphs.item_indent,
            );

            if Multiline::True.from_tree(leaf) {
                // if leaf.multiline {
                let rest_prefix: (&str, &str) = (
                    if last { leaf.glyphs.last_skip } else { leaf.glyphs.middle_skip },
                    leaf.glyphs.skip_indent,
                );
                debug_assert_eq!(prefix.0.chars().count(), rest_prefix.0.chars().count());
                debug_assert_eq!(prefix.1.chars().count(), rest_prefix.1.chars().count());

                let root: String = leaf.root.to_string();
                for line in root.lines() {
                    // Print single line
                    for s in spaces.as_slice() {
                        if *s {
                            write!(f, "{}{}", self.glyphs.last_skip, self.glyphs.skip_indent)?;
                        } else {
                            write!(f, "{}{}", self.glyphs.middle_skip, self.glyphs.skip_indent)?;
                        }
                    }
                    writeln!(f, "{}{}{}", prefix.0, prefix.1, line)?;
                    prefix = rest_prefix;
                }
            } else {
                // Print single line
                for s in spaces.as_slice() {
                    if *s {
                        write!(f, "{}{}", self.glyphs.last_skip, self.glyphs.skip_indent)?;
                    } else {
                        write!(f, "{}{}", self.glyphs.middle_skip, self.glyphs.skip_indent)?;
                    }
                }
                writeln!(f, "{}{}{}", prefix.0, prefix.1, leaf.root)?;
            };

            // Recursion
            if !leaf.leaves.is_empty() {
                let s: &Vec<bool> = &spaces;
                let mut child_spaces = s.clone();
                child_spaces.push(last);
                let child_spaces: Rc<Vec<bool>> = Rc::new(child_spaces);
                enqueue_leaves(&mut queue, leaf, child_spaces);
            }
        }

        Ok(())
    }
}

// ---------------------------------------------------------

type DisplayQueue<'t, D> = VecDeque<(bool, &'t Tree<D>, Rc<Vec<bool>>)>;

fn enqueue_leaves<'t, D>(
    queue: &mut DisplayQueue<'t, D>,
    parent: &'t Tree<D>,
    spaces: Rc<Vec<bool>>,
) where
    D: Display,
{
    for (i, leaf) in parent.leaves.iter().rev().enumerate() {
        let last = i == 0;
        queue.push_front((last, leaf, spaces.clone()));
    }
}

// ---------------------------------------------------------

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct GlyphPalette {
    pub middle_item: &'static str,
    pub last_item: &'static str,
    pub item_indent: &'static str,

    pub middle_skip: &'static str,
    pub last_skip: &'static str,
    pub skip_indent: &'static str,
}

/// The advantage of implementing or deriving Default is that your type can now be used where a
/// Default implementation is required, most prominently, any of the *or_default functions in the
/// standard library.
impl Default for GlyphPalette {
    fn default() -> Self {
        Self::new()
    }
}

impl GlyphPalette {
    pub fn new() -> Self {
        Self {
            middle_item: "├",
            last_item: "└",
            item_indent: "── ",

            middle_skip: "│",
            last_skip: " ",
            skip_indent: "   ",
        }
    }
}

// ---------------------------------------------------------

pub enum Multiline {
    False,
    True,
}

impl Default for Multiline {
    fn default() -> Self {
        Self::False
    }
}

impl Multiline {
    fn from_tree<D>(&self, leaf: &Tree<D>) -> bool
    where
        D: Display,
    {
        matches!((self, leaf.multiline), (Multiline::True, true))
    }
}

// impl From<Multiline> for bool {
//     fn from(val: Multiline) -> Self {
//         match val {
//             Multiline::True => true,
//             Multiline::False => false,
//         }
//     }
// }
