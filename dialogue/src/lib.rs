#![allow(unused)]
#![feature(const_trait_impl)]

mod binary_tree;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    hash::Hasher,
    net::IpAddr,
    ops::Index,
};

use console::Style;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Select, Sort};
use itertools::Itertools;
use uuid::Uuid;

pub use crate::binary_tree::BinaryTree;

pub fn run() -> Result<(), Box<dyn Error>> {
    let tree = match init_dialogue_config() {
        Ok(None) => {
            println!("Aborted.");
            None
        }
        Ok(Some(config)) => {
            println!("{:#?}", config);
            Some(config)
        }
        Err(err) => {
            println!("error: {}", err);
            None
        }
    };

    playground(tree)?;

    Ok(())
}

fn playground(tree: Option<DialogueConfig>) -> Result<(), Box<dyn Error>> {
    let tree: DialogueConfig = match tree {
        Some(v) => v,
        None => std::process::exit(2),
    };
    let (root, mut rest) = (tree.root_node, tree.rest_nodes.unwrap());
    rest.insert(0, root);

    let uuid = Uuid::new_v4();

    // ITERTOOLS
    let hashmap_group =
        rest.iter().enumerate().map(|(k, v)| (k as u64, (uuid, v))).into_group_map();
    dbg!(hashmap_group);

    Ok(())
}

// nodes from hashmap
// let mut node = PrimitiveNode { node: rest[0].clone(), index: 0 };
// let nodes: Vec<PrimitiveNode> = rest .iter() .enumerate() .map(|(index, node)| PrimitiveNode
// { node: node.clone(), index }) .collect(); let mut map = HashMap::new();
// for node in nodes { map.insert(node.node, node.index); }
// let node_map = PrimitiveNodeMap(map);

// values
// let mut nodes = PrimitiveNodeVec::new(rest);
// nodes.multi_select();
// nodes.sort();
/* //  (PrimitiveNode::new("Einar", "Norway"), 25),
//  (Viking::new("Olaf", "Denmark"), 24),
//  (Viking::new("Harald", "Iceland"), 12),
#[derive(Hash, Eq, PartialEq, Debug)]
struct PrimitiveNode {
    node: String,
    index: usize,
}

// To make something the key of a HashMap, you need to satisfy 3 traits:
// Hash — How do you calculate a hash value for the type?
// PartialEq — How do you decide if two instances of a type are the same?
// Eq — Can you guarantee that the equality is reflexive, symmetric, and transitive? This requires
// PartialEq. This is based on the definition of HashMap:
// #[derive(Hash, Eq, PartialEq, Debug)]
#[derive(Debug)]
struct PrimitiveNodeMap<T, U>(HashMap<T, U>);

impl<T, U> Eq for PrimitiveNodeMap<T, U>
where
    T: std::cmp::Eq + std::hash::Hash,
    U: std::cmp::PartialEq + std::hash::Hash,
{
    fn assert_receiver_is_total_eq(&self) {}
}

impl<T, U> PartialEq for PrimitiveNodeMap<T, U>
where
    T: std::cmp::Eq + std::hash::Hash,
    U: std::cmp::PartialEq + std::hash::Hash,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
} */

// impl<T, U> std::hash::Hash for PrimitiveNodeMap<T, U> { fn hash<H>(&self, state: &mut H) where H:
// ~const Hasher, { self.map.hash(state); } }
// struct Wrapper<T>(HashSet<T>);
// struct Wrapper<T, U>(HashMap<T, U>);

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

#[derive(Debug)]
#[allow(dead_code)]
pub struct DialogueConfig {
    total_node: u32,
    root_node: String,
    rest_nodes: Option<Vec<String>>,
}

pub fn init_dialogue_config() -> Result<Option<DialogueConfig>, Box<dyn Error>> {
    let theme =
        ColorfulTheme { values_style: Style::new().yellow().dim(), ..ColorfulTheme::default() };
    println!("Welcome to the binary tree setup wizard");

    let total_nodes: u32 =
        Input::with_theme(&theme).with_prompt("total_nodes").default(3).interact()?;

    let root_node: String = Input::with_theme(&theme)
        .with_prompt("root_node")
        .default("root".to_string())
        .interact()?;

    // let vec = &[2, 3, 4, 5, 6];
    let mut children = Vec::new();
    for i in 1..total_nodes {
        // let uuid = Uuid::new_v4();
        match i % 2 == 0 {
            true => {
                let child_node: String = Input::with_theme(&theme)
                    .with_prompt(format!(
                        r#".
root: {}
rest: {:?}
Enter left_node: "#,
                        root_node, children
                    ))
                    .default("left".to_string())
                    .interact()?;
                children.push(child_node);
            }
            false => {
                let child_node: String = Input::with_theme(&theme)
                    .with_prompt(format!(
                        r#".
root: {}
rest: {:?}
Enter right_node: "#,
                        root_node, children
                    ))
                    .default("left".to_string())
                    .interact()?;
                children.push(child_node);
            }
        }
    }
    // if !Confirm::with_theme(&theme).with_prompt("Add more child_nodes").interact()? {
    //     return Ok(None);
    // }
    if !Confirm::with_theme(&theme).with_prompt("Save progress").interact()? {
        return Ok(None);
    }

    let rest_nodes = Some(children);
    Ok(Some(DialogueConfig { total_node: total_nodes, root_node, rest_nodes }))
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Config {
    interface: IpAddr,
    hostname: String,
    use_acme: bool,
    private_key: Option<String>,
    cert: Option<String>,
}

pub fn init_config() -> Result<Option<Config>, Box<dyn Error>> {
    let theme =
        ColorfulTheme { values_style: Style::new().yellow().dim(), ..ColorfulTheme::default() };
    println!("Welcome to the setup wizard");

    if !Confirm::with_theme(&theme).with_prompt("Do you want to continue?").interact()? {
        return Ok(None);
    }

    let interface = Input::with_theme(&theme)
        .with_prompt("Interface")
        .default("127.0.0.1".parse().unwrap())
        .interact()?;

    let hostname = Input::with_theme(&theme).with_prompt("Hostname").interact()?;

    let tls = Select::with_theme(&theme)
        .with_prompt("Configure TLS")
        .default(0)
        .item("automatic with ACME")
        .item("manual")
        .item("no")
        .interact()?;

    let (private_key, cert, use_acme) = match tls {
        0 => (Some("acme.pkey".into()), Some("acme.cert".into()), true),
        1 => (
            Some(Input::with_theme(&theme).with_prompt("  Path to private key").interact()?),
            Some(Input::with_theme(&theme).with_prompt("  Path to certificate").interact()?),
            false,
        ),
        _ => (None, None, false),
    };

    Ok(Some(Config { hostname, interface, private_key, cert, use_acme }))
}
