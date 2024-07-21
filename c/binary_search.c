#include <stdio.h>

int binary_search(int arr[], int size, int key) {
  int left = 0;
  int right = size;

  while (left <= right) {
    int mid = left + (right - left) / 2;

    if (arr[mid] == key) {
      return mid;
    } else if (arr[mid] < key) {
      left = mid + 1;
    } else {
      right = mid - 1;
    }
  }

  return -1;
}

int main(int argc, char **argv) {

  int arr[] = {1, 2, 3, 4, 5, 6, 7, 8, 9};
  int size = sizeof(arr) / sizeof(arr[0]);

  for (int i = 0; i < size; i++) {
    printf("Element %d is found at index %d\n", arr[i],
           binary_search(arr, size, arr[i]));
  }

  return 0;
}
