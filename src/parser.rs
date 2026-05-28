use core::str::{self, from_utf8};
// This is parts of the ini file
#[derive(Debug)]
pub enum Event<'a> {
    Section(&'a str),
    Property((&'a str, &'a str)),
    Comment(&'a str),
}

// This is types of erros 
#[derive(Debug)]
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

    pub fn new(content: &'a str) -> Self {
        Self {
            content: content.as_bytes(),
            cursor: 0,
        }
    }

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

    fn read_section(&mut self) -> Result<Event<'a>, Error> {
        self.advance();
        let start_index = self.cursor;
        loop {
            match self.peek() {
                Some(character) => {
                    if character == b']' {
                        break;
                    } else if character == b'\n' || character == b'\r' {
                        return Err(Error::MissingBracket(self.cursor))
                    } else {
                        self.advance();
                    }
                }
                None => return Err(Error::MissingBracket(self.cursor))
            }
        }
        let end_index = self.cursor;
        self.advance();

        str::from_utf8(&self.content[start_index..end_index]).map(Event::Section).map_err(|_| Error::UnexpectedCharacter(self.cursor))
    }

    fn read_property(&mut self) -> Result<Event<'a>, Error> {
        let key_start = self.cursor;
        let key;
        loop {
            match self.peek() {
                Some(character) => {
                    if character == b'=' {
                        key = &self.content[key_start..self.cursor];
                        self.advance();
                        break;
                    } else if character == b'\n' || character == b'\r' {
                        return Err(Error::UnexpectedCharacter(self.cursor));
                    } else {
                        self.advance();
                    }
                }
                None => return Err(Error::UnexpectedCharacter(self.cursor))
            }
        }
        self.skip_whitespace();
        let value_start = self.cursor;
        let value;
        loop {
            match self.peek() {
                Some(character) => {
                    if character == b'\n' || character == b'\r' {
                        value = &self.content[value_start..self.cursor];
                        break;
                    } else {
                        self.advance();
                    }
                }
                None => {
                    value = &self.content[value_start..self.cursor];
                    break;
                }
            }
        }

        let key = from_utf8(key).map_err(|_| Error::UnexpectedCharacter(self.cursor))?;
        let value = from_utf8(value).map_err(|_| Error::UnexpectedCharacter(self.cursor))?;

        Ok(Event::Property((key.trim_end(), value.trim_end())))
    }

    fn read_comment(&mut self) -> Result<Event<'a>, Error> {
        self.advance();
        let start_index = self.cursor;
        let comment;
        loop {
            match self.peek() {
                Some(character) => {
                    if character == b'\n' || character == b'\r' {
                        comment = &self.content[start_index..self.cursor];
                        break;
                    } else {
                        self.advance();
                    }
                }
                None => {
                    comment = &self.content[start_index..self.cursor];
                    break;
                }
            }
        }

        from_utf8(comment).map(|c| Event::Comment(c)).map_err(|_| Error::UnexpectedCharacter(self.cursor))
    }

}

//Iterator to move next
impl<'a> Iterator for Parser<'a> {
    type Item = Result<Event<'a>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.skip_whitespace();
            match self.peek() {
                Some(character) => {
                    if character == b'\n' || character == b'\r' {
                        self.advance();
                    } else if character == b'[' {
                        return Some(self.read_section());
                    } else if character == b';' || character == b'#' {
                        return Some(self.read_comment());
                    } else {
                        return Some(self.read_property());
                    }
                }
                None => return None
            }
        }
    }
}








