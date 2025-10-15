mod encoder;
mod isa;
mod lexer;
mod parser;

fn main() {
    use lexer::Lexer;

    let input = "MULT     R6, R1, 0;        #Start Y Loop
ADD     R8, R0, R6;          #R8 = current pixel
JETV     34, R0, Test;          # Is it on a boarder
JETV     34, R0, 19;
JETV     34, R1, 0;
JETV     34, R1, 19;
Test:
    ADD R8, R0, R6;
";
    // let lexer = Lexer::new("ADD     R8, R0, R6          #R8 = current pixel");
    let lexer = Lexer::new(input);
    let res = lexer.lex();

    for token in res {
        println!("{}", token);
    }
}
