use std::time::SystemTime;

use rand::seq::SliceRandom;
use rand::thread_rng;

fn generate_random_vector(n: isize) -> Vec<isize> {
    let mut vec: Vec<isize> = (0..n).collect();
    vec.shuffle(&mut thread_rng());
    vec
}

pub fn sort(list: Vec<isize>) -> Vec<isize> {
    mergesort(list)
}

/**
/// Sort a vector using bubble sort
#[warn(dead_code)]
fn bubblesort<T: Ord>(mut list: Vec<T>) -> Vec<T> {
    let mut sorted: bool;

    if list.len() > 1 {
        loop {
            sorted = true;
            for i in 0..list.len() - 1 {
                if i + 1 < list.len() && list[i] > list[i + 1] {
                    sorted = false;
                    list.swap(i, i + 1);
                }
            }
            if sorted {
                break;
            }
        }
    }
    list
}
**/

/// Sort a vector using merge sort
#[warn(dead_code)]
fn mergesort<T: Ord + Clone + Copy>(mut list: Vec<T>) -> Vec<T> {
    let mut left = list[..list.len() / 2].to_vec();
    let mut right = list[list.len() / 2..].to_vec();

    if left.len() > 1 {
        left = mergesort(left.clone());
    }
    if right.len() > 1 {
        right = mergesort(right.clone());
    }

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            list[k] = left[i];
            i += 1;
        } else {
            list[k] = right[j];
            j += 1
        }
        k += 1;
    }

    while i < left.len() {
        list[k] = left[i];
        i += 1;
        k += 1;
    }
    while j < right.len() {
        list[k] = right[j];
        j += 1;
        k += 1;
    }

    list
}

fn check_time(n: isize) {
    let vec: Vec<isize> = generate_random_vector(n);

    let sys_time = SystemTime::now();
    sort(vec.clone());
    let difference = sys_time.elapsed().unwrap();

    println!(
        "Sorting time on array of size {}: {:?}",
        vec.len(),
        difference
    );
}

fn main() {
    let mut vec = generate_random_vector(40);
    println!("Good day. This is a demonstration of a sorting algorith.");
    println!("Below is a shuffled list of elements:\n{:?}", vec);
    vec = sort(vec.clone());
    println!(
        "After sorting, you can clearly see the list is now ordered:\n{:?}",
        vec
    );

    for i in 1..10 {
        check_time(i * 500)
    }
}

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

#[cfg(test)]
mod tests {
    use super::*;

    #[quickcheck]
    fn sorted_output_is_sorted(xs: Vec<isize>) -> bool {
        if xs.len() <= 1 {
            return true;
        }
        let result = sort(xs.clone());

        for i in 0..(result.len() - 2) {
            if result[i] > result[i + 1] {
                println!(
                    "Found error at index {}: {} > {}",
                    i,
                    result[i],
                    result[i + 1]
                );
                return false;
            }
        }
        true
    }

    #[quickcheck]
    fn check_time(xs: Vec<isize>) -> bool {
        let sys_time = SystemTime::now();
        sort(xs.clone());
        let difference = sys_time
            .duration_since(sys_time)
            .expect("Clock may have gone backwards");
        println!(
            "Sorting time on array of size {}: {:?}",
            xs.len(),
            difference
        );
        return true;
    }
}
