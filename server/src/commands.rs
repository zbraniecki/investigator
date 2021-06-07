use crate::asset;
use crate::identity;

pub async fn handle_command(args: &[String]) {
    let mut available_commands = vec![];

    let prefix = identity::commands::get_prefix();
    for cmd in identity::commands::get_list() {
        available_commands.push(format!("{}_{}", prefix, cmd));
    }

    let prefix = asset::commands::get_prefix();
    for cmd in asset::commands::get_list() {
        available_commands.push(format!("{}_{}", prefix, cmd));
    }

    if let Some(arg) = args.get(1) {
        if let Some(cmd) = available_commands.iter().find(|cmd| *cmd == arg) {
            let (prefix, cmd) = cmd.split_once("_").unwrap();
            match prefix {
                "identity" => identity::commands::handle_command(cmd, &args),
                "asset" => asset::commands::handle_command(cmd, &args),
                _ => {}
            }
            return;
        }
    }
    println!("{:#?}", available_commands);
}
