public class Insertion {

    void sort(int arr[])
    {

        for(int i = 1; i < arr.length; i++) {
            int key = arr[i];

            int j = i - 1;

            while(j >= 0 && arr[j] > key) {
                arr[j + 1] = arr[j];
                j--;
            }

            arr[j + 1] = key;
        }

    }

    static void printArray(int arr[]) {
        for (int i = 0; i < arr.length; i++)
            System.out.print(arr[i] + " ");

        System.out.println();
    }

    public static void main(String[] args)
    {

        int arr[] = {31, 41, 59, 26, 41, 58};

        printArray(arr);

        Insertion ob = new Insertion();
        ob.sort(arr);

        printArray(arr);

    }
}
