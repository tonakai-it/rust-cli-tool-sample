use clap::{Arg, ArgAction, Command};

fn main() {
    // CLIの構成
    let matches = Command::new("Clap Sample")
        .version("1.0")
        .author("Your Name <youremail@example.com>")
        .about("A simple CLI example using clap")
        .arg(
            Arg::new("first")
                .help("The First Positional Argument")
                .required(true) // 必須の位置引数
        )
        .arg(
            Arg::new("second")
                .help("The Second Positional Argument")
                .required(false), // 任意の位置引数
        )
        .arg(
            Arg::new("name")
                .short('n')
                .long("name")
                .value_name("NAME")
                .help("Sets a custom name")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .value_name("DEBUG")
                .help("Enables debug mode")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("list")
                .short('l')
                .long("list")
                .value_name("List")
                .help("List")
                .action(ArgAction::Append)
        )
        .get_matches();
    // 引数の取得と処理
    let first = matches.get_one::<String>("first").expect("first is required");
    let second_default_value = String::from("default.txt");
    let second = matches.get_one::<String>("second").unwrap_or(&second_default_value);

    println!("first: {}", first);
    println!("second: {}", second);

    if let Some(name) = matches.get_one::<String>("name") {
        println!("Hello, {}!", name);
    }

    if matches.get_flag("debug") {
        println!("Debug mode is enabled");
    }

    if let Some(values) = matches.get_many::<String>("list") {
        let items: Vec<&String> = values.collect();
        println!("List items: {:?}", items);
    } else {
        println!("No items provided.");
    }
}
