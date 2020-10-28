use rand::seq::SliceRandom;
use rand::thread_rng;

fn bogosort<T: Ord>(x: &mut [T]) {
    fn is_sorted<T: Ord>(x: &[T]) -> bool {
        for window in x.windows(2) {
            if window[0] > window[1] {
                return false;
            }
        }
        true
    }

    let mut rng = thread_rng();

    while !is_sorted(x) {
        x.shuffle(&mut rng);
    }
}

fn main() {
    let mut x = vec![1i32, 9, 4, 7, 5];

    println!("{:?}", x);
    bogosort(&mut x);
    println!("{:?}", x);
}
