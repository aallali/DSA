#include <stdio.h>
#include <iostream>
// #include <vector>
#include <algorithm>


int main() {
	std::vector<int> v{1,2,3,4,5,6};

	std::for_each(v.begin(), v.end(), [](int x){
		if (x % 2 == 0) {
			std::cout << x << " is even number\n";
		} else {
			std::cout << x << " is odd number\n";
		}
	});

	return 0;
}
