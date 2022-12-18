""" This module will be used to demonstrate the use of abstract classes """

from abc import ABC, abstractmethod


class Animal(ABC):
    """This is an abstract class"""

    def __init__(self, name: str) -> None:
        self.name = name

    @abstractmethod
    def speak(self) -> None:
        """This is an abstract method"""

    @abstractmethod
    def move(self) -> None:
        """This is an abstract method"""

    @abstractmethod
    def description(self) -> None:
        """This is an abstract method"""


class Dog(Animal):
    """This is the Dog class"""

    def __init__(self, name: str, breed: str) -> None:
        """This is a concrete class"""
        super().__init__(name)
        self.breed = breed

    def speak(self):
        """This is a concrete method"""
        return "Woof!"

    def move(self):
        return "Run!"

    def description(self):
        return f"{self.name} is a {self.breed} dog."


class Cat(Animal):
    """This is the Cat class"""

    def __init__(self, name: str, breed: str) -> None:
        """This is a concrete class"""
        super().__init__(name)
        self.breed = breed

    def speak(self):
        """This is a concrete method"""
        return "Meow!"

    def move(self):
        return "Walk!"

    def description(self):
        return f"{self.name} is a {self.breed} cat."


def main() -> None:
    """This is the main function"""
    dog = Dog("Rex", "Labrador")
    cat = Cat("Fluffy", "Persian")
    print(dog.description())
    print(dog.speak())
    print(dog.move())
    print(cat.description())
    print(cat.speak())
    print(cat.move())


if __name__ == "__main__":
    main()
