---
title: "Euclidean Algorithm"
slug: euclideanalgorithm
image: "/images/tech-future.jpg"
language: "English"
tag: ["Algorithm", "Math", "C/C++", "Python", "Rust"]
date: "2024-01-16"
author: "Skuld Norniern"
---

First of all, What is a Euclidean Algorithm?

Euclidean Algorithm is a algorithm that gets the greatest common divisor

This algorithm usually made with a simple praise x % y=r and y % r 's greatest common divisor is same with x % y

This is the core mechanism of the Euclidean Algorithm

I'll show you guys the simplest way of coding the Euclidean Algorithm

- C/C++
```c++
#include<stdio.h>
int main()
{
    int i,x,y,r,temp;
    scanf("%d %d",&x,&y);
    while(1)
    {
        r=x%y;
        if(!r)break;
        else x=y,y=r;
    }
    printf("GCD: %d",y);
    return 0;
}
```

- Python
```python
x=int(input())
y=int(input())
while 1:
    r=x%y
    if r==0:break
    else :
        x=y
        y=r
print(y)
```

- Rust
```rust
use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter two numbers (separated by space):");
    io::stdin().read_line(&mut input).unwrap();

    let numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

    if numbers.len() != 2 {
        println!("Please enter exactly two numbers.");
        return;
    }

    let mut x = numbers[0];
    let mut y = numbers[1];
    let mut r;

    while {
        r = x % y;
        if r != 0 {
            x = y;
            y = r;
        }
        r != 0
    } {}

    println!("GCD is: {}", y);
}

```
