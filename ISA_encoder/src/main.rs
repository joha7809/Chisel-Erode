mod encoder;
mod errors;
mod isa;
mod lexer;
mod parser;
mod resolver;

fn main() {
    use lexer::Lexer;

    let input = "MULT     R6, R1, 0;        #Start Y Loop
ADD     R8, R0, R6;          #R8 = current pixel
JETV     34, R0, Test;          # Is it on a boarder
Test:
JETV     34, R0, 19;
JETV     34, R1, 0;
JETV     34, R1, 19;
    ADD R8, R0, R6;
";
    // let lexer = Lexer::new("ADD     R8, R0, R6          #R8 = current pixel");
    let lexer = Lexer::new(input);
    let res = lexer.lex();

    for token in res.iter().by_ref() {
        println!("{}", token);
    }

    let mut parser = parser::Parser::new(res);
    let instructions = parser.parse_instructions().unwrap();
    let map = parser.generate_label_map();
    println!("Label Map: {:?}", map);
    for instr in instructions.iter() {
        println!("{:?}", instr);
    }
}
