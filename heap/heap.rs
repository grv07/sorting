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

    fn update_key(&mut self, i: usize, delta: i32) {
        self.a[i] += delta;
        self.build_heap();
    }
}

fn main() {
    let mut heap = Heap::new(vec![7, 5, 6, 3, 4, 1, 2]);

    let item = heap.pop();
    println!("Res: {item:?}");

    let item = heap.pop();
    println!("Res: {item:?}");

    let item = heap.pop();
    println!("Res: {item:?}");

    println!("Heap: {heap:?}");

    heap.update_key(3, 45);
    println!("Heap: {heap:?}");

    let item = heap.pop();
    println!("Res: {item:?}");
}
