---
title: "Binary Search Algorithm"
slug: binarysearchalgorithm
image: "/images/tech-future.jpg"
language: "English"
tag: ["Algorithm", "Math", "C/C++"]
date: "2024-09-24"
author: "Skuld Norniern"
---

## Introduction

The **Binary Search Algorithm** is a fundamental search technique used in computer science. It efficiently finds the position of a target value within a **sorted** array by repeatedly dividing the search interval in half. This method is significantly faster than a linear search, especially for large datasets.

## Why Binary Search?

Unlike a linear search that checks each element one by one with a time complexity of \(O(n)\), binary search reduces the search time to \(O(\log n)\). This efficiency is achieved by eliminating half of the remaining elements in each step of the search process.

## How Binary Search Works

To perform a binary search, follow these steps:

1. **Ensure the array is sorted.** Binary search only works on sorted arrays.
2. **Initialize pointers:**
   - `left` pointing to the start of the array.
   - `right` pointing to the end of the array.
3. **Calculate the middle index:**
   - `mid = (left + right) / 2`
4. **Compare the middle element with the target value:**
   - If `arr[mid]` is equal to the target, the search is successful.
   - If `arr[mid]` is greater than the target, adjust the `right` pointer to `mid - 1`.
   - If `arr[mid]` is less than the target, adjust the `left` pointer to `mid + 1`.
5. **Repeat the process** until the target is found or the `left` pointer exceeds the `right` pointer.

## C++ Implementation

Below is a C++ implementation of the binary search algorithm:

```cpp
#include<stdio.h>
int arr[10]={1, 2, 5, 6, 8, 9, 10, 16, 19, 24};
int main(){
 
    int n, res=0, left=0, right=9, mid;
    scanf("%d",&n); 
    while(left<=right)
    {
        mid=(left+right)/2;
        if(arr[mid]>n) right=mid-1;
        else if(arr[mid]<n)left=mid+1;
        else {res=mid;break;}
    }
    printf("%d ",res);
    return 0;
}
```

## STL Alternatives: `lower_bound` and `upper_bound`

The C++ Standard Template Library (STL) provides efficient alternatives to manual binary search implementation through the `lower_bound` and `upper_bound` functions.

### Usage

You can use these functions as follows:

```cpp
auto it = lower_bound(arr, arr + n, key);
// or
auto it = upper_bound(arr, arr + n, key);
```

### Return Value and Index Calculation

These functions return an iterator. To get the index of the found element:

- For vectors: `it - v.begin()`
- For arrays: `it - arr`

### Function Descriptions

1. **`lower_bound`**: 
   - Finds the position of the first element that is not less than the given key.
   - In other words, it returns the first position where the key could be inserted while maintaining the sorted order.

2. **`upper_bound`**: 
   - Finds the position of the first element that is greater than the given key.
   - It returns the position after the last occurrence of the key in the sorted sequence.

### Example

```cpp
#include <algorithm>
#include <iostream>
#include <vector>

int main() {
    std::vector<int> v = {1, 2, 4, 4, 5, 6, 7};
    int key = 4;

    auto lower = std::lower_bound(v.begin(), v.end(), key);
    auto upper = std::upper_bound(v.begin(), v.end(), key);

    printf("Lower bound of %d is at index: %d\n", key, (lower - v.begin()));
    printf("Upper bound of %d is at index: %d\n", key, (upper - v.begin()));

    return 0;
}
```

This example demonstrates how `lower_bound` finds the first position of 4, while `upper_bound` finds the position after the last occurrence of 4.

### Advantages

- Concise and readable code
- Optimized performance
- Works with any container that supports random access iterators

By understanding and utilizing `lower_bound` and `upper_bound`, you can efficiently perform binary search operations in your C++ programs without manually implementing the algorithm.
