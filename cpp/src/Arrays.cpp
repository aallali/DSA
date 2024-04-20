#include <iostream>

template<typename T, size_t N>

size_t ft_arr_length(T (&arr)[N]) {
    return sizeof(arr) / sizeof(arr[0]);
}


int main()
{
    // Static declaration with a fixed size
    int arr[3] = {1, 2, 3};

    int size = ft_arr_length(arr);

    for (int i = 0; i < size; i++)
    {
        std::cout << arr[i] << "\n";
    }

    return 0;
}
