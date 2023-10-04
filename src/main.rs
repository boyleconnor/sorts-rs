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

        let mut pivot_start = 0;
        let mut pivot_end = 0;
        for scanner in 0..list.len() {
            if list[scanner] < pivot {
                let x = list[pivot_start].clone();
                list[pivot_start] = list[scanner].clone();
                list[scanner] = list[pivot_end].clone();
                list[pivot_end] = x;

                pivot_start += 1;
                pivot_end += 1;
            } else if list[scanner] == pivot {
                let x = list[scanner].clone();
                list[scanner] = list[pivot_end].clone();
                list[pivot_end] = x;

                pivot_end += 1;
            }
        }

        let (first_half, partition_and_second_half) = list.split_at_mut(pivot_start);
        let (_partition, second_half) = partition_and_second_half.split_at_mut(pivot_end - pivot_start);
        quick_sort(first_half);
        quick_sort(second_half);
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

#[test]
fn test_quicksort() {
    let mut list = vec![23, 10, 18, 14, 20, 20, 13, 13, 13, 13, 13, 13, 21, 9, 7, 8, 8, 8];
    quick_sort(&mut list);
    assert_eq!(list, vec![7, 8, 8, 8, 9, 10, 13, 13, 13, 13, 13, 13, 14, 18, 20, 20, 21, 23]);
}
