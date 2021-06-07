use crate::identity;

pub async fn handle_command(args: &[String]) {
    let mut available_commands = vec![];

    let prefix = identity::commands::get_prefix();
    for cmd in identity::commands::get_list() {
        available_commands.push(format!("{}_{}", prefix, cmd));
    }

    if let Some(arg) = args.get(1) {
        if let Some(cmd) = available_commands.iter().find(|cmd| *cmd == arg) {
            let (_, cmd) = cmd.split_at(prefix.len() + 1);
            identity::commands::handle_command(cmd, &args);
        }
    }
}
