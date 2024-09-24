---
title: "Binary Search Algorithm"
slug: binarysearchalgorithm
image: "/images/tech-future.jpg"
language: "English"
tag: ["Algorithm", "Math", "C/C++"]
date: "2024-09-24"
author: "Skuld Norniern"
---

Binary Search Algorithm is a one of a Search Algorithm and the main point of this algorithm is that dividing the search radius to two part.

and that's why binary search is faster than normal search algorithm that search the whole search radius.
The way you do the Binary Search is to use the left, right, mid. First of all the mid value will be (left + right)/2 and we will compare that value with the value that we are trying to find.
when you are doing the Binary Search Algorithm the array should be sorted
set the mid value using the left, right value
compare the mid value with the value that we are trying to find
after comparing if the mid value is bigger than the value that we are trying to found, then the left value should be mid+1 and if the mid value is smaller, the right value should be mid-1
continue this method while left <= right to get the answer we are looking for
Normal search method's Time Complexity should be O(n) but when we use this method the Time Complexity should be O(log(n)), So should be very fast 
[I will insert the image for  easy understanding]
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

You can also use the lower_bound, upper_bound from the STL libraryyou can use it as lower/upper_bound(arr, arr+n, key);The only downside is that the return type is a form of a Iterator so if you want the value of the key placement, you can - v.begin() or use the array's name 
What you have to know is that the lower_bound is a function that find's the number's placement value of that is bigger or the sameand the upper_bound is a function that find's the number's placement value of that is bigger 