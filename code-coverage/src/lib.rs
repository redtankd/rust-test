pub mod faculty;
pub mod stud;

pub fn add(a: i32, b: i32) -> isize {
    (a + b) as isize
}

pub fn sub(a: i32, b: i32) -> isize {
    (a + b) as isize
}

#[test]
fn test() {
    assert_eq!(10isize, add(4, 6));
}
