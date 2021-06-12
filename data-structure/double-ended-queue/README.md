# Deque data structure

## Table of contents
- [Deque data structure](#deque-data-structure)
  - [Table of contents](#table-of-contents)
  - [Introduction](#introduction)
  - [Pseudocode](#pseudocode)
    - [Default](#default)
    - [Insert from front](#insert-from-front)
    - [Delete from front](#delete-from-front)
    - [Insert from rear](#insert-from-rear)
    - [Delete from rear](#delete-from-rear)
    - [IsEmpty](#isempty)
    - [IsFull](#isfull)
  - [Resource](#resource)

## Introduction

- Insertion and deletion of elements can either be performed from the front or the rear.
- Thus, it does not follow **FIFO(First in, First out)**
- Types of Deque

  - Input Restricted Deque
    - Input is restricted at a single end but allows deletion at both the ends.
  - Output Restricted Deque
    - Output is restricted at a single end but allows insertion at both the ends.

  <img src="https://cdn.programiz.com/sites/tutorial2program/files/deque.png" alt="deque-example" width="400"/>

## Pseudocode

- Representation :-
  <img src="https://www.happycoders.eu/wp-content/uploads/2020/04/Datastructure_Deque.png" alt="deque-working-example" width="400"/>

### Default

```
initialize SIZE for the array/vector in rust
initialize FRONT = -1 and REAR = -1
initialize DEQUE an empty array/vector in rust
```

### Insert from front

```
if FRONT == 0
    print deque is full
else if FRONT == -1
    print deque yet to be filled
else
    decrement FRONT by 1
    set DEQUE[FRONT] = VALUE
```

### Delete from front

```
if FRONT == -1
    print deque is empty
else
    store DEQUE[FRONT] in DELETED_DATA
    delete DEQUE[FRONT]
    if FRONT == REAR
        FRONT = REAR = -1
    else
        increment FRONT by 1
```

### Insert from rear

```
if REAR == SIZE - 1
    print deque is full
else
    if FRONT == -1
        set FRONT = 0
    increment REAR by 1
    set DEQUE[REAR] = VALUE
```

### Delete from rear

```
if REAR == -1 
    print deque is empty
else
    store DEQUE[REAR] in DELETED_DATA
    delete DEQUE[REAR]
    if FRONT == REAR
        FRONT = REAR = -1
    else
        decrement REAR by 1
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
if FRONT == 0 and REAR == SIZE - 1
    return true
else
    return false
```

## Resource

[Programmiz](https://www.programiz.com/)
