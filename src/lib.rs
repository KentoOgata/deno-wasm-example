use std::collections::HashMap;

use wasm_bindgen::prelude::*;

fn fib_main(cache: &mut HashMap<i64, i64>, n: i64) -> i64 {
    if !cache.contains_key(&n) {
        let value = match n {
            0 | 1 => n,
            _ => fib_main(cache, n - 2) + fib_main(cache, n - 1),
        };
        cache.insert(n, value);
    }
    return cache[&n];
}

#[wasm_bindgen]
pub fn fib(n: i64) -> i64 {
    let mut cache = HashMap::new();
    fib_main(&mut cache, n)
}

#[cfg(test)]
mod tests_fib {
    use crate::fib;

    #[test]
    fn tests_fib() {
        assert_eq!(0, fib(0));
        assert_eq!(1, fib(1));
        assert_eq!(1, fib(2));
        assert_eq!(2, fib(3));
        assert_eq!(3, fib(4));
        assert_eq!(5, fib(5));
        assert_eq!(8, fib(6));
        assert_eq!(13, fib(7));
        assert_eq!(21, fib(8));
    }
}
