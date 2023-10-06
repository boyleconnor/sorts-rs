use std::{cmp, thread};
use rand::Rng;
use rand::rngs::ThreadRng;
use tokio::time::Instant;

fn random_range(rng: &mut ThreadRng, n: usize, lower: usize, upper: usize) -> Vec<usize> {
    (0..n).map(|_| rng.gen_range(lower..upper)).collect::<Vec<usize>>()
}

fn thread_quick_sort<T: PartialOrd + Clone + Send>(list: &mut [T], num_threads: u8) {
    if list.len() > 1 {
        let pivot = list[0].clone();

        let (pivot_start, pivot_end) = wide_lomuto_partition(list, pivot);

        let (first_half, partition_and_second_half) = list.split_at_mut(pivot_start);
        let (_partition, second_half) = partition_and_second_half.split_at_mut(pivot_end - pivot_start);

        if num_threads > 1 {
            let left_share = (first_half.len() as f64 / (first_half.len() + second_half.len()) as f64) * num_threads as f64;
            let left_num_threads = cmp::max(cmp::min(left_share.round() as u8, num_threads - 1), 1);
            thread::scope(|s| {
                s.spawn(|| { thread_quick_sort(first_half, left_num_threads); });
                thread_quick_sort(second_half, num_threads - left_num_threads);
            });
        } else {
            quick_sort(first_half);
            quick_sort(second_half);
        }
    }
}

fn rayon_quick_sort<T: PartialOrd + Clone + Send>(list: &mut [T]) {
    if list.len() > 1 {
        let pivot = list[0].clone();

        let (pivot_start, pivot_end) = wide_lomuto_partition(list, pivot);

        let (first_half, partition_and_second_half) = list.split_at_mut(pivot_start);
        let (_partition, second_half) = partition_and_second_half.split_at_mut(pivot_end - pivot_start);

        rayon::join(
            || { rayon_quick_sort(first_half); },
            || { rayon_quick_sort(second_half); }
        );
    }
}

fn quick_sort<T: PartialOrd + Clone>(list: &mut [T]) {
    if list.len() > 1 {
        // Partition:
        // FIXME: This is a bad way to pick the pivot
        let pivot = list[0].clone();

        let (pivot_start, pivot_end) = wide_lomuto_partition(list, pivot);

        let (first_half, partition_and_second_half) = list.split_at_mut(pivot_start);
        let (_partition, second_half) = partition_and_second_half.split_at_mut(pivot_end - pivot_start);
        quick_sort(first_half);
        quick_sort(second_half);
    }
}

fn wide_lomuto_partition<T: PartialOrd + Clone>(list: &mut [T], pivot: T) -> (usize, usize) {
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
    (pivot_start, pivot_end)
}

fn max_heapify<T: PartialOrd + Ord + Clone>(list: &mut [T]) {
    // TODO: This makes max-heaps; generalize to either comparator
    for x in 0..list.len() {
        let heap_start = list.len() - x - 1;
        sift_down(list, heap_start);
    }
}

fn sift_down<T: PartialOrd + Ord + Clone>(list: &mut [T], x: usize) {
    // TODO: See TODO on max-heapify
    let mut i = x;
    let (mut left_child, mut right_child) = (2 * i + 1, 2 * i + 2);
    while right_child < list.len() {
        let greater_child = cmp::max_by_key(
            left_child, right_child, |&child| list[child].clone());
        if list[greater_child] > list[i] {
            let swap = list[i].clone();
            list[i] = list[greater_child].clone();
            list[greater_child] = swap;
            i = greater_child;
            (left_child, right_child) = (2 * i + 1, 2 * i + 2);
        } else {
            break;
        }
    }
    if left_child < list.len() && list[left_child] > list[i] {
        let swap = list[i].clone();
        list[i] = list[left_child].clone();
        list[left_child] = swap;
    }
}

fn heap_sort<T: PartialOrd + Ord + Clone>(list: &mut [T]) {
    max_heapify(list);
    let mut heap_length = list.len();
    while heap_length > 1 {
        heap_length -= 1;

        let x = list[0].clone();
        list[0] = list[heap_length].clone();
        list[heap_length] = x;

        let (heap, _sorted_list) = list.split_at_mut(heap_length);
        sift_down(heap, 0);
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let list = random_range(&mut rng, 5_000_000, 0, 5_000_000);

    let mut rayon_list = list.clone();
    let start = Instant::now();
    rayon_quick_sort(&mut rayon_list);
    println!("Completed rayon quick sort in: {:?}", start.elapsed());

    let mut thread_list = list.clone();
    let start = Instant::now();
    thread_quick_sort(&mut thread_list, 24);
    println!("Completed thread quick sort in: {:?}", start.elapsed());

    let mut serial_list = list.clone();
    let start = Instant::now();
    quick_sort(&mut serial_list);
    println!("Completed serial quick sort in: {:?}", start.elapsed());

    let mut heapsort_list = list.clone();
    let start = Instant::now();
    heap_sort(&mut heapsort_list);
    println!("Completed heap sort in: {:?}", start.elapsed());
}

#[test]
fn test_quicksort() {
    let mut list = vec![23, 10, 18, 14, 20, 20, 13, 13, 13, 13, 13, 13, 21, 9, 7, 8, 8, 8];
    quick_sort(&mut list);
    assert_eq!(list, vec![7, 8, 8, 8, 9, 10, 13, 13, 13, 13, 13, 13, 14, 18, 20, 20, 21, 23]);
}

#[test]
fn test_rayon_quicksort() {
    let mut list = vec![23, 10, 18, 14, 20, 20, 13, 13, 13, 13, 13, 13, 21, 9, 7, 8, 8, 8];
    rayon_quick_sort(&mut list);
    assert_eq!(list, vec![7, 8, 8, 8, 9, 10, 13, 13, 13, 13, 13, 13, 14, 18, 20, 20, 21, 23]);
}

#[test]
fn test_thread_quicksort() {
    let mut list = vec![23, 10, 18, 14, 20, 20, 13, 13, 13, 13, 13, 13, 21, 9, 7, 8, 8, 8];
    thread_quick_sort(&mut list, 24);
    assert_eq!(list, vec![7, 8, 8, 8, 9, 10, 13, 13, 13, 13, 13, 13, 14, 18, 20, 20, 21, 23]);
}

#[test]
fn test_heapify() {
    let mut list = vec![23, 8, 9, 10, 17, 10, 12, 12, 12, 0, 34, 12, 15, 13, 10, 9, 9, 2, 9, 10];
    max_heapify(&mut list);
    for i in 0..list.len() {
        let (left_child, right_child) = (2 * i + 1, 2 * i + 2);
        assert!(left_child >= list.len() || list[i] >= list[left_child]);
        assert!(right_child >= list.len() || list[i] >= list[right_child]);
    }
}

#[test]
fn test_heapsort() {
    let mut list = vec![23, 10, 18, 14, 20, 20, 13, 13, 13, 13, 13, 13, 21, 9, 7, 8, 8, 8];
    heap_sort(&mut list);
    assert_eq!(list, vec![7, 8, 8, 8, 9, 10, 13, 13, 13, 13, 13, 13, 14, 18, 20, 20, 21, 23]);
}
