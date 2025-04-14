#[derive(Debug)]
struct Heap<T: Copy> {
    a: Vec<T>,
    size: usize,
}

impl<T: Copy + std::cmp::PartialOrd> Heap<T> {
    fn new(a: Vec<T>) -> Self {
        let size = a.len();
        Heap { a, size }
    }

    #[allow(dead_code)]
    fn build_heap(&mut self) {
        let mut i = self.size as i32 / 2;
        while i >= 0 {
            self.heapify(i as usize);
            i -= 1;
        }
    }

    fn heapify(&mut self, i: usize) {
        let mut i = i;
        let a = &mut self.a;

        while i < self.size / 2 {
            let l = 2 * i + 1;
            let r = 2 * i + 2;
            let mut largest = i;

            if l < self.size && a[l] > a[i] {
                largest = l;
            }

            if r < self.size && a[r] > a[l] {
                largest = r;
            }

            if largest == i {
                return;
            }

            a.swap(i as usize, largest as usize);
            i = largest;
        }
    }
}

impl<T: Copy + std::cmp::PartialOrd + std::ops::AddAssign<i32>> Heap<T> {
    fn pop(&mut self) -> Option<T> {
        if self.size < 1 {
            return None;
        }

        let a = &mut self.a;
        let top = a[0];

        if let Some(last) = a.pop() {
            self.size -= 1;

            if self.size > 0 {
                a[0] = last;
                self.heapify(0);
            }
        }

        Some(top)
    }

    fn insert(&mut self, delta: T) {
        self.a.push(delta);
        self.size += 1;

        // println!("Size: {}", self.size);
        self.increase_key(self.size - 1, delta);
    }

    fn increase_key(&mut self, i: usize, delta: T) {
        let a = &mut self.a;

        if self.size < i || a[i] > delta {
            return;
        }

        a[i] = delta;

        let mut i = i;

        while i > 0 {
            let p = i / 2;
            if a[p] < a[i] {
                a.swap(i, p);
                i = p;
            } else {
                break;
            }
        }
    }

    fn decrease_key(&mut self, i: usize, delta: T) {
        if self.size < i || self.a[i] < delta {
            return;
        }
        self.a[i] = delta;

        self.build_heap();
    }
}

fn main() {
    let mut heap = Heap::new(vec![7, 5, 6, 3, 4, 1, 2]);

    let item = heap.pop();
    println!("POP: {item:?}");

    let item = heap.pop();
    println!("POP: {item:?}");

    let item = heap.pop();
    println!("POP: {item:?}");

    println!("Heap: {heap:?}");

    heap.increase_key(3, 45);
    println!("INC_K 3: {heap:?}");

    heap.increase_key(2, 46);
    println!("INC_K 2: {heap:?}");

    heap.decrease_key(0, 45);
    println!("INC_K 0: {heap:?}");

    heap.decrease_key(3, 1);
    println!("DEC_K 0: {heap:?}");

    heap.decrease_key(2, 3);
    println!("DEC_K 2: {heap:?}");

    heap.insert(23);
    println!("INSERT 23: {heap:?}");

    heap.insert(1);
    println!("INSERT 1: {heap:?}");

    heap.insert(25);
    println!("INSERT 25: {heap:?}");

    heap.increase_key(6, 6);
    println!("INC_K 6: {heap:?}");

    while let Some(item) = heap.pop() {
        println!("POP: {item:?}");
    }
}
