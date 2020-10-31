use std::ops::{Index, IndexMut};

/// a Heap struct. The actual elements are stored
/// in form of an array, using the following formula :
/// $$right(i) = i * 2 + 2$$
/// $$left(i) = i * 2 + 1$$
/// where i is 0-indexed
#[derive(Debug)]
pub struct Heap<'a, T> {
    pub arr: &'a mut Vec<T>,
    pub heap_size: usize,
}

impl<'a, T> Index<usize> for Heap<'a, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.arr[index]
    }
}

impl<'a, T> IndexMut<usize> for Heap<'a, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.arr[index]
    }
}

impl<'a, T> Heap<'a, T>
where
    T: Copy + PartialOrd,
{
    /// returns a new heap, consuming the mutable reference
    pub fn new(arr: &'a mut Vec<T>) -> Self {
        Heap {
            heap_size: arr.len(),
            arr,
        }
    }

    /// returns actual lenght of array
    pub fn len(&self) -> usize {
        self.arr.len()
    }

    /// returns the parent of a valid node
    pub fn parent(&self, idx: usize) -> usize {
        assert!(idx < self.heap_size);
        (idx - 1) >> 1
    }

    /// returns the next left node of a valid node
    pub fn left(&self, idx: usize) -> usize {
        // return the last element if the left elem
        // does not exits
        if idx >= self.heap_size >> 1 {
            return self.heap_size - 1;
        }
        (idx << 1) + 1
    }

    /// returns the next right node of a valid node
    pub fn right(&self, idx: usize) -> usize {
        // return the last element if the right elem
        // does not exits
        if idx + 1 >= self.heap_size >> 1 {
            return self.heap_size - 1;
        }
        (idx << 1) + 2
    }

    /// exchanges two elements of the array
    pub fn exchange(&mut self, idx1: usize, idx2: usize) {
        assert!(idx1 < self.heap_size);
        assert!(idx2 < self.heap_size);

        let temp = self[idx1];
        self[idx1] = self[idx2];
        self[idx2] = temp;
    }

    /// the element is given index is put to it's place as
    /// per the condition of max heap
    pub fn max_heapify(&mut self, idx: usize) {
        let (l, r) = (self.left(idx), self.right(idx));

        // determining the largest of values at index idx, l & r
        let mut largest = if self[l] > self[idx] { l } else { idx };
        if self[r] > self[largest] {
            largest = r
        };

        if largest != idx {
            self.exchange(largest, idx);
        } else {
            return;
        }

        let limit = self.heap_size >> 1;

        while largest < limit {
            let (l, r) = (self.left(largest), self.right(largest));

            // determining the largest of values at index idx, l & r
            let mut new_largest = if self[l] > self[largest] { l } else { largest };
            if self[r] > self[new_largest] {
                new_largest = r
            };

            if new_largest != largest {
                self.exchange(largest, new_largest);
                largest = new_largest;
            } else {
                return;
            }
        }
    }

    /// the element is given index is put to it's place as
    /// per the condition of min heap
    pub fn min_heapify(&mut self, idx: usize) {
        let (l, r) = (self.left(idx), self.right(idx));

        // determining the smallest of values at index idx, l & r
        let mut smallest = if self[l] < self[idx] { l } else { idx };
        if self[r] < self[smallest] {
            smallest = r
        };

        if smallest != idx {
            self.exchange(smallest, idx);
        } else {
            return;
        }

        let limit = self.heap_size >> 1;

        while smallest < limit {
            let (l, r) = (self.left(smallest), self.right(smallest));

            // determining the smallest of values at index idx, l & r
            let mut new_smallest = if self[l] < self[smallest] {
                l
            } else {
                smallest
            };
            if self[r] < self[new_smallest] {
                new_smallest = r
            };

            if new_smallest != smallest {
                self.exchange(smallest, new_smallest);
                smallest = new_smallest;
            } else {
                return;
            }
        }
    }

    /// convert the heap to a max heap
    pub fn to_max_heap(&mut self) {
        for i in (0..(self.heap_size / 2)).rev() {
            self.max_heapify(i);
        }
    }

    /// convert the heap to a min heap
    pub fn to_min_heap(&mut self) {
        for i in (0..(self.heap_size / 2)).rev() {
            self.min_heapify(i);
        }
    }

    /// deletes the node at ith position
    pub fn delete(&mut self, idx: usize) -> T {
        assert!(self.heap_size > 0);
        self.exchange(idx, self.heap_size - 1);

        if self[idx] < self[self.heap_size - 1] {
            self.heap_size -= 1;
            self.max_heapify(idx);
        } else {
            self.reevaluate(idx);
        }

        self.arr.pop().unwrap()
    }

    /// use is increasing a node's priority
    pub fn reevaluate(&mut self, idx: usize) {
        assert!(self[idx] < self[self.right(idx)] && self[idx] < self[self.left(idx)]);

        let mut i = self.parent(idx);
        let mut parent = self.parent(i);

        let key = self[i];

        while i > 0 && self[parent] < self[i] {
            self[i] = self[parent];
            i = parent;
            parent = self.parent(i);
        }
        self[i] = key;
    }

    /// insert elements into queue
    /// assumes length == heap size
    pub fn push(&mut self, elem: T) {
        self.arr.push(elem);
        self.heap_size += 1;
        self.reevaluate(self.heap_size - 1);
    }

    /// immutable reference to top element
    pub fn peek(&self) -> &T {
        &self.arr[0]
    }

    /// remove the top element
    pub fn pop(&mut self) -> T {
        self.delete(0)
    }
}

type PriorityQueue<'a, T> = Heap<'a, T>;

#[cfg(test)]
mod test {
    use super::*;

    // TODO: write more tests, complete tests of all methods

    #[test]
    fn max_heapify() {
        let mut a = vec![17, 3, 16, 13, 10, 1, 5, 7, 12, 4, 8, 9, 0];
        let mut a = Heap::new(&mut a);

        a.max_heapify(2);

        println!("{:?}", a);
    }
}
