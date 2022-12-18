""" Creating the fibonacci sequence using a generator function. """


def recursive_fibonacci(nth_fib):
    """Recursive function to generate the fibonacci sequence."""
    if nth_fib in (0, 1):
        return nth_fib

    return recursive_fibonacci(nth_fib - 1) + recursive_fibonacci(nth_fib - 2)


def iterative_fibonacci(nth_fib):
    """Iterative function to generate the fibonacci sequence."""
    if nth_fib in (0, 1):
        return nth_fib

    fib1 = 0
    fib2 = 1
    for _ in range(nth_fib - 1):
        fib1, fib2 = fib2, fib1 + fib2

    return fib2


def generator_fibonacci(nth_fib):
    """Generator function to generate the fibonacci sequence."""
    fib1 = 0
    fib2 = 1
    for _ in range(nth_fib + 1):
        yield fib1
        fib1, fib2 = fib2, fib1 + fib2


def main():
    """Main function."""
    nth_fib = 10
    print(f"Recursive fibonacci: {recursive_fibonacci(nth_fib)}")
    print(f"Iterative fibonacci: {iterative_fibonacci(nth_fib)}")
    print(f"Generator fibonacci: {list(generator_fibonacci(nth_fib))}")


if __name__ == "__main__":
    main()
