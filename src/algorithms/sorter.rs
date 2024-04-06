use std::usize;

pub struct Sorter<T> {
    compare_fn: Box<dyn Fn(&T, &T) -> i32>
}

impl<T> Sorter<T> {
    pub fn new(compare_fn: Box<dyn Fn(&T, &T) -> i32>) -> Self {
       return Sorter { compare_fn }; 
    }

    pub fn bubble_sort(&self, collection: &mut [T]) {
        let mut i = collection.len() - 1;
        let mut sorted = false;

        while !sorted && i > 0 {
            sorted = true;
            for j in 0..i {
                if (self.compare_fn)(&collection[j], &collection[j + 1]) <= 0 {
                    continue;
                }
                collection.swap(j, j + 1);
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
        return self.quick_sort_recursion(collection, 0, (collection.len() - 1) as isize);
    }
}
