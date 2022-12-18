""" Create a Dog class """


class Dog:
    """Create a Dog class"""

    def __init__(self, name: str, age: int) -> None:
        """Initialize the Dog class
        Args:
        name (str): name of the dog
        age (int): age of the dog
        """

        self.name = name
        self.age = age

    def bark(self) -> None:
        """Bark method"""
        print(f"{self.name} says woof!")

    def get_info(self) -> None:
        """Get info method"""
        print(f"{self.name} is {self.age} years old")

    def set_age(self, age: int) -> None:
        """Set age method"""
        self.age = age

    def roll_over(self) -> None:
        """Roll over method"""
        print(f"{self.name} rolls over!")


class GoldenRetriever(Dog):
    """Create a GoldenRetriever class"""

    def __init__(self, name: str, age: int, fur_color: str) -> None:
        """Initialize the GoldenRetriever class
        Args:
        name (str): name of the dog
        age (int): age of the dog
        """

        super().__init__(name, age)
        self.fur_color = fur_color

    def bark(self) -> None:
        """Bark method"""
        print(f"{self.name} says woof! woof!")


def main() -> None:
    """Main function"""
    dog = Dog("Spot", 5)
    dog.bark()
    dog.get_info()
    dog.set_age(10)
    dog.get_info()
    dog.roll_over()

    golden = GoldenRetriever("Max", 3, "golden")
    golden.bark()
    golden.get_info()
    golden.set_age(10)
    golden.get_info()
    golden.roll_over()


if __name__ == "__main__":
    main()
