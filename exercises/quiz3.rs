// quiz3.rs
// This quiz tests:
// - Generics
// - Traits
// An imaginary magical school has a new report card generation system written in Rust!
// Currently the system only supports creating report cards where the student's grade
// is represented numerically (e.g. 1.0 -> 5.5).
// However, the school also issues alphabetical grades (A+ -> F-) and needs
// to be able to print both types of report card!

// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to "A+"
// to show that your changes allow alphabetical grades.

// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.

use std::fmt::Display;


//This definitely went over my head for no reason...

pub struct ReportCard<T> {
    pub grade: T, //This was the only change I knew to make, and the compiler guided me all the way through.
    pub student_name: String,
    pub student_age: u8,
}

//After changing from f32 to T, it kept kept autocompleting for me, and eventually I was back to a compiled state.
// My issue was that it told me to change the grade to A+, but I didn't realize it was asking me to change the TEST value. I thought
// that my file was weird because I already had an A+ there, and was thinking that was supposed to be a different grade, and someone filled it in
// on accident. Basically this all came down to an issue of understanding. I thought I was supposed to translate from number to letter, when in
// reality all I was supposed to do was provide support for the same variable to use numbers and letters. How to Read a Book

impl<T: Display> ReportCard<T> {
    pub fn print(&self) -> String {
        format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard {
            grade: "A+",
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}