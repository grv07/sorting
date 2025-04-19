#[derive(Debug)]
struct Heap<T> {
    a: Vec<T>,
    size: usize,
}

impl Heap<i32> {
    fn heapify(&mut self, i: usize) {
        let n = self.size;
        let mut i = i;

        while i < n {
            let mut largest = i;

            let l = i * 2 + 1;
            let r = i * 2 + 2;

            if n > l && self.a[l] > self.a[largest] {
                largest = l;
            }

            if n > r && self.a[r] > self.a[largest] {
                largest = r;
            }

            if largest == i {
                return;
            }

            self.a.swap(i, largest as usize);
            i = largest;
        }
    }

    fn build_heap(&mut self) {
        let n = self.size;
        for i in (0..((n / 2) + 1)).rev() {
            self.heapify(i);
        }
    }

    fn heap_sort(&mut self) {
        while self.size > 0 {
            self.a.swap(self.size - 1, 0);

            self.size -= 1;

            self.heapify(0);
        }
    }

    fn pop(&mut self) -> Option<i32> {
        if self.size <= 0 {
            return None;
        }

        let item = self.a[0];

        self.a.swap(0, self.size - 1);
        self.size -= 1;

        self.heapify(0);

        Some(item)
    }

    fn push(&mut self, v: i32) {
        self.a.push(v);
        self.size += 1;
        let mut i = self.size - 1;

        while i > 0 {
            let p = (f32::ceil(i as f32 / 2.0) - 1.0) as usize;

            if self.a[p] > v {
                break;
            }

            self.a.swap(i, p);
            println!("Res: {:?} for p {:?} i: {i}", self.a, p);
            i = p;
        }
    }
}

fn main() {
    let a = vec![1, 2, 3, 4, 5];
    let n = a.len();
    let mut heap = Heap { a, size: n };

    heap.build_heap();
    println!("Res: {heap:?}");

    heap.heap_sort();
    println!("Res: {heap:?}");

    let a = vec![1, 1, 1, 1];
    let n = a.len();
    let mut heap = Heap { a, size: n };
    heap.build_heap();
    heap.heap_sort();
    println!("Res: {heap:?}");

    let a = vec![1, 2, 2, 2, 2, 1];
    let n = a.len();
    let mut heap = Heap { a, size: n };

    heap.build_heap();
    println!("Res: {heap:?}");

    heap.heap_sort();
    println!("Res: {heap:?}");

    let a = vec![1, 2, 2, 2, 2, 1, -190];
    let n = a.len();
    let mut heap = Heap { a, size: n };

    heap.build_heap();
    println!("Res: {heap:?}");

    heap.heap_sort();
    println!("Res: {heap:?}");

    let a = vec![1, 2, 2, 2, 2, 1, -190];
    let n = a.len();
    let mut heap = Heap { a, size: n };

    heap.build_heap();
    println!("Res: {heap:?}");
    while let Some(item) = heap.pop() {
        println!("POP: {item:?}");
    }

    let a = vec![1, 2, 2, 2, 2, 1, -190];
    let n = a.len();
    let mut heap = Heap { a, size: n };

    heap.build_heap();
    println!("Res: {heap:?}");

    heap.push(23);
    println!("Res: {heap:?}");
}
