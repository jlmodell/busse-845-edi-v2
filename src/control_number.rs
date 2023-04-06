use nanoid::nanoid;

const NUMS: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

pub struct ControlNumber;

impl ControlNumber {
    pub fn create() -> String {
        nanoid!(9, &NUMS)
    }

    pub fn to_i32(control_number: &str) -> i32 {
        control_number.parse::<i32>().unwrap()
    }
}
