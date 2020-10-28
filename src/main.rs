use rand::seq::SliceRandom;
use rand::thread_rng;

fn bogosort<T: Ord>(xs: &mut [T]) {
    fn is_sorted<T: Ord>(xs: &[T]) -> bool {
        for window in xs.windows(2) {
            if window[0] > window[1] {
                return false;
            }
        }
        true
    }

    let mut rng = thread_rng();

    while !is_sorted(xs) {
        xs.shuffle(&mut rng);
    }
}


fn main() {
    let mut xs = vec![1i32, 9, 4, 7, 5];
    println!("{:?}", xs);

    bogosort(&mut xs);
    println!("{:?}", xs);
}
