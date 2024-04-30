mod data_type;
mod parser;
mod runtime;
mod tokeniser;

fn main() {
    let args = std::env::args();
    let file = args.into_iter().last().unwrap();
    let file_string = std::fs::read_to_string(file).unwrap();

    // let (instructions, function_indexes, block_indexes) =
    parser::parse(tokeniser::tokenise(&file_string));

    // runtime::execute(instructions, function_indexes, block_indexes);
}
