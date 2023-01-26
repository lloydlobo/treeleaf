mod binary_tree;
mod term_tree;
#[cfg(test)]
mod tests;

// ---------------------------------------------------------

#[macro_use]
extern crate lazy_static;

use std::{error::Error, fs::File, path::Path, thread, time::Duration};

use console::Style;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use crate::{binary_tree::BinaryTree, term_tree::*};

// ---------------------------------------------------------

lazy_static! {
    static ref THEME: ColorfulTheme = ColorfulTheme::default();
    static ref CONFIG_FILE_PATH: &'static str = "config.json";
}

// ---------------------------------------------------------

pub fn tree<P>(p: P) -> std::io::Result<Tree<String>>
where
    P: AsRef<Path> + std::fmt::Debug,
{
    println!("Path to file: {:?}", p);

    // render tree with leaves
    let tree = Tree::new("foo".to_string()).with_leaves(
        // new nodes
        [Tree::new("bar".to_string()).with_leaves(
            // new nodes
            ["baz".to_string()],
        )],
    );
    println!("{}", format!("{}", tree));
    // render tree with multiple leaves
    let tree = Tree::new("foo".to_string()).with_leaves(
        // new nodes
        ["bar".to_string(), "baz".to_string()],
    );
    println!("{}", format!("{}", tree));
    // render tree with multiline leaves
    let tree = Tree::new("foo".to_string()).with_leaves([
        // new nodes
        Tree::new("bar\nbaz".to_string()).with_multiline(Multiline::True),
        Tree::new("foo\nbar".to_string()).with_multiline(Multiline::True),
    ]);
    println!("{}", format!("{}", tree));

    Ok(tree)
}

// ---------------------------------------------------------

pub fn run() -> Result<(), Box<dyn Error>> {
    {
        treeroot::run();
        thread::sleep(Duration::from_millis(10000));
    }
    {
        let path = Path::new("tree.json"); // cli.path OR DialogueConfig.path
        let tree_str = tree(path)?;
        let _tree_str = format!("{}", tree_str);
        thread::sleep(Duration::from_millis(10000));
    }

    let dialogue_cfg: Option<DialogueConfig> =
        match DialogueConfig::default().init_dialogue_config() {
            Ok(None) => {
                println!("Aborted.");
                None
            }
            Ok(Some(config)) => {
                println!("{:?}", config);
                Some(config)
            }
            Err(err) => {
                println!("error: {}", err);
                None
            }
        };

    let dialogue_cfg = if let Some(x) = dialogue_cfg { x } else { DialogueConfig::default() };

    let vec_nodes = dialogue_cfg.rest_nodes.unwrap();
    let mut rest = vec_nodes.iter().map(|x| x.node.as_str()).collect::<Vec<_>>();
    let root = dialogue_cfg.root_node.node.as_str();
    rest.insert(0, root);
    let vec = rest.clone();
    // let tree = BinaryTree::new(&root);
    let tree = BinaryTree::from_vec(&vec);
    println!("{:#?}", &tree);
    {
        let path = "tree.json";
        let mut file = File::create(path)?;
        serde_json::to_writer(&mut file, &tree)?;
    }

    //TODO: A Parse binary tree to mermaid-diagram.
    //TODO: B draft::playground(tree)?;
    Ok(())
}

// ---------------------------------------------------------

mod treeroot {

    use std::{
        fs::{self, DirEntry, Metadata},
        path::Path,
    };

    use crate::Tree;

    /// Returns the final component of the `Path`, if there is one.
    ///
    /// If the path is a normal file, this is the file name. If it's the path of a directory, this
    /// is the directory name.
    ///
    /// Returns [`None`] if the path terminates in `..`.
    fn label<P>(path: P) -> String
    where
        P: AsRef<Path>,
    {
        path.as_ref().file_name().unwrap().to_str().unwrap().to_owned()
    }

    // * `fold()` - fn fold<Acc, Fold>(self, init: Acc, fold: Fold) -> Acc
    // * `canonicalize` - Returns the canonical, absolute form of the path with all intermediate
    //   components normalized and symbolic links resolved.
    // * `metadata` - Returns the metadata for the file that this entry points at.
    fn tree<P>(path: P) -> std::io::Result<Tree<String>>
    where
        P: AsRef<Path>,
    {
        let result = fs::read_dir(&path)?.filter_map(|e| e.ok()).fold(
            Tree::new(label(path.as_ref().canonicalize()?)),
            |mut root: Tree<String>, entry: DirEntry| {
                let dir: Metadata = entry.metadata().unwrap();
                if dir.is_dir() {
                    root.push(tree(entry.path()).unwrap());
                } else {
                    root.push(Tree::new(label(entry.path())));
                }
                root
            },
        );
        Ok(result)
    }

