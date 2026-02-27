pub trait ToStringWithIndent {
    fn to_string_with_indent(&self, indent: usize) -> String;
}

impl<T: ToString> ToStringWithIndent for T {
    fn to_string_with_indent(&self, indent: usize) -> String {
        let out = self.to_string();
        let indent_str = " ".repeat(indent);
        out.split('\n')
            .map(|line| format!("{}{}", indent_str, line))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
