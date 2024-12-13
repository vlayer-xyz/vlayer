use clap::command;
let mut command = clap::command!();
// Register `complete` subcommand
command = clap_autocomplete::add_subcommand(command);

// Add other arguments and subcommands

let command_copy = command.clone();
// Resolve the matches
let matches = command.get_matches();
if let Some(result) = clap_autocomplete::test_subcommand(&matches, command_copy) {
    if let Err(err) = result {
        eprintln!("Insufficient permissions: {err}");
        std::process::exit(1);
    } else {
        std::process::exit(0);
    }
} else {
    // Continue with the application logic
}