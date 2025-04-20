#[derive(Debug)]
struct Heap {
    a: Vec<i32>,
    size: usize,
}

impl Heap {
    fn new(a: Vec<i32>) -> Self {
        let size = a.len();
        Self { a, size }
    }

    fn heapify(&mut self, i: usize) {
        let mut i = i;
        let a = &mut self.a;
        while i < self.size / 2 {
            let mut largest = i;
            let r = 2 * i + 1;
            let l = 2 * i + 2;

            if l < self.size && a[largest] < a[l] {
                largest = l;
            }

            if r < self.size && a[largest] < a[r] {
                largest = r;
            }

            if largest == i {
                return;
            }

            a.swap(largest, i);
            i = largest;
        }
    }

    fn build_heap(&mut self) {
        for i in (0..self.size / 2).rev() {
            self.heapify(i);
        }
    }

    fn pop(&mut self) -> Option<i32> {
        if self.size <= 0 {
            return None;
        }

        let item = self.a[0];

        if let Some(last) = self.a.pop() {
            self.size -= 1;
            if self.size > 0 {
                self.a[0] = last;
            }
            self.heapify(0);

            Some(item)
        } else {
            None
        }
    }

    fn sort(&mut self) {
        while self.size > 1 {
            self.a.swap(0, self.size - 1);
            self.size -= 1;
            self.heapify(0);
        }
        self.size = self.a.len();
    }

    fn insert(&mut self, v: i32) {
        let a = &mut self.a;
        self.size += 1;
        a.push(v);

        let mut i = self.size; // index from 1

        while i > 1 {
            let p = i / 2; // index from 1
            if a[p - 1] > a[i - 1] {
                return;
            }

            a.swap(p - 1, i - 1);
            i = p;
        }
    }
}

fn solve(a: Vec<i32>) -> i32 {
    let mut heap = Heap::new(a);
    heap.build_heap();

    while heap.size > 1 {
        if let (Some(a), Some(b)) = (heap.pop(), heap.pop()) {
            let d = (a - b).abs();
            heap.insert(d);
        }
    }

    heap.pop().unwrap_or_default()
}

fn main() {
    let mut heap = Heap::new(vec![1, 2, 3, 4, 5, 6, 7]);
    heap.build_heap();
    heap.sort();

    heap.insert(-1);

    heap.build_heap();
    heap.sort();

    println!("Heap: {heap:?}");

    let res = solve(vec![1, 2, 3, 4, 5, 6, 7]);
    println!("Res: {res}");
    let res = solve(vec![2, 7, 4, 1, 8, 1]);
    println!("Res: {res}");

    let res = solve(vec![2, 7, 4, 1]);
    println!("Res: {res}");

    let res = solve(vec![2]);
    println!("Res: {res}");

    assert_eq!(solve(vec![2, 7, 4, 1, 8, 1]), 1); // typical case
    assert_eq!(solve(vec![10, 4, 2, 10]), 2); // two equal large numbers
    assert_eq!(solve(vec![1, 1]), 0); // equal stones cancel each other
    assert_eq!(solve(vec![9]), 9); // single stone remains
    assert_eq!(solve(vec![]), 0); // no stones
    assert_eq!(solve(vec![3, 7, 2, 5]), 1); // multiple rounds
    assert_eq!(solve(vec![31, 26, 33, 21, 40]), 9); // more complex diff rounds
    assert_eq!(solve(vec![5, 5, 5, 5]), 0); // all stones same
    assert_eq!(solve(vec![100, 50, 25]), 25); // uneven smash steps
}
