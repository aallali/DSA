#include <iostream>

struct Node
{
    int data;
    Node *next;
    void print()
    {
        std::cout << "--: " << data << "\n";
    }
};

void printLinkedList(Node *node)
{
    while (node)
    {
        // std::cout << node->data << "\n";
        node->print();
        node = node->next;
    }
}

void insertAtTheFront(Node **node, int data)
{
    // 1- create new node
    Node *freshNode = new Node();
    // 2- set new node in front of current node
    freshNode->data = data;
    freshNode->next = *node;
    // 3- set new node as the head of the linked list
    *node = freshNode;
}

void insertAtTheEnd(Node **head, int data)
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

void insertAfter(Node **prevNode, int data)
{
    // 1- check if prevNode is NULL
    if (*prevNode == NULL) {
        std::cout << "Previous node cannot be NULL";
        return;
    }
    // 2- Prepare a newNode
    Node *newNode = new Node();
    newNode->data = data;

    // // 3- Insert newNode after prevNode
    newNode->next = (*prevNode)->next;
    (*prevNode)->next = newNode;
}

int main()
{
    Node *head = new Node();
    head->data = 1;

    insertAfter(&head, 2);
    Node *second = head->next;

    Node *third = new Node();
    third->data = 3;
    second->next = third;

    printLinkedList(head);
    std::cout << "------------------------\n";
    insertAtTheFront(&head, 0);
    insertAtTheEnd(&head, 4);
    printLinkedList(head);

    std::cout << "------------------------\n";
    std::cout << &head->next << std::endl;
    std::cout << head->next << std::endl;
    return 0;
}

// OUTPUT:

/**
--: 1
--: 2
--: 3
------------------------
--: 0
--: 1
--: 2
--: 3
--: 4
------------------------
0x5ff3c1096328
0x5ff3c1095eb0
*/
