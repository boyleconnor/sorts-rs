use std::fmt::{Debug, Display};
use rand::Rng;
use rand::rngs::ThreadRng;
use tokio::time::Instant;

fn random_range(rng: &mut ThreadRng, n: usize, lower: usize, upper: usize) -> Vec<usize> {
    (0..n).map(|_| rng.gen_range(lower..upper)).collect::<Vec<usize>>()
}

fn quick_sort<T: PartialOrd + Clone + Display + Debug>(list: &mut [T]) {
    if list.len() > 1 {
        // Partition:
        // FIXME: This is a bad way to pick the pivot
        let pivot = list[0].clone();

        let mut pivot_index = 0;
        for scanner in 0..list.len() {
            if list[scanner] < pivot {
                let x = list[scanner].clone();
                list[scanner] = list[pivot_index].clone();
                list[pivot_index] = x;

                pivot_index += 1;
            }
        }

        let (first_half, second_half) = list.split_at_mut(pivot_index);
        quick_sort(first_half);
        quick_sort(&mut second_half[1..]);
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    // FIXME: Quicksort is very slow on narrow ranges (e.g. upper=100)
    let mut list = random_range(&mut rng, 500_000, 0, 100);

    let start = Instant::now();
    quick_sort(&mut list);
    println!("Completed quick sort in: {:?}", start.elapsed());

}
