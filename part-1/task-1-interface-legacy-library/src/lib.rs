#[allow(dead_code)]
mod ffi {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]

    pub(crate) mod bindings;
}

pub fn ask(data: u32) -> u32 {
    let input = ffi::bindings::Nonsense_Input { data };
    unsafe { ffi::bindings::Nonsense_Ask(&input) }
}

pub fn add(a: f64, b: f64) -> f64 {
    unsafe { ffi::bindings::Math_Arithmetic_Add(a, b) }
}

pub fn subtract(a: f64, b: f64) -> f64 {
    unsafe { ffi::bindings::Math_Arithmetic_Subtract(a, b) }
}

pub fn multiply(a: f64, b: f64) -> f64 {
    unsafe { ffi::bindings::Math_Arithmetic_Multiply(a, b) }
}

pub fn divide(a: f64, b: f64) -> f64 {
    unsafe { ffi::bindings::Math_Arithmetic_Divide(a, b) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        let result = ask(128);
        assert_eq!(result, 42);
    }
}
