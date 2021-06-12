#[derive(Debug)]
struct Deque<T> {
    capacity: usize,
    vector: Vec<T>,
    front: isize,
    rear: isize,
}

impl<T: std::fmt::Debug> Deque<T> {
    fn create_deque(capacity: usize, vector: Vec<T>, front: isize, rear: isize) -> Self {
        Self {
            capacity,
            vector,
            front,
            rear,
        }
    }

    fn check_deque_full(&self) -> bool {
        if self.front == 0 && self.rear == (self.capacity - 1) as isize {
            return true;
        }

        false
    }

    fn check_deque_empty(&self) -> bool {
        if self.front == -1 && self.rear == -1 {
            return true;
        }

        false
    }

    fn insert_front(&mut self, data: T) {
        if self.front == 0 {
            eprintln!("Cannot add data at front since deque is full from front side\n");
        } else if self.front < 0 {
            eprintln!("Cannot add data at front since deque is yet to be filled from front side\n");
        } else {
            self.front -= 1;
            self.vector[self.front as usize] = data;
        }
    }

    fn insert_rear(&mut self, data: T) {
        if self.rear == (self.capacity - 1) as isize {
            eprintln!("Cannot add data at rear since deque is full from rear side\n");
        } else {
            if self.front == -1 {
                self.front = 0;
            }
            self.rear += 1;
            self.vector.push(data);
        }
    }

    fn delete_front(&mut self) {
        if self.front == -1 {
            eprintln!("Cannot remove data since deque is empty\n");
        } else {
            let element = &self.vector[self.front as usize];
            println!(
                "Element removed is {:?} and was situated at Deque[{}]",
                element, self.front
            );
            if self.front == self.rear {
                self.front = -1;
                self.rear = -1;
            } else {
                self.front += 1;
            }
        }
    }

    fn delete_rear(&mut self) {
        if self.rear == -1 {
            eprintln!("Cannot remove data since deque is empty\n");
        } else {
            let element = self.vector.pop().unwrap();
            println!(
                "Element removed is {:?} and was situated at Deque[{}]",
                element, self.rear
            );
            if self.front == self.rear {
                self.front = -1;
                self.rear = -1;
            } else {
                self.rear -= 1;
            }
        }
    }

    fn display_deque(&self) {
        if self.check_deque_empty() {
            println!("[]");
        } else {
            let mut deque_vector = vec![];
            for i in self.front as usize..=self.rear as usize {
                deque_vector.push(&self.vector[i]);
            }

            println!("{:?}", deque_vector);
        }

        println!("Is Deque empty? {}", self.check_deque_empty());
        println!("Is Deque full? {}\n", self.check_deque_full());
    }
}

fn deque_with_string_data_type() {
    let capacity = 10;
    let vector: Vec<String> = vec![];
    let front = -1;
    let rear = -1;

    // Creating an empty deque
    let mut deque = Deque::<String>::create_deque(capacity, vector, front, rear);
    println!("After initializing Deque");
    Deque::<String>::display_deque(&deque);

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

    // Inserting values into the empty deque from rear side
    for i in 0..capacity {
        Deque::<String>::insert_rear(&mut deque, data[i].clone());
    }

    println!("After pushing data from rear side");
    Deque::<String>::display_deque(&deque);

    // Deleting values from the front end of the deque
    for _i in 0..5 {
        Deque::<String>::delete_front(&mut deque);
    }

    println!("\nAfter popping data from front side");
    Deque::<String>::display_deque(&deque);

    // Inserting values into the empty deque from front side
    for i in 10..data.len() {
        Deque::<String>::insert_front(&mut deque, data[i].clone());
    }

    println!("\nAfter pushing data from front side");
    Deque::<String>::display_deque(&deque);

    // Deleting values from the rear end of the deque
    for _i in 0..5 {
        Deque::<String>::delete_rear(&mut deque);
    }

    println!("\nAfter popping data from rear side");
    Deque::<String>::display_deque(&deque);
}

fn deque_with_i32_data_type() {
    let capacity = 10;
    let vector: Vec<i32> = vec![];
    let front = -1;
    let rear = -1;

    // Creating an empty deque
    let mut deque = Deque::<i32>::create_deque(capacity, vector, front, rear);
    println!("After initializing Deque");
    Deque::<i32>::display_deque(&deque);

    // Inserting values into the empty deque from rear side
    for i in 1..=10 {
        Deque::<i32>::insert_rear(&mut deque, i);
    }

    println!("After pushing data from rear side");
    Deque::<i32>::display_deque(&deque);

    // Deleting values from the front end of the deque
    for _i in 0..5 {
        Deque::<i32>::delete_front(&mut deque);
    }

    println!("\nAfter popping data from front side");
    Deque::<i32>::display_deque(&deque);

    // Inserting values into the empty deque from front side
    for i in 11..=15 {
        Deque::<i32>::insert_front(&mut deque, i);
    }

    println!("After pushing data from front side");
    Deque::<i32>::display_deque(&deque);

    // Deleting values from the rear end of the deque
    for _i in 0..5 {
        Deque::<i32>::delete_rear(&mut deque);
    }

    println!("\nAfter popping data from rear side");
    Deque::<i32>::display_deque(&deque);
}

fn main() {
    println!("-------------------------");
    println!("For `String` data type :-");
    println!("-------------------------\n");
    deque_with_string_data_type();

    println!("\n-------------------------");
    println!("For `i32` data type :-");
    println!("-------------------------\n");
    deque_with_i32_data_type();
}
