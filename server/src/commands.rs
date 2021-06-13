use crate::asset;
use crate::identity;
use crate::portfolio;
use crate::price;
use crate::service;

macro_rules! try_handle {
    ( $prefix:expr, $cmd:expr, $args:expr, $module:path ) => {{
        use $module as base;
        let prefix = base::get_prefix();
        if prefix == $prefix {
            base::handle_command($cmd, $args)
        } else {
            false
        }
    }};
}

macro_rules! try_handle_async {
    ( $prefix:expr, $cmd:expr, $args:expr, $module:path ) => {{
        use $module as base;
        let prefix = base::get_prefix();
        if prefix == $prefix {
            base::handle_command($cmd, $args).await
        } else {
            false
        }
    }};
}

macro_rules! add_commands {
    ( $av_cmd:expr, $module:path ) => {{
        use $module as base;
        let prefix = base::get_prefix();
        let cmds = base::get_list();
        for cmd in cmds {
            $av_cmd.push(format!("{}_{}", prefix, cmd));
        }
    }};
}

pub async fn handle_command(args: &[String]) {
    #[allow(unused_must_use)]
    if let Some(arg) = args.get(1) {
        let (prefix, cmd) = arg.rsplit_once("_").unwrap();
        if try_handle!(prefix, cmd, args, identity::commands)
            || try_handle_async!(prefix, cmd, args, asset::commands::asset)
            || try_handle!(prefix, cmd, args, asset::commands::category)
            || try_handle!(prefix, cmd, args, asset::commands::tag)
            || try_handle_async!(prefix, cmd, args, price::commands)
            || try_handle!(prefix, cmd, args, service::commands::service)
            || try_handle!(prefix, cmd, args, service::commands::wallet)
            || try_handle_async!(prefix, cmd, args, portfolio::commands)
        {
            return;
        }
    }
    let mut available_commands = vec![];
    add_commands!(available_commands, identity::commands);
    add_commands!(available_commands, asset::commands::asset);
    add_commands!(available_commands, asset::commands::category);
    add_commands!(available_commands, asset::commands::tag);
    add_commands!(available_commands, price::commands);
    add_commands!(available_commands, service::commands::service);
    add_commands!(available_commands, service::commands::wallet);
    add_commands!(available_commands, portfolio::commands);

    println!("{:#?}", available_commands);
}
