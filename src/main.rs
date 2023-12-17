
use clap::{command,ArgAction,arg};



fn main() {

    let matches = command!()

    .arg(arg!(<TEXT> "Input text (required)").action(ArgAction::Append))
    .arg(arg!(-n --omit_newline "Dont print new line").action(ArgAction::SetTrue))
    .get_matches();

    let arg_text = matches.get_many::<String>("TEXT").unwrap_or_default().map(|txt|txt.as_str()).collect::<Vec<
    _>>();
    let arg_omit_newline = matches.get_flag("omit_newline");

    print!("{}{}",arg_text.join(" "),if arg_omit_newline {""}else{"\n"});
    
}


