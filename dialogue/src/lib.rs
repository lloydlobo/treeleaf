mod binary_tree;
use std::{error::Error, net::IpAddr};

use console::Style;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};

pub use crate::binary_tree::BinaryTree;

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
    // if !Confirm::with_theme(&theme).with_prompt("Add more").interact()? {
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
