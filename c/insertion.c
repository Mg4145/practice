#include <stdio.h>

void insertionSort(int arr[], int n) {
    int i, key, j;

    for (i = 1; i < n; i++) {

        key = arr[i];
        j = i - 1;

        while (j >= 0 && key < arr[j]) {
            arr[j+1] = arr[j];
            j--;
        }

        arr[j+1] = key;
    }
}

void printArray(int arr[], int n) {
    int i;
    printf("[ ");
    for(i = 0; i < n; i++)
        printf("%d ", arr[i]);
    printf("]");
    printf("\n");
}

int main(int argc, char* argv[]) {

    int arr[] = {5, 4, 3, 2, 1};
    int n = sizeof(arr)/sizeof(arr[0]);
    printArray(arr, n);
    insertionSort(arr, n);
    printArray(arr, n);

    return 0;
}
