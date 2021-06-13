# Stack data structure

## Table of contents
- [Stack data structure](#stack-data-structure)
  - [Table of contents](#table-of-contents)
  - [Introduction](#introduction)
  - [Pseudocode](#pseudocode)
    - [Default](#default)
    - [Push](#push)
    - [Pop](#pop)
    - [IsEmpty](#isempty)
    - [IsFull](#isfull)
    - [Peek](#peek)
  - [Resource](#resource)

## Introduction

- Data structure that follows **LIFO(Last in, First out)**
- This means that the last element is inserted first and then the first element is deleted first.

  <img src="https://cdn.programiz.com/sites/tutorial2program/files/stack.png" alt="stack-example" width="400"/>

## Pseudocode

<img src="https://cdn.programiz.com/sites/tutorial2program/files/stack-operations.png" alt="stack-working-example" width="400"/>

### Default

```
initialize SIZE for the array/vector in rust
initialize TOP = -1
initialize STACK an empty array/vector in rust
```

### Push

```
if TOP == SIZE - 1
    Print Stack is full
else
    insert VALUE into STACK
    increment TOP by 1
```

### Pop

```
if TOP == - 1
    Print Stack is empty
else
    store STACK[TOP] in DELETED_DATA
    delete STACK[TOP]
    decrement TOP by 1
```

### IsEmpty

```
if TOP == - 1
    return true
else
    return false
```

### IsFull

```
if TOP == SIZE - 1
    return true
else
    return false
```

### Peek

```
get INDEX
if INDEX <= TOP
    print STACK[INDEX]
```

## Resource

[Programmiz](https://www.programiz.com/)
