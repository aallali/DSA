#include <iostream>

struct Node
{
    int data;
    Node *next;
};

void printLinkedList(Node *node)
{
    while (node)
    {
        std::cout << node->data << "\n";
        node = node->next;
    }
}

void pre_append(Node **node, int data)
{
    // 1- create new node
    Node *freshNode = new Node();
    // 2- set new node in front of current node
    freshNode->data = data;
    freshNode->next = *node;
    // 3- set new node as the head of the linked list
    *node = freshNode;
}

void append(Node **head, int data)
{
    // 1-  prepare fresh node
    Node *freshNode = new Node();
    freshNode->data = data;
    freshNode->next = NULL;

    // 2- if the linked list is empty (a.k.a head is null)
    // then set the fresh node as the first node in list
    if (*head == NULL)
    {
        *head = freshNode;
    }

    // 3- traverse to last node in linked list
    Node *last = *head;

    while (last->next != NULL)
    {
        last = last->next;
    }

    // 4- set the fresh node at the tail of list;
    last->next = freshNode;
}

int main()
{
    Node *head = new Node();
    head->data = 1;

    Node *second = new Node();
    second->data = 2;
    head->next = second;

    Node *third = new Node();
    third->data = 3;
    second->next = third;

    printLinkedList(head);
    std::cout << "------------------------\n";
    pre_append(&head, 0);
    append(&head, 4);
    printLinkedList(head);

    std::cout << "------------------------\n";
    std::cout << &head->next << std::endl;
    std::cout << head->next << std::endl;
    return 0;
}

// OUTPUT:

/**
1
2
3
------------------------
0
1
2
3
4
------------------------
0x5a355939a328
0x5a3559399eb0
*/
