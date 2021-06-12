#[derive(Debug)]
struct Stack<T> {
    capacity: usize,
    arr: Vec<T>,
    top: i8,
}

impl<T: std::fmt::Debug> Stack<T> {
    fn create_stack(capacity: usize, arr: Vec<T>, top: i8) -> Self {
        Self { capacity, arr, top }
    }

    fn check_stack_empty(top: &i8) -> bool {
        match *top {
            -1 => true,
            _ => false,
        }
    }

    fn check_stack_full(top: &i8, capacity: &usize) -> bool {
        if *top == (*capacity - 1) as i8 {
            return true;
        }

        false
    }

    fn access_data(&self, index: usize) {
        if self.top == -1 {
            eprintln!(
                "Cannot access data at index {} since stack is empty\n",
                index
            );
        } else if index > self.capacity {
            eprintln!(
                "Cannot access data at index {} since total capacity of stack is {}\n",
                index, self.capacity
            );
        } else {
            println!("The data at stack[{}] is = {:?}\n", index, self.arr[index]);
        }
    }

    fn push_data(&mut self, data: T) {
        if self.top == (self.capacity - 1) as i8 {
            eprintln!("Cannot add more data since stack is full\n");
        } else {
            &mut self.arr.push(data);
            self.top += 1;
        }
    }

    fn pop_data(&mut self) {
        if self.top == -1 {
            eprintln!("Cannot remove any data since stack is empty\n");
        } else {
            &mut self.arr.pop();
            self.top -= 1;
        }
    }

    fn display_data(&self, info: String) {
        println!("{}", info);
        println!("{:?}", self.arr);
        println!(
            "Is stack empty? {}",
            Stack::<T>::check_stack_empty(&self.top)
        );
        println!(
            "Is stack full? {}\n",
            Stack::<T>::check_stack_full(&self.top, &self.capacity)
        );
    }
}

fn stack_with_string_data_type() {
    let capacity = 10;
    let vector: Vec<String> = vec![];
    let top = -1;

    // Create a new stack
    let mut stack = Stack::<String>::create_stack(capacity, vector, top);
    Stack::<String>::display_data(
        &stack,
        String::from("Before pushing and popping data to stack"),
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

    // Pushing data into the stack created
    for i in 0..capacity {
        Stack::<String>::push_data(&mut stack, data[i].clone());
    }
    Stack::<String>::display_data(&stack, String::from("After pushing data to stack"));

    let index: usize = 4;
    Stack::<String>::access_data(&mut stack, index);

    // Popping data into the stack created
    for _i in 0..capacity {
        Stack::<String>::pop_data(&mut stack);
    }
    Stack::<String>::display_data(&stack, String::from("After popping data to stack"));
}

fn stack_with_i32_data_type() {
    let capacity = 10;
    let vector: Vec<i32> = vec![];
    let top = -1;

    // Create a new stack
    let mut stack = Stack::<i32>::create_stack(capacity, vector, top);
    Stack::<i32>::display_data(
        &stack,
        String::from("Before pushing and popping data to stack"),
    );

    // Pushing data into the stack created
    for i in 1..=10 {
        Stack::<i32>::push_data(&mut stack, i);
    }
    Stack::<i32>::display_data(&stack, String::from("After pushing data to stack"));

    let index: usize = 4;
    Stack::<i32>::access_data(&mut stack, index);

    // Popping data into the stack created
    for _i in 0..=9 {
        Stack::<i32>::pop_data(&mut stack);
    }
    Stack::<i32>::display_data(&stack, String::from("After popping data to stack"));
}

fn main() {
    println!("-------------------------");
    println!("For `String` data type :-");
    println!("-------------------------\n");
    stack_with_string_data_type();
    
    println!("\n-------------------------");
    println!("For `i32` data type :-");
    println!("-------------------------\n");
    stack_with_i32_data_type();
}
