#[derive(Debug)]
struct CircularQueue<T> {
    capacity: usize,
    vector: Vec<T>,
    front: isize,
    rear: isize,
}

impl<T: std::fmt::Debug> CircularQueue<T> {
    fn create_circular_queue(capacity: usize, vector: Vec<T>, front: isize, rear: isize) -> Self {
        Self {
            capacity,
            vector,
            front,
            rear,
        }
    }

    fn check_circular_queue_empty(&self) -> bool {
        if self.front == -1 && self.rear == -1 {
            return true;
        }

        false
    }

    fn check_circular_queue_full(&self) -> bool {
        if self.front == 0 && self.rear == (self.capacity - 1) as isize
            || self.front == self.rear + 1
        {
            return true;
        }

        false
    }

    fn access_data(&self, index: usize) {
        if self.check_circular_queue_empty() {
            eprintln!(
                "Cannot access data at index {} since circular queue is empty\n",
                index
            );
        } else if index >= self.capacity {
            eprintln!(
                "Cannot access data at index {} since total capacity of circular queue is {}\n",
                index, self.capacity
            );
        } else {
            println!(
                "The data at CircularQueue[{}] is = {:?}\n",
                index, self.vector[index]
            );
        }
    }

    fn enqueue(&mut self, data: T) {
        if self.check_circular_queue_full() {
            eprintln!("Cannot add more data since circular queue is full\n");
        } else {
            if self.front == -1 {
                self.front = 0;
            }
            self.rear = (self.rear + 1) % 10;
            if self.vector.len() < self.capacity {
                self.vector.push(data);
            } else {
                self.vector[self.rear as usize] = data;
            }
        }
    }

    fn dequeue(&mut self) {
        if self.check_circular_queue_empty() {
            eprintln!("Cannot remove data since circular queue is empty\n");
        } else {
            let element = &self.vector[self.front as usize];
            println!(
                "Element removed is {:?} and was situated at CircularQueue[{}]",
                element, self.front
            );
            if self.front == self.rear {
                self.front = -1;
                self.rear = -1;
            } else {
                self.front = (self.front + 1) % 10;
            }
        }
    }

    fn display_circular_queue(&self) {
        if self.check_circular_queue_empty() {
            println!("[]");
        } else {
            let mut vector = vec![];
            if self.front < self.rear {
                for i in self.front as usize..=self.rear as usize {
                    vector.push(&self.vector[i]);
                }
            } else {
                for i in self.front as usize..self.capacity {
                    vector.push(&self.vector[i]);
                }

                for i in 0..=self.rear as usize {
                    vector.push(&self.vector[i]);
                }
            }

            println!("Circular Queue => {:?}", vector);
        }
    }
}

