use std::{fmt, fs::File};

use genpdf::Size;
use indexmap::IndexMap;
use prettytable::Table;

pub struct EvaluatorResult {
    pub(crate) result: Vec<IndexMap<String, bool>>
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
    fn get_table(&self)-> Table {
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
    pub fn save_to_html(&self,file_name: &str) {
        let table = self.get_table();
        let mut out = File::create(file_name).unwrap();
        table.print_html(&mut out).unwrap();
    }
    pub fn save_to_csv(&self,file_name: &str) {
        let table = self.get_table();
        let mut out = File::create(file_name).unwrap();
        table.to_csv(&mut out).unwrap();
    }

    pub fn save_to_pdf(&self, file_name: &str) {
        use genpdf::elements;
         let default_font =
         genpdf::fonts::from_files("files", "calibri", None)
            .expect("Failed to load the default font family");
        let mut doc = genpdf::Document::new(default_font);
        doc.set_title("Truth Table");
        doc.set_font_size(14);
        doc.set_line_spacing(1.5);
        
        if let Some(header) = self.result.first() {
            let header:Vec<&String> = header.iter().map(|x| x.0).collect();
            let total_chars = header.iter().map(|x|x.len()).sum::<usize>();
            let w = total_chars as f32 * 0.352778 * 10.0;
            doc.set_paper_size(Size{ width: (w).into(), height: 100.into() });
            let weights: Vec<usize> = header.iter().map(|x| x.len().max(5)).collect();
            let mut table = elements::TableLayout::new(weights);
            table.set_cell_decorator(elements::FrameCellDecorator::new(true, true, true));
            let mut row = table.row();
            for h in header {
                let mut e = elements::Paragraph::new(h);
                e.set_alignment(genpdf::Alignment::Center);
                row.push_element(e);
            }
            row.push().expect("Invalid table row");
            for r in self.result.iter() {
                let values:Vec<String> = r.iter().map(|x| x.1.to_string()).collect();
                let mut row = table.row();
                for ri in values {
                    let mut e = elements::Paragraph::new(ri);
                    e.set_alignment(genpdf::Alignment::Center);
                    row.push_element(e);
                }
                row.push().expect("Invalid table row");
            }
            
            doc.push(table);
            doc.render_to_file(file_name).expect("Failed to write PDF file");
        }
    }
}