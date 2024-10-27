use std::fs::File;
use crate::evaluator_result::EvaluatorResult;

#[allow(dead_code)]
impl EvaluatorResult {
    pub fn save_to_csv(&self,file_name: &str) {
        let table = self.get_table();
        let mut out = File::create(file_name).unwrap();
        table.to_csv(&mut out).unwrap();
    }
}