    pub fn run() {
        let dir: String = std::env::args().nth(1).unwrap_or_else(|| String::from("."));
        match tree(dir) {
            Ok(tree) => println!("{}", tree),
            Err(e) => eprintln!("error: {}", e),
        }
    }
}

// ---------------------------------------------------------

enum Nodes {
    Root,
    Child,
}

// #[derive(Debug, Serialize, Deserialize)]
// struct DialogueNode(u32, String, String);
#[derive(Debug, Serialize, Deserialize, Clone)]
struct DialogueNode {
    index: u32,
    uuid: String,
    node: String,
}

struct ChildPatUuid<'a> {
    child: &'a str,
    pat: Option<char>,
    uuid: Uuid,
}
struct ChildUuid<'a> {
    child: String,
    uuid: ChildPatUuid<'a>,
}

// ---------------------------------------------------------

/// ```json
/// {
///     "total_nodes": 7,
///     "root_node": { "index": 0, "uuid": "63d5b897-a6cd-4167-b303-65cdd6402629", "node": "A" },
///     "rest_nodes": [
///         { "index": 1, "uuid": "22e40956-0d56-4914-8826-03b9a349a949", "node": "B" },
///         { "index": 2, "uuid": "30b74bd8-1b0f-4b6f-9e0b-247472b03c2a", "node": "C" },
///         { "index": 3, "uuid": "5ba79ee7-efd6-494d-a3a3-617a88740ce2", "node": "D" },
///         { "index": 4, "uuid": "de4fde64-af50-40dd-b431-fe286e963d15", "node": "E" },
///         { "index": 5, "uuid": "2f5ff480-b36d-42b3-bbab-f8ae99d4f332", "node": "F" },
///         { "index": 6, "uuid": "30e98c6a-d4c3-4d31-bbf5-52b4394c1d03", "node": "G" }
///     ]
/// }
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct DialogueConfig {
    total_nodes: u32,
    root_node: DialogueNode,
    rest_nodes: Option<Vec<DialogueNode>>,
}

impl Default for DialogueConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl DialogueConfig {
    fn get_child_details(uuid: Uuid, i: u32) -> ChildUuid<'static> {
        let which_child = |side: &'static str| ChildUuid {
            child: side.to_string(),
            uuid: ChildPatUuid { child: side, pat: Some('-'), uuid },
        };
        let left_or_right = match i % 2 != 0 {
            true => which_child("left"),
            false => which_child("right"),
        };
        left_or_right
    }

    pub fn init_dialogue_config(mut self) -> Result<Option<DialogueConfig>, Box<dyn Error>> {
        const ROOT_IDX: u32 = 0;
        let theme = Self::theme();
        println!("Welcome to the binary tree setup wizard");

        // loop{ }
        self.total_nodes = Input::with_theme(&theme)
            .with_prompt("total_nodes")
            .default(self.total_nodes)
            .interact()?;

        let uuid: Uuid = Uuid::new_v4();
        let root_node = Input::with_theme(&theme)
            .with_prompt("root_node")
            .default(Self::parse_default_node(ChildPatUuid {
                child: "root",
                pat: Some('-'),
                uuid,
            })?)
            .interact()?;

        let mut children: Vec<DialogueNode> = Vec::new();
        for i in (ROOT_IDX + 1)..self.total_nodes {
            let display_prev: &String = match i {
                1 => &root_node,
                _ => &children[i as usize - 2].node,
            };

            let uuid = Uuid::new_v4();
            let details = Self::get_child_details(uuid, i);
            let child_node = Input::with_theme(&theme)
                .with_prompt(format!(
                    ".\nroot: {}\nrest: {:?}\nEnter {}_node: ",
                    root_node, display_prev, details.child
                ))
                .default(Self::parse_default_node(details.uuid)?)
                .interact()?;
            children.push(DialogueNode { index: i, uuid: uuid.to_string(), node: child_node });
        }

        // Maybe use a loop to let the user redo from start?
        if !Confirm::with_theme(&theme).with_prompt("Save progress").default(true).interact()? {
            return Ok(None);
        }

        self.root_node = DialogueNode { index: ROOT_IDX, uuid: uuid.to_string(), node: root_node };
        self.rest_nodes = Some(children);

        self.print_to_config_file(*CONFIG_FILE_PATH)?;

        Ok(Some(self))
    }

    /// Creates a new [`DialogueConfig`].
    pub fn new() -> Self {
        Self {
            total_nodes: 3,
            root_node: DialogueNode { index: 0, uuid: String::new(), node: String::new() },
            rest_nodes: None,
        }
    }

    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    fn parse_default_node<'a>(cpu: ChildPatUuid<'a>) -> Result<String, Box<dyn Error>> {
        let pat = if let Some(p) = cpu.pat { p } else { '-' };

        let uuid = cpu.uuid.to_string();
        let uuid =
            if let Some(it) = uuid.split('-').collect::<Vec<_>>().first() { it } else { cpu.child };
        Ok(format!("{}{}{}", cpu.child, pat, uuid))
    }

    //PERF: Use tempfile?
    fn print_to_config_file<P>(&self, path: P) -> Result<(), Box<dyn Error>>
    where
        P: AsRef<std::path::Path>,
    {
        let mut file = File::create(path)?;
        serde_json::to_writer(&mut file, &self)?;
        Ok(())
    }

    fn theme() -> ColorfulTheme {
        ColorfulTheme { values_style: Style::new().yellow().dim(), ..ColorfulTheme::default() }
    }
}

