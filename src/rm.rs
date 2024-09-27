use clap::Parser;

#[derive(Parser)]
struct RemoveArgs {
    tool_name: String,
    tool_version: String,
}

pub fn remove_function(args: &RemoveArgs) {
    println!(
        "Removinging {} version {}",
        args.tool_name, args.tool_version
    );
    // Add actual uninstallation logic here
}
