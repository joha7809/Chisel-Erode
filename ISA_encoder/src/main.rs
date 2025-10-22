mod encoder;
mod errors;
mod isa;
mod lexer;
mod parser;
mod resolver;

fn main() {
    use lexer::Lexer;

    let input = "
LI     R3, 0;            #R0 = x, R1 = y, R2 = 20, R3 = 0, R4 = 400, R5 = 255, R6 = tempY, R7= R7, R8 = R8
LI     R2, 20;
LI     R5, 255;
LI     R4, 400;
LI     R0, 0;                   #X = 0  
LI     R1, 0;                      #Y = 0         Start X Loop
MULT     R6, R1, R2;        #Start Y Loop
ADD     R8, R0, R6;          #R8 = current pixel
JETV     R0, 0, 34;          # Is it on a boarder
JETV     R0, 19, 34;
JETV     R1, 0, 34;
JETV     R1, 19, 34;
LD     R7, R8;         # Is it black
JETV     R7, 0, 34;
SUBI     R8, R8, 1;         #x-1 # Is it's Neighbors Black
LD     R7, R8;          #current pixel status
ADDI     R8, R8, 1;         #x neutral
JETV     R7, 0, 34;
ADDI     R8, R8, 1;         #x+1
LD     R7, R8; 
SUBI     R8, R8, 1;         #x neutral
JETV     R7, 0, 34;
SUBI     R8, R8, 20;         #y-1
LD     R7, R8; 
ADDI     R8, R8, 20;         #y neutral
JETV     R7, 0, 34;
ADDI     R8, R8, 20;         #y+1
LD     R7, R8; 
SUBI     R8, R8, 20;         # neutral
JETV     R7, 0, 34;
ADD     R8, R8, R4;        #Set current pixel to 255 “SCPT255”
SD     R5, R8;
JR     36;
ADD     R8, R8, R4;        #Set current pixel to 0      “SCPT0”
SD     R3, R8; 
ADDI    R1, R1, 1;        #Loops
JLT     R1, 19, 7;                #Loop y
ADDI     R0, R0, 1;  
JLT     R0, 19, 6;                 #Loop x         
END;
";
    // let lexer = Lexer::new("ADD     R8, R0, R6          #R8 = current pixel");
    let lexer = Lexer::new(input);
    let res = lexer.lex();

    for token in res.iter().by_ref() {
        println!("{}", token);
    }

    let mut parser = parser::Parser::new(res);
    let instructions = parser.parse_instructions().unwrap();
    for instr in instructions.iter() {
        println!("parsed: {:?}", instr);
    }
    // Test the encoder
    let encoded = encoder::encode_program(&instructions).unwrap();
    for code in encoded.iter() {
        println!("{:032b}", code);
    }
}
