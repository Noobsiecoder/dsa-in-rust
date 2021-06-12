#[derive(Debug)]
struct Queue<T> {
    capacity: usize,
    vector: Vec<T>,
    front: isize,
    rear: isize,
}

impl<T: std::fmt::Debug> Queue<T> {
    fn create_queue(capacity: usize, vector: Vec<T>, front: isize, rear: isize) -> Self {
        Self {
            capacity,
            vector,
            front,
            rear,
        }
    }

    fn check_queue_empty(&self) -> bool {
        if self.front == -1 && self.rear == -1 {
            return true;
        }

        false
    }

    fn check_queue_full(&self) -> bool {
        if self.front == 0 && self.rear == (self.capacity - 1) as isize {
            return true;
        }

        false
    }

    fn access_data(&self, index: usize) {
        if self.front == -1 && self.rear == -1 {
            eprintln!(
                "Cannot access data at index {} since queue is empty\n",
                index
            );
        } else if index >= self.capacity {
            eprintln!(
                "Cannot access data at index {} since total capacity of queue is {}\n",
                index, self.capacity
            );
        } else {
            println!(
                "The data at Queue[{}] is = {:?}\n",
                index, self.vector[index]
            );
        }
    }

    fn enqueue(&mut self, data: T) {
        if self.rear == (self.capacity - 1) as isize {
            eprintln!("Cannot add more data since queue is full\n");
        } else {
            if self.front == -1 {
                self.front = 0;
            }
            self.rear += 1;
            self.vector.push(data);
        }
    }

    fn dequeue(&mut self) {
        if self.front == -1 {
            eprintln!("Cannot remove data since queue is empty\n");
        } else {
            let element = &self.vector[self.front as usize];
            println!(
                "Element removed is {:?} and was situated at Queue[{}]",
                element, self.front
            );
            if self.front >= self.rear {
                self.front = -1;
                self.rear = -1;
            } else {
                self.front += 1;
            }
        }
    }

    fn display_queue(&self) {
        if self.front == -1 && self.rear == -1 {
            eprintln!("[]");
        } else {
            let mut vector = vec![];
            for i in self.front as usize..=self.rear as usize {
                vector.push(&self.vector[i]);
            }
            println!("Queue => {:?}", vector);
        }
    }
}

fn queue_with_string_data_type() {
    let capacity = 10;
    let vector: Vec<String> = vec![];
    let front = -1;
    let rear = -1;

    // Create a new queue
    let mut queue = Queue::<String>::create_queue(capacity, vector, front, rear);
    println!("After initializing queue");
    Queue::<String>::display_queue(&queue);
    println!(
        "Is queue empty? {}",
        Queue::<String>::check_queue_empty(&queue)
    );
    println!(
        "Is queue full? {}\n",
        Queue::<String>::check_queue_full(&queue)
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
    ];

    // Pushing data into queue
    for i in 0..capacity {
        Queue::<String>::enqueue(&mut queue, data[i].clone());
    }
    println!("After pushing data");
    Queue::<String>::display_queue(&queue);
    println!(
        "Is queue empty? {}",
        Queue::<String>::check_queue_empty(&queue)
    );
    println!(
        "Is queue full? {}\n",
        Queue::<String>::check_queue_full(&queue)
    );

    // Accessing data from queue
    Queue::<String>::access_data(&queue, 6);

    // Popping data into queue
    for _i in 0..capacity {
        Queue::<String>::dequeue(&mut queue);
    }
    println!("\nAfter popping data");
    Queue::<String>::display_queue(&queue);
    println!(
        "Is queue empty? {}",
        Queue::<String>::check_queue_empty(&queue)
    );
    println!(
        "Is queue full? {}\n",
        Queue::<String>::check_queue_full(&queue)
    );
}

fn queue_with_i32_data_type() {
    let capacity = 10;
    let vector: Vec<i32> = vec![];
    let front = -1;
    let rear = -1;

    // Create a new queue
    let mut queue = Queue::<i32>::create_queue(capacity, vector, front, rear);
    println!("After initializing queue");
    Queue::<i32>::display_queue(&queue);
    println!(
        "Is queue empty? {}",
        Queue::<i32>::check_queue_empty(&queue)
    );
    println!(
        "Is queue full? {}\n",
        Queue::<i32>::check_queue_full(&queue)
    );

    // Pushing data into queue
    for i in 1..=10 {
        Queue::<i32>::enqueue(&mut queue, i);
    }
    println!("After pushing data");
    Queue::<i32>::display_queue(&queue);
    println!(
        "Is queue empty? {}",
        Queue::<i32>::check_queue_empty(&queue)
    );
    println!(
        "Is queue full? {}\n",
        Queue::<i32>::check_queue_full(&queue)
    );

    // Accessing data from queue
    Queue::<i32>::access_data(&queue, 6);

    // Popping data into queue
    for _i in 1..=5 {
        Queue::<i32>::dequeue(&mut queue);
    }
    println!("\nAfter popping data");
    Queue::<i32>::display_queue(&queue);
    println!(
        "Is queue empty? {}",
        Queue::<i32>::check_queue_empty(&queue)
    );
    println!(
        "Is queue full? {}\n",
        Queue::<i32>::check_queue_full(&queue)
    );
}

fn main() {
    println!("-------------------------");
    println!("For `String` data type :-");
    println!("-------------------------\n");
    queue_with_string_data_type();

    println!("\n-------------------------");
    println!("For `i32` data type :-");
    println!("-------------------------\n");
    queue_with_i32_data_type();
}
