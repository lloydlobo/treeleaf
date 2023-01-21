mod binary_tree;
use std::{error::Error, net::IpAddr};

use console::Style;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Select, Sort};

pub use crate::binary_tree::BinaryTree;

pub fn run() -> Result<(), Box<dyn Error>> {
    let tree: Option<DialogueConfig> = match init_dialogue_config() {
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

    let tree: DialogueConfig = match tree {
        Some(v) => v,
        None => std::process::exit(2),
    };
    let (root, mut rest) = (tree.root_node, tree.rest_nodes.unwrap());
    rest.insert(0, root);

    let mut values = Values::new(rest);
    values.multi_select();
    values.sort();
    Ok(())
}

#[derive(Debug)]
struct Values {
    vec: Vec<String>,
}

impl Values {
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
