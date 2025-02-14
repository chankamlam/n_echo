
use clap::{command,ArgAction,arg};



fn main() {

    let matches = command!()

    .arg(arg!(<TEXT> "Input text (required)").action(ArgAction::Append))
    .arg(arg!(-n --omit_newline "Do not print the trailing newline character.").action(ArgAction::SetTrue))
    .get_matches();

    let arg_text:Vec<&str> = matches.get_many::<String>("TEXT").unwrap_or_default().map(|txt|txt.as_str()).collect();
    let omit_newline = matches.get_flag("omit_newline");

    print!("{}{}",arg_text.join(" "),if omit_newline {""}else{"\n"});
    
}