fn circular_queue_with_string_data_type() {
    let capacity = 10;
    let vector: Vec<String> = vec![];
    let front = -1;
    let rear = -1;

    // Create a new circular queue
    let mut queue = CircularQueue::<String>::create_circular_queue(capacity, vector, front, rear);
    println!("After initializing Circular queue");
    CircularQueue::<String>::display_circular_queue(&queue);
    println!(
        "Is Circular queue empty? {}",
        CircularQueue::<String>::check_circular_queue_empty(&queue)
    );
    println!(
        "Is Circular queue full? {}\n",
        CircularQueue::<String>::check_circular_queue_full(&queue)
    );

    let data: Vec<String> = vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
        String::from("d"),
        String::from("e"),
        String::from("f"),
        String::from("g"),
        String::from("h"),
        String::from("i"),
        String::from("j"),
        String::from("k"),
        String::from("l"),
        String::from("m"),
        String::from("n"),
        String::from("o"),
    ];

    // Pushing data into circular queue
    for i in 0..capacity {
        CircularQueue::<String>::enqueue(&mut queue, data[i].clone());
    }
    println!("After pushing data");
    CircularQueue::<String>::display_circular_queue(&queue);
    println!(
        "Is Circular queue empty? {}",
        CircularQueue::<String>::check_circular_queue_empty(&queue)
    );
    println!(
        "Is Circular queue full? {}\n",
        CircularQueue::<String>::check_circular_queue_full(&queue)
    );

    // Accessing data from circular queue
    CircularQueue::<String>::access_data(&queue, 6);

    // Popping data into circular queue
    for _i in 1..=5 {
        CircularQueue::<String>::dequeue(&mut queue);
    }
    println!("\nAfter popping data");
    CircularQueue::<String>::display_circular_queue(&queue);
    println!(
        "Is Circular queue empty? {}",
        CircularQueue::<String>::check_circular_queue_empty(&queue)
    );
    println!(
        "Is Circular queue full? {}\n",
        CircularQueue::<String>::check_circular_queue_full(&queue)
    );

    // Pushing data into circular queue again
    for i in 10..data.len() {
        CircularQueue::<String>::enqueue(&mut queue, data[i].clone());
    }
    println!("After pushing data again");
    CircularQueue::<String>::display_circular_queue(&queue);
    println!(
        "Is Circular queue empty? {}",
        CircularQueue::<String>::check_circular_queue_empty(&queue)
    );
    println!(
        "Is Circular queue full? {}\n",
        CircularQueue::<String>::check_circular_queue_full(&queue)
    );
}

fn circular_queue_with_i32_data_type() {
    let capacity = 10;
    let vector: Vec<i32> = vec![];
    let front = -1;
    let rear = -1;

    // Create a new circular queue
    let mut queue = CircularQueue::<i32>::create_circular_queue(capacity, vector, front, rear);
    println!("After initializing Circular queue");
    CircularQueue::<i32>::display_circular_queue(&queue);
    println!(
        "Is Circular queue empty? {}",
        CircularQueue::<i32>::check_circular_queue_empty(&queue)
    );
    println!(
        "Is Circular queue full? {}\n",
        CircularQueue::<i32>::check_circular_queue_full(&queue)
    );

    // Pushing data into circular queue
    for i in 1..=10 {
        CircularQueue::<i32>::enqueue(&mut queue, i);
    }
    println!("After pushing data");
    CircularQueue::<i32>::display_circular_queue(&queue);
    println!(
        "Is Circular queue empty? {}",
        CircularQueue::<i32>::check_circular_queue_empty(&queue)
    );
    println!(
        "Is Circular queue full? {}\n",
        CircularQueue::<i32>::check_circular_queue_full(&queue)
    );

    // Accessing data from circular queue
    CircularQueue::<i32>::access_data(&queue, 6);

    // Popping data into circular queue
    for _i in 1..=5 {
        CircularQueue::<i32>::dequeue(&mut queue);
    }
    println!("\nAfter popping data");
    CircularQueue::<i32>::display_circular_queue(&queue);
    println!(
        "Is Circular queue empty? {}",
        CircularQueue::<i32>::check_circular_queue_empty(&queue)
    );
    println!(
        "Is Circular queue full? {}\n",
        CircularQueue::<i32>::check_circular_queue_full(&queue)
    );

    // Pushing data into circular queue again
    for i in 11..=15 {
        CircularQueue::<i32>::enqueue(&mut queue, i);
    }
    println!("After pushing data again");
    CircularQueue::<i32>::display_circular_queue(&queue);
    println!(
        "Is Circular queue empty? {}",
        CircularQueue::<i32>::check_circular_queue_empty(&queue)
    );
    println!(
        "Is Circular queue full? {}\n",
        CircularQueue::<i32>::check_circular_queue_full(&queue)
    );
}

fn main() {
    println!("-------------------------");
    println!("For `String` data type :-");
    println!("-------------------------\n");
    circular_queue_with_string_data_type();

    println!("\n-------------------------");
    println!("For `i32` data type :-");
    println!("-------------------------\n");
    circular_queue_with_i32_data_type();
}
