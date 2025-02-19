use monkey_interpreter_rs::lexer::Lexer;

fn main() {
    let input = "hello = 123 + (abc)";
    let mut l = Lexer::new(input);

    while let Some(ch) = l.next() {
        println!("{}", ch);
    }

    // let mut lexer = Lexer::new(input);
    // while let Some(tok) = lexer.next_token().token_type {
    //     println!("{:?}", tok);
    // }
}