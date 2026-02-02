use zephyr::raw::ENOTSUP;

pub fn divide_numbers(a: &u8, b: &u8, c: &mut f32) -> i32 {
    if *b == 0 {
        ENOTSUP.try_into().unwrap()
    } else {
        *c = *a as f32 / *b as f32;
        0
    }
}