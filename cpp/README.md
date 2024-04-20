
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
