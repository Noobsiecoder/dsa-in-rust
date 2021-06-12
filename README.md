# Data structures and Algorithms in Rust programming language

## What is a data structure?

- It is a way to store and organize data so that we can access and also update the data easily
- There are different types of data structures and they are :-

| Linear data structure | Non linear data structure |
| --------------------- | :------------------------ |
| Array                 | Graphs                    |
| Stack                 | Trees                     |
| Queue                 |                           |
| Linked list           |                           |

### Pseudocode for data structures

1. [Stack](https://github.com/Noobsiecoder/dsa-in-rust/tree/main/data-structure/stack/README.md)
2. [Linear queue](https://github.com/Noobsiecoder/dsa-in-rust/tree/main/data-structure/linear-queue/README.md)
3. [Circular queue](https://github.com/Noobsiecoder/dsa-in-rust/tree/main/data-structure/circular-queue/README.md)
4. [Deque(Double ended queue)](https://github.com/Noobsiecoder/dsa-in-rust/tree/main/data-structure/double-ended-queue/README.md)

## What is an algorithm?

- It is a set of well-defined instructions to solve a particular problem.
- We need algorithms to solve a problem efficiently.
- Consider a case where we need to find the sum of `N` numbers
- We would write a code :-

  ```rs
  fn sum_of_natural_num(num: i128) -> i128 {
      let mut result: i128 = 0;
      for i in 1..=num {
          result += i;
      }

      result // returns sum
  }
  ```

- What if `num` was _100000000000000000_ ?
- Even though we would get the result, the performance won't be good. This is because the for loop will run for _100000000000000000 + 1_ times.
- We can enhance the algorithm and code it as :-

  ```rs
  fn sum_of_natural_num(num: i128) -> i128 {
      num * (num + 1) / 2 // returns sum
  }
  ```
- The above code runs only once and performs well when compared with first case.