use dialogue::*;

fn main() {
    match init_dialogue_config() {
        Ok(None) => println!("Aborted."),
        Ok(Some(config)) => println!("{:#?}", config),
        Err(err) => println!("error: {}", err),
    }
    // match init_config() {
    //     Ok(None) => println!("Aborted."),
    //     Ok(Some(config)) => println!("{:#?}", config),
    //     Err(err) => println!("error: {}", err),
    // }
}
