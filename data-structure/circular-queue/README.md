# Circular queue data structure

## Table of contents
- [Circular queue data structure](#circular-queue-data-structure)
  - [Table of contents](#table-of-contents)
  - [Introduction](#introduction)
  - [Pseudocode](#pseudocode)
    - [Default](#default)
    - [Enqueue](#enqueue)
    - [Dequeue](#dequeue)
    - [IsEmpty](#isempty)
    - [IsFull](#isfull)
    - [Peek](#peek)
  - [Resource](#resource)

## Introduction

- Data structure that follows **FIFO(First in, First out)**
- This means that the first element is inserted first and then the first element is also deleted first.
- In a normal queue after insertion and few deletion, there will be non usable spaces in the queue. Circular queue solves this problem.

  <img src="https://cdn.programiz.com/sites/tutorial2program/files/circular-increment.png" alt="circ-queue-example" width="400"/>

## Pseudocode

- Representation :-
  <img src="https://cdn.programiz.com/sites/tutorial2program/files/circular-queue-program.png" alt="circ-queue-working-example" width="400"/>

### Default

```
initialize SIZE for the array/vector in rust
initialize FRONT = -1 and REAR = -1
initialize CIRCULAR_QUEUE an empty array/vector in rust
```

### Enqueue

```
if (FRONT == 0 and REAR == SIZE - 1) or FRONT == REAR + 1
    print circular queue is full
else
    if FRONT == -1
        FRONT = 0
    set REAR = (REAR + 1) % 10
    if CIRCULAR_QUEUE.size() < SIZE
        push VALUE into QUEUE
    else
        set CIRCULAR_QUEUE[REAR] = VALUE
```

### Dequeue

```
if FRONT == - 1 and REAR == -1
    print circular queue is empty
else
    store CIRCULAR_QUEUE[FRONT] in DELETED_DATA
    delete CIRCULAR_QUEUE[FRONT]
    if FRONT == REAR
        FRONT = REAR = -1
    else
        set FRONT = (FRONT + 1) % 10
```

### IsEmpty

```
if FRONT == - 1 and REAR == -1
    return true
else
    return false
```

### IsFull

```
if (FRONT == 0 and REAR == SIZE - 1) or FRONT == REAR + 1
    return true
else
    return false
```

### Peek

```
if NOT FRONT == - 1 and REAR == -1 and NOT INDEX > SIZE
    print CIRCULAR_QUEUE[INDEX]
```

## Resource

[Programmiz](https://www.programiz.com/)
