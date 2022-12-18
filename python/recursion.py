""" Recursion examples. """


def factorial(number):
    """Calculate the factorial of a number."""
    if number < 0:
        raise ValueError("Factorial of negative numbers is not defined.")

    # Base case
    if number == 0:
        return 1

    # Recursive case
    return number * factorial(number - 1)


def fibonacci(number):
    """Calculate the fibonacci sequence of a number."""
    if number < 0:
        raise ValueError("Fibonacci of negative numbers is not defined.")

    # Base case
    if number in (0, 1):
        return number

    # Recursive case
    return fibonacci(number - 1) + fibonacci(number - 2)


def lucas(number):
    """Calculate the lucas sequence of a number."""
    if number < 0:
        raise ValueError("Lucas of negative numbers is not defined.")

    # Base case
    if number == 0:
        return 2
    if number == 1:
        return 1

    # Recursive case
    return lucas(number - 1) + lucas(number - 2)


def main():
    """Main"""
    factorial_list = [factorial(number) for number in range(10)]
    lucas_list = [lucas(number) for number in range(10)]
    fibonacci_list = [fibonacci(number) for number in range(10)]

    print("Factorial: {}".format(factorial_list))
    print("Lucas: {}".format(lucas_list))
    print("Fibonacci: {}".format(fibonacci_list))


if __name__ == "__main__":
    main()
