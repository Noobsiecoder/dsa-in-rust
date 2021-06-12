# Linear queue data structure

## Table of contents
- [Linear queue data structure](#linear-queue-data-structure)
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

  <img src="https://cdn.programiz.com/sites/tutorial2program/files/queue.png" alt="queue-example" width="400"/>

## Pseudocode

- Representation :-
  <img src="https://cdn.programiz.com/sites/tutorial2program/files/Queue-program-enqueue-dequeue.png" alt="queue-working-example" width="400"/>

### Default

```
initialize SIZE for the array/vector in rust
initialize FRONT = -1 and REAR = -1
initialize QUEUE an empty array/vector in rust
```

### Enqueue

```
if REAR == SIZE - 1
    print queue is full
else
    insert VALUE into QUEUE
    if FRONT == -1
        FRONT = 0
    increment REAR by 1
```

### Dequeue

```
if FRONT == - 1
    print queue is empty
else
    store QUEUE[FRONT] in DELETED_DATA
    delete QUEUE[FRONT]
    if FRONT >= REAR
        FRONT = REAR = -1
    else
        increment FRONT by 1
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
if REAR == SIZE - 1
    return true
else
    return false
```

### Peek

```
get INDEX
if INDEX <= REAR
    print QUEUE[INDEX]
```

## Resource

[Programmiz](https://www.programiz.com/)
