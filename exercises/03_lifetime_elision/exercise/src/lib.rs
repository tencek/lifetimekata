pub fn example_a(_number: &i32) -> (&i32, &i32) {
    (_number, _number)
}

pub fn example_b(_first_arg: &i32, _second_arg: &i32, _third_arg: &Option<&i32>) {
    unimplemented!()
}

pub fn example_c<'a>(_first_arg: &'a i32, _second_arg: &'a i32) -> &'a i32 {
    unimplemented!()
}

pub fn example_d<'a>(_first_arg: &'a i32, _second_arg: &i32) -> &'a i32 {
    unimplemented!()
}
