/**
 * Convert input text to a list of tokens
 * sammc
 */


/**
 * Given an input string, and strings that define the tokens to be created
 * returns a vector of tokens as defined above
 */
pub fn tokenize<'d, 'o, 'p>(input: String, defined_as_text: &'d str, operator_string: &'o str, punctuation_string: &'p str) -> Vec<(CharType, String)> {
    let mut token_factory: Tokenizer<'_, '_, '_, '_> = Tokenizer::new(&input, defined_as_text, operator_string, punctuation_string);
    let mut tokens: Vec<(CharType, String)> = Vec::<(CharType, String)>::new();

    // extract all the tokens from the given input string
    while token_factory.has_next() {
        let token_option: Option<(CharType, String)> = token_factory.next();

        if token_option.is_none() { continue }

        let tkn: (CharType, String) = token_option.unwrap();
        // Ignore whitespace 
        if tkn.0 == CharType::Space { continue }
        // None token means there was an error
        if tkn.0 == CharType::None { panic!("Error tokenizing input string") }
        tokens.push(tkn);
    }

    tokens
}

// Print all the tokens in a token vec
pub fn print_tokens(tokens: Vec<(CharType, String)>) {
    for tkn in tokens {
        println!("Type:[{:?}], Contains:[{}]", tkn.0, tkn.1)
    }
}


#[derive(Debug, PartialEq, Eq)]
pub enum CharType {
        Text, Integer, Space, Operator, Puctuation, None
}

// Object to parse a String for tokens
#[derive(Debug)]
pub struct Tokenizer<'i, 'd, 'o, 'p> {
    input_string: &'i [u8],
    defined_as_text: &'d str,
    operators: &'o str,
    punctuation: &'p str,
    index: usize
}
impl<'i, 'd, 'o, 'p> Tokenizer<'i, 'd, 'o, 'p> {

    // Constructor
    fn new(input: &'i String, defined_as_text: &'d str, operator_string: &'o str, punctuation_string: &'p str) -> Tokenizer<'i, 'd, 'o, 'p> {
        Tokenizer {
            input_string: input.as_bytes(),
            defined_as_text: defined_as_text, // contains characters defined to be text, as well as the normal alphabetic characters
            operators: operator_string,       // Defines the operators class of characters
            punctuation: punctuation_string,  // Defines the puctation class of characters
            index: 0
        }
    }

    fn next(&mut self) -> Option<(CharType, String)> {
        if !self.has_next() {
            return None;
        }

        // get the starting index of the token in the input byte array
        let s_ind= self.index;
        let mut e_ind = s_ind;

        // get the type of the first character
        let first_char_type: CharType = self.get_char_type(self.input_string[s_ind]);

        // collect all the characters after it that share the same type
        while e_ind < self.input_string.len() && self.get_char_type(self.input_string[e_ind]) == first_char_type{
            e_ind += 1;
        }

        // get the resulting string
        let token_string_array = &self.input_string[s_ind..(e_ind)];
        let res:Result<String, std::string::FromUtf8Error>  = String::from_utf8(token_string_array.to_vec());
        if res.is_err() { return None }
        let string_result:String = res.unwrap();
        
        // update the tokenizer's position
        self.index = e_ind;

        Some((first_char_type, string_result))
    }

    // returns true if there is more text to parse
    fn has_next(&self) -> bool {
        return self.index < self.input_string.len()
    }

    // Implicitely defines the possible types of tokens that can be encountered.
    fn get_char_type(&self, byte_value: u8) -> CharType {
        let c: char = char::from(byte_value);
        if      c.is_alphabetic() || self.defined_as_text.contains(c)   { CharType::Text        }
        else if c.is_ascii_digit()                                      { CharType::Integer     }
        else if c.is_whitespace()                                       { CharType::Space       }
        else if self.operators.contains(c)                              { CharType::Operator    }
        else if self.punctuation.contains(c)                            { CharType::Puctuation  }
        else                                                            { CharType::None        }
    }
}

