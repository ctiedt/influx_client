use std::fmt::Display;

pub struct Sort<'a> {
    columns: &'a [&'a str],
    desc: bool,
}

impl<'a> Sort<'a> {
    pub fn new(columns: &'a [&'a str], desc: bool) -> Self {
        Self { columns, desc }
    }
}

impl<'a> Display for Sort<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let columns = if self.columns.is_empty() {
            String::new()
        } else {
            let escaped: Vec<String> = self.columns.iter().map(|c| format!("\"{}\"", c)).collect();
            format!("columns: [{}]", escaped.join(", "))
        };
        let desc = if self.desc {
            "desc: true"
        } else {
            "desc: false"
        };
        write!(f, "sort({}, {})", columns, desc)
    }
}
