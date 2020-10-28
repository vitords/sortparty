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

fn stalinsort<T>(xs: &mut Vec<T>) -> Vec<T>
where
    T: Ord + Clone + Copy
{
    let mut gulag: Vec<T> = Vec::new();
    let mut comrade = xs[0];
    let population = xs.len();

    for (i, x) in xs.clone().iter().enumerate() {
        if i < population && x < &comrade {
            // If an element is out of order, put it in the gulag
            gulag.push(xs.remove(i - gulag.len()));
        } else {
            // Elemnt is in order, move along
            comrade = *x;
        }
    }
    gulag
}

fn main() {
    let mut xs = vec![1i32, 9, 4, 7, 5];

    println!("{:?}", xs);
    bogosort(&mut xs);
    println!("BogoSort{:?}", xs);

    let mut ys = vec![1i32, 9, 4, 7, 5];
    let gulag = stalinsort(&mut ys);
    println!("StalinSort: {:?}", ys);
    println!("Gulag: {:?}", gulag);
}
