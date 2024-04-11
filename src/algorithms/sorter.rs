use std::{isize, usize};

pub struct Sorter<T> {
    compare_fn: Box<dyn Fn(&T, &T) -> i32>
}

impl<T> Sorter<T> {
    pub fn new(compare_fn: Box<dyn Fn(&T, &T) -> i32>) -> Self {
       return Sorter { compare_fn }; 
    }

    pub fn bubble_sort(&self, collection: &mut [T]) {
        let mut i: isize = (collection.len() - 1) as isize;
        let mut sorted = false;

        while !sorted {
            sorted = true;
            for j in 0..i {
                if (self.compare_fn)(&collection[j as usize], &collection[(j + 1) as usize]) <= 0 {
                    continue;
                }
                collection.swap(j as usize, (j + 1) as usize);
                sorted = false;
            }
            i -= 1;
        }
    }

    pub fn selection_sort(&self, collection: &mut [T]) {
        let length = collection.len();
        for i in 0..length {
            let mut min_index = i;
            for j in (i + 1)..length {
                if (self.compare_fn)(&collection[j], &collection[min_index]) >= 0 {
                    continue;
                }
                min_index = j;
            }
            collection.swap(i, min_index);
        }
    }

    pub fn insertion_sort(&self, collection: &mut [T]) {
        for i in 1..collection.len() {
            let mut j = i;

            while j > 0 && (self.compare_fn)(&collection[j], &collection[j - 1]) < 0 {
                collection.swap(j, j - 1);
                j -= 1;
            }
        }
    }

    fn partition(&self, collection: &mut [T], first: isize, last: isize) -> isize {
        let pivot_index = last as usize;
        let mut left = first;
        let mut right = last - 1;

        loop {
            while (self.compare_fn)(&collection[left as usize], &collection[pivot_index]) < 0 {
                left += 1;
            }
            while (self.compare_fn)(&collection[right as usize], &collection[pivot_index]) > 0 && right > left {
                right -= 1;
            }
            if left >= right {
                break;
            }
            collection.swap(left as usize, right as usize);
            left += 1;
        }
        collection.swap(left as usize, last as usize);
        return left;
    }

    fn quick_sort_recursion(&self, collection: &mut [T], first: isize, last: isize) {
        if first >= last {
            return;
        }

        let pivot_index = self.partition(collection, first, last);
        self.quick_sort_recursion(collection, first, pivot_index - 1);
        self.quick_sort_recursion(collection, pivot_index + 1, last);
    }

    pub fn quick_sort(&self, collection: &mut [T]) {
        self.quick_sort_recursion(collection, 0, (collection.len() - 1) as isize);
    }
}

impl<T : Clone> Sorter<T> {
    fn merge(&self, collection: &mut [T], temp: &mut Vec<T>, first: usize, mid: usize, last: usize) {        
        let mut i = first;
        let mut j = mid + 1;

        while i <= mid && j <= last {
            if (self.compare_fn)(&collection[i], &collection[j]) <= 0 {
                temp.push(collection[i].clone());
                i += 1;
            } else {
                temp.push(collection[j].clone());
                j += 1;
            }
        }
        while i <= mid {
            temp.push(collection[i].clone());
            i += 1;
        }
        while j <= last {
            temp.push(collection[j].clone());
            j += 1;
        }
        for k in first..=last {
            collection[last + first - k] = temp.pop().unwrap();
        }
    }

    fn merge_sort_recursion(&self, collection: &mut [T], temp: &mut Vec<T>, first: usize, last: usize) {
        if first >= last {
            return;
        }

        let mid = (first + last) / 2;
        self.merge_sort_recursion(collection, temp, first, mid);
        self.merge_sort_recursion(collection, temp, mid + 1, last);
        self.merge(collection, temp, first, mid, last);
    }

    pub fn merge_sort(&self, collection: &mut [T]) {
        let size = collection.len();
        if size == 0 {
            return;
        }
        let mut temp: Vec<T> = Vec::with_capacity(size);
        self.merge_sort_recursion(collection, &mut temp, 0, size - 1); 
    }
}
