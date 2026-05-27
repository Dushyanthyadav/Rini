
// This is parts of the ini file
pub enum Event<'a> {
    Section(&'a str),
    Property((&'a str, &'a str)),
    Comment(&'a str),
}

// This is types of erros 
pub enum Error{
    UnexpectedCharacter(usize),
    MissingBracket(usize)
}

//This is the actual parser which is doing the work
pub struct Parser<'a> {
    content: &'a [u8],
    cursor: usize
}

impl<'a> Parser<'a> {
    fn is_eof(&self) -> bool {
        let length = self.content.len();
        self.cursor >= length
    }
    
    //peek the current value
    fn peek(&self) -> Option<u8> {
        match self.is_eof() {
            true => None,
            false => {
                Some(self.content[self.cursor])
            }
        }
    }
    
    //Advance the cursor
    fn advance(&mut self) {
        match self.is_eof() {
            true => (),
            false => self.cursor += 1
        }
    }

    // It skips the whitespace
    fn skip_whitespace(&mut self) {
        while let Some(character) = self.peek() {
            if (character == b' ') || (character == b'\t') {
                self.advance();
            } else {
                break;
            }
        }
    }


}

//Iterator to move next
impl<'a> Iterator for Parser<'a> {
    type Item = Result<Event<'a>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}