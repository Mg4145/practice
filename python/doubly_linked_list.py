""" Doubly Linked List Implementation """


class Node:
    """Node class for doubly linked list"""

    def __init__(self, data) -> None:
        self.data = data
        self.next = None
        self.prev = None

    def __str__(self) -> str:
        return str(self.data)


class DoublyLinkedList:
    """Doubly Linked List Class"""

    def __init__(self) -> None:
        self.head = None
        self.tail = None

    def insert(self, data) -> None:
        """Insert data into linked list"""
        tmp = Node(data)
        if self.head is None:
            # If head is None, set head to tmp
            self.head = tmp
            self.tail = tmp
        else:
            # Else, traverse to end of list and set next to tmp
            current = self.head
            while current.next is not None:
                current = current.next

            current.next = Node(data)
            current.next.prev = current
            self.tail = current.next

    def print_list(self) -> None:
        """Print linked list"""
        current = self.head
        while current is not None:
            print(f"{current.data} <-> ", end="")
            current = current.next
        if current is None:
            print("None")


def main():
    """Main function"""
    doubly_linked_list = DoublyLinkedList()
    doubly_linked_list.print_list()
    print("Adding 1 to 10")
    for i in range(1, 11):
        doubly_linked_list.insert(i)
    doubly_linked_list.print_list()


if __name__ == "__main__":
    main()
