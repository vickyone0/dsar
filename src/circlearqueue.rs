struct CircularQueue {
    data: Vec<i32>,
    head: usize,
    tail: usize,
    size: usize,
    capacity: usize,
}


impl CircularQueue {
    fn new(capacity: usize) -> Self {
        CircularQueue {
            data: vec![0; capacity],
            head: 0,
            tail: 0,
            size: 0,
            capacity,
        }
    }

    fn enqueue(&mut self, val: i32) -> bool {
        if self.size == self.capacity {
            return false;
        }
        self.data[self.tail] = val;
        self.tail = (self.tail + 1) % self.capacity;
        self.size += 1;
        true
    }

    fn dequeue(&mut self) -> Option<i32> {
        if self.size == 0 {
            return None;
        }
        let val = self.data[self.head];
        self.head = (self.head + 1) % self.capacity;
        self.size -= 1;
        Some(val)
    }

    fn front(&self) -> Option<i32> {
        if self.size == 0 {
            return None;
        }
        Some(self.data[self.head])
    }
    fn rear(&self) -> Option<i32> {
        if self.size == 0 {
            return None;
        }
        let rear_index = if self.tail == 0 {
            self.capacity - 1
        } else {
            self.tail - 1
        };
        Some(self.data[rear_index])
    }
    fn is_empty(&self) -> bool {
        self.size == 0
    }
    fn is_full(&self) -> bool {
        self.size == self.capacity
    }
}