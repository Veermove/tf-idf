pub struct Lexer<'a> {
    content: &'a [char],
}

impl <'a> Lexer<'a> {

    pub fn new(content: &'a [char]) -> Self {
        return Self { content };
    }

    fn trim_left(&mut self) {
        while self.content.len() > 0 && self.content[0].is_ascii_whitespace() {
            self.content = &self.content[1..]
        }
    }

    fn chop_while<P>(&mut self, mut predicate: P) -> &'a [char]
    where P: FnMut(&char) -> bool {
        let mut n = 0 as usize;
        while self.content.len() > 0 && n < self.content.len() && predicate(&self.content[n]) {
            n += 1;
        }

        return self.chop(n)
    }

    fn chop(&mut self, n: usize) -> &'a [char] {
        let token = &self.content[0..n];
        self.content = &self.content[n..];
        return token;
    }

    pub fn next_token(&mut self) -> Option<&'a [char]> {
        self.trim_left();

        if self.content.len() == 0 {
            return None;
        }

        if self.content[0].is_alphabetic() {
            return Some(self.chop_while(|c| c.is_alphanumeric()))
        }

        if self.content[0].is_ascii_digit() {
            return Some(self.chop_while(|c| c.is_numeric()))
        }

        return Some(self.chop(1));
    }

}

impl <'a> Iterator for Lexer<'a> {
    type Item = &'a [char];

    fn next(&mut self) -> Option<Self::Item> {
        return self.next_token()
    }
}

// fn unescape(s: &[u8]) -> Result<&[char], Box<dyn Error>> {
//     let mut output = Vec::new();
//     let mut i = 0;

//     while i < s.len() {
//         match s[i] {
//             b'\\' => {
//                 i += 1;
//                 match s[i] {
//                     b'u' => {
//                         let num = u8::from_str_radix(std::str::from_utf8(&s[i+1..][..4])?, 16)?;
//                         output.push(num);
//                         i += 4;
//                     }
//                     byte => output.push(byte),
//                 }
//             },
//             byte => output.push(byte),
//         }
//         i += 1;
//     }
//     Ok(&String::from_utf8(output)?.chars().collect::<Vec<_>>())
// }
