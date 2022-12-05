""" Linked list class implementation. """


class Node:
    """Node class for linked list"""

    def __init__(self, data) -> None:
        """Constructor for Node class"""
        self.data = data
        self.next = None

    def __repr__(self) -> str:
        """Representation of Node class"""
        return f"{self.data}"


class LinkedList:
    """Linked list class"""

    def __init__(self) -> None:
        """Constructor for LinkedList class"""
        self.head = None

    def insert(self, data) -> None:
        """Insert data into linked list"""
        tmp = Node(data)
        if self.head is None:
            # If head is None, set head to tmp
            self.head = tmp
        else:
            # Else, traverse to end of list and set next to tmp
            current = self.head
            while current.next is not None:
                current = current.next

            current.next = Node(data)

    def insert_at_beginning(self, data) -> None:
        """Insert data at beginning of linked list"""
        if self.head is None:
            # If head is None, set head to tmp
            self.head = Node(data)
        tmp = Node(data)
        tmp.next = self.head
        self.head = tmp

    def insert_at_end(self, data) -> None:
        """ Insert data at the end of linked list """
        if self.head is None:
            self.head = Node(data)

        # Traverse to end of list
        current = self.head

        while current.next is not None:
            current = current.next

        current.next = Node(data)

    def remove(self, data) -> None:
        """ Remove data from linked list

        Args:
        data: data to remove from linked list
        """

        current = self.head
        previous = None

        while current is not None:
            if current.data == data:
                if previous is None:
                    self.head = current.next
                else:
                    previous.next = current.next
            previous = current
            current = current.next

    def remove_duplicates(self) -> None:
        """ Remove duplicates from linked list """

        if self.head is None:
            return

        current = self.head

        while current is not None:
            previous = current
            runner = current.next

            while runner is not None:
                if runner.data == current.data:
                    previous.next = runner.next
                previous = runner
                runner = runner.next
            current = current.next

    def print_list(self) -> None:
        """Print linked list"""
        current = self.head
        while current is not None:
            print(f"{current.data} -> ", end="")
            current = current.next
        if current is None:
            print("None")


def main():
    """Main function"""
    my_ll = LinkedList()
    my_ll.insert(1)
    my_ll.insert(2)
    my_ll.insert(3)
    print("Linked list:")
    my_ll.print_list()
    print("Insert at beginning:")
    my_ll.insert_at_beginning(0)
    my_ll.print_list()
    print("Insert at end:")
    my_ll.insert_at_end(4)
    my_ll.print_list()
    print("Remove 2:")
    my_ll.remove(2)
    my_ll.print_list()
    my_ll.insert(1)
    my_ll.insert(2)
    my_ll.insert(3)
    my_ll.insert(4)
    print("Before removing duplicates:")
    my_ll.print_list()
    print("After removing duplicates:")
    my_ll.remove_duplicates()
    my_ll.print_list()


if __name__ == "__main__":
    main()
