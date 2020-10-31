#[derive(Debug)]
enum Token {
    Paren(char),
    Number(String),
    String(String),
    Name(String)
}


fn main() {
    let input = String::from("(add 2 (subtract 4 2))");
    let tokens = tokenizer(&input);
    println!("{:?}", tokens)
}

/**
 * We're gonna start off with our first phase of parsing, lexical analysis, with
 * the tokenizer. We're just going to take our string of code and break it down into an array
 * of tokens.
 *
 *   (add 2 (subtract 4 2))   =>   [{ type: 'paren', value: '(' }, ...]
 */
fn tokenizer(input: &String) -> Vec<Token> {
    // A `current` variable for tracking our position in the code like a cursor
    let mut current: usize = 0;
 
    // And a `tokens` array for pushing our tokens to.
    let mut tokens: Vec<Token> = vec![];

    let chars: Vec<char> = input.chars().collect();

    while current < input.len() {
        // We're also going to store the `current` character in the `input`.
        let mut current_char = chars[current];
        // The first thing we want to check for is an open parenthesis. This will
        // later be used for `CallExpression` but for now we only care about the
        // character.
        //
        // We check to see if we have an open parenthesis:
        if current_char == '(' {
            tokens.push(Token::Paren(current_char));
            current += 1;

            // And we `continue` onto the next cycle of the loop.
            continue;
        }

        // Next we're going to check for a closing parenthesis. We do the same exact
        // thing as before: Check for a closing parenthesis, add a new token,
        // increment `current`, and `continue`.
        if current_char == ')' {
            tokens.push(Token::Paren(current_char));
            current += 1;
            continue;
        }

        // Moving on, we're now going to check for whitespace. This is interesting
        // because we care that whitespace exists to separate characters, but it
        // isn't actually important for us to store as a token. We would only throw
        // it out later.
        //
        // So here we're just going to test for existence and if it does exist we're
        // going to just `continue` on.
        if current_char.is_whitespace() {
            current += 1;
            continue;
        }

        // The next type of token is a number. This is different than what we have
        // seen before because a number could be any number of characters and we
        // want to capture the entire sequence of characters as one token.
        //
        //   (add 123 456)
        //        ^^^ ^^^
        //        Only two separate tokens
        //
        // So we start this off when we encounter the first number in a sequence.
        if current_char.is_digit(10) {
            // We're going to create a `value` string that we are going to push
            // characters to.
            let mut value = String::new();

            // Then we're going to loop through each character in the sequence until
            // we encounter a character that is not a number, pushing each character
            // that is a number to our `value` and incrementing `current` as we go.
            while current_char.is_digit(10) {
                value.push(current_char);
                current += 1;
                current_char = chars[current];
            }

            tokens.push(Token::Number(value));
            continue;
        }

        // We'll also add support for strings in our language which will be any
        // text surrounded by double quotes (").
        //
        //   (concat "foo" "bar")
        //            ^^^   ^^^ string tokens
        //
        // We'll start by checking for the opening quote:
        if current_char == '"' {
            // Keep a `value` variable for building up our string token.
            let mut value = String::new();

            // We'll skip the opening double quote in our token.
            current += 1;
            current_char = chars[current];

            // Then we'll iterate through each character until we reach another
            // double quote.
            while current_char != '"' {
                value.push(current_char);
                current += 1;
                current_char = chars[current];
            }

            // Skip the closing double quote.
            current += 2;

            tokens.push(Token::String(value));

            continue;
        }

        // The last type of token will be a `name` token. This is a sequence of
        // letters instead of numbers, that are the names of functions in our lisp
        // syntax.
        //
        //   (add 2 4)
        //    ^^^
        //    Name token
        //
        if current_char.is_alphabetic() {
            // Keep a `value` variable for building up our string token.
            let mut value = String::new();

            // Again we're just going to loop through all the letters pushing them to
            // a value.
            while current_char.is_alphabetic() {
                value.push(current_char);
                current += 1;
                current_char = chars[current];
            }

            // And pushing that value as a token with the type `name` and continuing.
            tokens.push(Token::Name(value));
            continue;
        }
        // Finally if we have not matched a character by now, we're going to throw
        // an error and completely exit.
        panic!("I dont know what this character is: {}", current_char)
    }
    return tokens;
}
