#include <iostream>
#include <string>
#include <vector>

template <typename T, size_t N>

size_t ft_arr_length(T (&arr)[N])
{
    return sizeof(arr) / sizeof(arr[0]);
}

void line(std::string title)
{
    std::cout << "\n--------------------------- " << title << " ---------------------------\n\n";
}

void all_types_lists()
{
    // Integer array declaration
    int intArr[5]; // Array of integers with size 5

    // String array declaration
    std::string stringArr[3]; // Array of strings with size 3

    // Boolean array declaration
    bool boolArr[4]; // Array of booleans with size 4

    // Floating-point array declaration
    float floatArr[6]; // Array of floats with size 6

    // Double array declaration
    double doubleArr[7]; // Array of doubles with size 7

    // Character array declaration
    char charArr[8]; // Array of characters with size 8

    // Vector declaration (dynamic array)
    std::vector<int> intVector(5); // Vector of integers with initial size 5

    return 0;
}

void overview()
{
    // Static array declaration (allocated on the stack)
    int staticArr[5]; // Array of integers with size 5

    // Dynamic array declaration (allocated on the heap)
    int size = 5;
    int *dynamicArr = new int[size]; // Array of integers with size specified by size variable

    // Initialization with values
    int initializedArr[] = {1, 2, 3, 4, 5}; // Array of integers initialized with values

    // Multidimensional array declaration (allocated on the stack)
    int matrix[3][3]; // 3x3 matrix

    // Array of pointers declaration (allocated on the stack)
    int *pointerArr[5]; // Array of integer pointers

    // Vector declaration (dynamic array allocated on the heap)
    std::vector<int> intVector(5); // Vector of integers with initial size 5

    // Output sizes for demonstration
    std::cout << "Size of staticArr: " << sizeof(staticArr) / sizeof(staticArr[0]) << std::endl;                                           // Stack
    std::cout << "Size of dynamicArr: " << size << std::endl;                                                                              // Heap
    std::cout << "Size of initializedArr: " << sizeof(initializedArr) / sizeof(initializedArr[0]) << std::endl;                            // Stack
    std::cout << "Size of matrix: " << sizeof(matrix) / sizeof(matrix[0]) << "x" << sizeof(matrix[0]) / sizeof(matrix[0][0]) << std::endl; // Stack
    std::cout << "Size of pointerArr: " << sizeof(pointerArr) / sizeof(pointerArr[0]) << std::endl;                                        // Stack
    std::cout << "Size of intVector: " << intVector.size() << std::endl;                                                                   // Heap

    // Deallocate memory for dynamic array
    delete[] dynamicArr;
}
int main()
{
    line("static array");

    // Static declaration with a fixed size
    // declared in the hype
    int arr[3] = {11, 22, 33};

    // int size = sizeof(arr) / sizeof(arr[0]);
    int size = ft_arr_length(arr);
    int *ptr = arr;

    for (int i = 0; i < size; i++)
    {
        std::cout << "Index [" << i << "] " << arr[i] << "\n";
    }
    arr[0] = 1337;
    arr[1] = 42;
    arr[2] = 1973;

    *(ptr + 1) = 10101010;
    std::cout << arr << std::endl;

    std::cout << "\n";
    for (int i = 0; i < (int)ft_arr_length(arr); i++)
    {
        std::cout << "Index [" << i << "] " << arr[i] << "\n";
    }

    line("dynamic array");

    int *dyncArr = new int[2];

    dyncArr[0] = 1100;
    dyncArr[1] = 2200;
    dyncArr[2] = 3300;
    std::cout << dyncArr << std::endl;
    for (int i = 0; i < 3; i++)
    {
        std::cout << "Index [" << i << "] " << dyncArr[i] << "\n";
    }

    delete[] dyncArr;

    line("overview");
    overview();

    line("debug addresses");

    int stackVar = 5;           // Allocated on the stack
    int *heapVar = new int(10); // Allocated on the heap

    std::cout << "Address of stackVar: " << &stackVar << std::endl;
    std::cout << "Address of heapVar: " << heapVar << std::endl;

    delete heapVar; // Deallocate heapVar
    return 0;
}
