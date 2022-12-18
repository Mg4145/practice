""" Merge Sort Algorithm """


def merge(left, right):
    """Merge Function"""
    result = []
    while left and right:
        if left[0] <= right[0]:
            result.append(left.pop(0))
        else:
            result.append(right.pop(0))
    if left:
        result += left
    if right:
        result += right
    return result


def merge_sort(array):
    """Merge Sort Implementation"""
    if len(array) <= 1:
        return array
    middle = len(array) // 2
    left = merge_sort(array[:middle])
    right = merge_sort(array[middle:])
    return merge(left, right)


def main():
    """Main Function"""
    array = [1, 3, 2, 5, 4, 6, 7, 8, 9, 10]

    print(array)
    print(merge_sort(array))


if __name__ == "__main__":
    main()
