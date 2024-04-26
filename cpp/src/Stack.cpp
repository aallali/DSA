#include <iostream>
#include <stack>

void printStackElems(std::stack<int> stk)
{
    int i = 1;
    const int stackSize = stk.size();
    while (!stk.empty())
    {
        std::cout << "Elem [" << (stackSize - i) << "]: " << stk.top() << "\n";
        stk.pop();
        i++;
    }
    return;
}
int main()
{
    std::stack<int> nbrStack;

    nbrStack.push(0);
    nbrStack.push(1);
    nbrStack.push(2);
    nbrStack.pop();
    nbrStack.push(3);
    nbrStack.push(4);
    nbrStack.push(5);

    printStackElems(nbrStack);
    std::cout << "Size of stack [" << nbrStack.size() << "]\n";
    return 0;
}

// LIFO : Last In First Out
// FILO : First In Last Out
// Stack funs:
//      - empty : tells if ur stack is empty or not.
//      - size  : number of elems in stack.
//      - top   : returns top X elem in the stack.
//      - push  : add element to stack.
//      - pop   : remove element from stack.


// Output:
/*

Elem [4]: 5
Elem [3]: 4
Elem [2]: 3
Elem [1]: 1
Elem [0]: 0
Size of stack [5]

*/