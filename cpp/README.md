
# Learning C++20 journey

## Description
this is my note-taking through my journey to learn c++20 following this roadmap https://roadmap.sh/cpp

---
### lambda expression
<details>

<summary>Click Me </summary>

 
this expression:
```c++
struct
	{
	 	void operator()(int x) {
	 		std::cout << x << "\n";
	    }
    } something;
```
can be translated to:
```c++
[](int x){
    std::cout << x << "\n";
}
```
example code:
```c++
#include <stdio.h>
#include <iostream>
#include <algorithm>


int main() {
	std::vector<int> v{1, 2, 3, 4, 5, 6};

	std::for_each(v.begin(), v.end(), [](int x){
        if (x % 2 == 0) {
			std::cout << x << " is even number\n";
		} else {
			std::cout << x << " is odd number\n";
		}
	});
	return 0;
}

// output
// 1 is odd number
// 2 is even number
// 3 is odd number
// 4 is even number
// 5 is odd number
// 6 is even number
```
in case u wanna acess a variable declared in the upper scope u have to pass it here:
```c++
// [cc](p){fd}
// [capture clause](perimeters){function definition}
int d = 3;
[d](...){...}
// if you wanna access and modify the variable
[&d](...){...}
// if you wanna access all upper scope variables without changing them
[=](...){...}
```
</details>

### Data types

<details>

<summary>Click Me </summary>

Oga Boga
</details>


### Array vs Vector
<details>

<summary>Click Me </summary>

`std::vector<int>` and `int intArr[]` represent two different ways of managing arrays in C++:

1. `std::vector<int>`:
   - `std::vector` is a dynamic array container provided by the C++ Standard Library.
   - It allows for dynamic resizing, meaning elements can be added or removed without worrying about managing memory explicitly.
   - It provides functions for efficient manipulation, such as `push_back()`, `pop_back()`, and random access using `[]` or `.at()`.
   - It automatically manages memory allocation and deallocation.
   - It provides safety checks like bounds checking when accessing elements.

2. `int intArr[]` (Static Array):
   - Static arrays are fixed-size collections of elements.
   - The size of the array must be known at compile time and cannot be changed during runtime.
   - Memory for static arrays is allocated on the stack (if declared within a function) or in the global/static memory (if declared outside a function).
   - You have to manage memory manually, including memory allocation and deallocation.
   - Static arrays lack many of the dynamic capabilities and safety checks provided by `std::vector`.

In summary, `std::vector<int>` is more flexible and provides dynamic resizing and memory management capabilities, while `int intArr[]` is a fixed-size array with manual memory management and no dynamic resizing capabilities.
</details>
