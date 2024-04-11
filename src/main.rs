use std::time::Instant;
use rand::prelude::*;
use crate::{algorithms::sorter::Sorter, collections::linked_list::LinkedList};
mod collections;
mod algorithms;

fn number_sorter(left: &i32, right: &i32) -> i32 {
    return left - right;
}

fn measure_time(mut sorter_fn: impl FnMut(&mut [i32]), collection: &mut [i32], name: &str) {
    let start = Instant::now();
    (sorter_fn)(collection);
    let elapsed = start.elapsed();

    println!("Elapsed time ({}): {:?}", name, elapsed);
}

fn test_linked_list() {
    let mut linked_list = LinkedList::new();

    linked_list.insert(1, 0);
    linked_list.insert(2, 1);
    linked_list.insert(4, 2);
    linked_list.insert(3, 2);
    linked_list.insert(0, 0);

    println!("{}", linked_list.to_string());
    
    linked_list.remove_at(2);
    
    println!("{}", linked_list.to_string());

}

fn test_sorters() {
    let sorter = Sorter::new(Box::new(number_sorter));
    let length = 10_000;
    let mut average_case: Vec<i32> = (0..length).collect();
    let mut inverse_case: Vec<i32> = average_case.clone();
    let ordered_case: Vec<i32> = average_case.clone();
    let mut rng = rand::thread_rng();

    average_case.shuffle(&mut rng);
    inverse_case.reverse();
    for tuple in [(average_case, "Average case"), (inverse_case, "Inverse case"), (ordered_case, "Ordered case")] {
        let array = tuple.0;
        println!("\n---------- {} ----------", tuple.1);
        measure_time(|collection| sorter.selection_sort(collection), &mut array.clone(), "Selection Sort");
        measure_time(|collection| sorter.bubble_sort(collection), &mut array.clone(), "Bubble Sort");
        measure_time(|collection| sorter.insertion_sort(collection), &mut array.clone(), "Insertion Sort");
        measure_time(|collection| sorter.quick_sort(collection), &mut array.clone(), "Quicksort");
        measure_time(|collection| sorter.merge_sort(collection), &mut array.clone(), "Merge Sort");
    }
    
}

fn main() {
    test_linked_list();
    test_sorters();
}
