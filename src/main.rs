fn calculate_final_score(written_exam_score: f64, cat_score: f64) -> f64 {
    written_exam_score + cat_score
}

fn grade_course(course_type: &str, written_exam_score: f64, cat_score: f64) -> (&str, f64) {
    // Validate input scores
    if written_exam_score > 70.0 || written_exam_score < 0.0 {
        panic!("Invalid written exam score: {}. It should be between 0 and 70.", written_exam_score);
    }
    if cat_score > 30.0 || cat_score < 0.0 {
        panic!("Invalid CAT score: {}. It should be between 0 and 30.", cat_score);
    }
    
    let final_score = calculate_final_score(written_exam_score, cat_score);
    
    let grade = if course_type == "non-medical" {
        match final_score {
            score if score >= 70.0 => "A",
            score if score >= 60.0 => "B",
            score if score >= 50.0 => "C",
            score if score >= 40.0 => "D",
            _ => "FAIL",
        }
    } else if course_type == "medical" {
        match final_score {
            score if score >= 70.0 => "A",
            score if score >= 60.0 => "B",
            score if score >= 50.0 => "C",
            _ => "FAIL",
        }
    } else {
        "Invalid course type"
    };
    
    (grade, final_score)
}

fn main() {
    let course_type = "medical";
    let written_exam_score = 31.0; // out of 70
    let cat_score = 24.0; // out of 30
    
    let (final_grade, final_score) = grade_course(course_type, written_exam_score, cat_score);
    println!("The final score is: {}", final_score);
    println!("The final grade is: {}", final_grade);
}




