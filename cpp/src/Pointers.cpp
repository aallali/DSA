#include <iostream>

int main()
{
    // Declare an integer variable x and initialize it with value 1337
    int x{1337};

    // Declare a pointer variable p and assign the address of x to it
    int *p = &x;

    // Print the address of x
    std::cout << "Address of x: " << &x << "\n";

    // Print the value of x
    std::cout << "Value of x: " << x << "\n";

    // Print the value stored in the pointer p (address of x)
    std::cout << "Value of pointer p (address of x): " << p << "\n";

    // Print the value pointed to by the pointer p (value of x)
    std::cout << "Value pointed to by pointer p (value of x): " << *p << "\n";

    return 0;
}
// OUTPUT:

// Address of x: 0x7ffdcd6be16c
// Value of x: 19
// Value of pointer p (address of x): 0x7ffdcd6be16c
// Value pointed to by pointer p (value of x): 19
