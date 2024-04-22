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
    // Integer array declaration and initialization
    int intArr[] = {1, 2, 3}; // Array of integers

    // String array declaration and initialization
    std::string stringArr[] = {"Hello", "World", "!"}; // Array of strings initialized with 3 values 

    // Boolean array declaration and initialization
    bool boolArr[] = {true, false, true}; // Array of booleans initialized inline

    // Floating-point array declaration and initialization
    float floatArr[] = {1.5f, 2.5f, 3.5f}; // Array of floats 

    // Double array declaration and initialization
    double doubleArr[] = {1.234, 5.678, 9.012}; // Array of doubles 

    // Character array declaration and initialization
    char charArr[] = {'a', 'b', 'c'}; // Array of characters 

    // Vector declaration and initialization
    std::vector<int> intVector = {1, 2, 3}; // Vector of integers 

    return;
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

// OUTPUT ====>

// --------------------------- static array ---------------------------

// Index [0] 11
// Index [1] 22
// Index [2] 33
// 0x7ffd64a48d14

// Index [0] 1337
// Index [1] 10101010
// Index [2] 1973

// --------------------------- dynamic array ---------------------------

// 0x5bcf2ecf92c0
// Index [0] 1100
// Index [1] 2200
// Index [2] 3300

// --------------------------- overview ---------------------------

// Size of staticArr: 5
// Size of dynamicArr: 5
// Size of initializedArr: 5
// Size of matrix: 3x3
// Size of pointerArr: 5
// Size of intVector: 5

// --------------------------- debug addresses ---------------------------

// Address of stackVar: 0x7ffd64a48ce4
// Address of heapVar: 0x5bcf2ecf92e0