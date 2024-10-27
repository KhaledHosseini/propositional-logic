use std::fmt;
use indexmap::IndexMap;
use prettytable::Table;

pub struct EvaluatorResult {
    pub result: Vec<IndexMap<String, bool>>
}

impl fmt::Display for EvaluatorResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let table = self.get_table();
        table.printstd();
        write!(f, "")
    }
}
#[allow(dead_code)]
impl EvaluatorResult {
    pub(crate) fn get_table(&self)-> Table {
        let mut table = Table::new();
        if let Some(header) = self.result.first() {
            let header = header.iter().map(|x| x.0).collect();
            table.add_row(header);
        }
        for row in self.result.iter() {
            let values = row.iter().map(|x| x.1.to_string()).collect();
            table.add_row(values);
        }
        table
    }
}