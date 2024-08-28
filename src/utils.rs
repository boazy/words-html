pub trait StringExt {
    fn escape_template_literal(&self) -> String;
}

impl StringExt for str {
    fn escape_template_literal(&self) -> String {
        let mut result = String::with_capacity(self.len() * 2);
        for c in self.chars() {
            match c {
                '`' => result.push_str("\\`"),
                '$' => result.push_str("\\$"),
                '\\' => result.push_str("\\\\"),
                _ => result.push(c),
            }
        };
        result
    }
}