// ---------------------------------------------------------

#[allow(unused)]
mod draft {
    use std::error::Error;

    use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Select, Sort};
    use itertools::Itertools;
    use uuid::Uuid;

    use crate::DialogueConfig;

    pub fn playground(tree: Option<DialogueConfig>) -> Result<(), Box<dyn Error>> {
        let tree: DialogueConfig = match tree {
            Some(v) => v,
            None => std::process::exit(2),
        };
        //         let (root, mut rest) = (tree.1, tree.2.unwrap());
        //         rest.insert(0, root);

        //         let uuid = Uuid::new_v4();
        // root_noderest_nodes
        //         // ITERTOOLS
        //         let hashmap_group =
        //             rest.iter().enumerate().map(|(k, v)| (k as u64, (uuid, v))).into_group_map();
        //         dbg!(hashmap_group);

        Ok(())
    }

    #[derive(Debug)]
    struct PrimitiveNodeVec {
        vec: Vec<String>,
    }

    impl PrimitiveNodeVec {
        fn new(vec: Vec<String>) -> Self {
            Self { vec }
        }

        fn multi_select(&mut self) {
            let defaults: Vec<bool> = Vec::with_capacity(self.vec.len()); // all false OR not selected.
            let selections = MultiSelect::with_theme(&ColorfulTheme::default())
                .with_prompt("Pick your food")
                .items(&self.vec[..])
                .defaults(&defaults[..])
                .interact()
                .unwrap();

            if selections.is_empty() {
                println!("You did not select anything :(");
            } else {
                println!("You selected these things:");
                for selection in selections {
                    println!("  {}", self.vec[selection]);
                }
            }
        }

        // ["root", "left", "right"]
        // [0 , 1, 2]
        // Just use HashMap?
        fn sort(&mut self) {
            let idx: Vec<usize> = Vec::with_capacity(self.vec.len());
            dbg!(&idx);
            let sorted_idx: Vec<usize> = Sort::with_theme(&ColorfulTheme::default())
                .with_prompt("Order your foods by preference")
                .items(&self.vec[..])
                .interact()
                .unwrap();
            dbg!(&sorted_idx);

            // println!("Your favorite item:");
            // println!("  {}", self.vec[sorted[0]]);
            // println!("Your least favorite item:");
            // println!("  {}", self.vec[sorted[sorted.len() - 1]]);
        }
    }
}

/*
 * ANAGRAMS
 * =======
 * mermaid
 * readme
 * diagram
 * tree
 * terminal
 * cli
 *
 * treelike
 * doctree
 * termaidoc
 * treeviz
 * mermaidoc
 * readiagram
 *
 * thoughtree
 * mdtree
 * doctree
 * vizdoc
 * docviz
 * imermaid
 * tree-like
 * treelist
 * linkediagram
 * linkertree
 * treelinker
 * ladders
 * lizter
 * leaflist
 * treeleaf
 *
 * md
 * markdown
 * term
 * cli
 * diag
 * gram
 * read
 * maid
 * merm
 * doc
 *
 * doctermaid
 * mermaidocgram
 *
 * serializedeserialize
 * ser      de
 * linkerterminaltreediagrams
 * li    ter          d    a
 *
 * treeleaves
 * leaf
 * leaves
 * leaf
 * display
 * treeleaf
 *
 */
