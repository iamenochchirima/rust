// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>


struct Student {
    name : String,
    assignment: Option<i32>
}

fn main() {
    let student = Student {
        name: "Enoch".to_owned(),
        assignment: Some(4)
    };

    println!("Student: {:?}", student.name);
    match student.assignment {
        Some(val) => println!("Locker: {:?}", val),
        None => println!("Student doesnt have a locker")
    };
}
