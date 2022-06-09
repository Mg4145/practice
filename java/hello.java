import java.util.Scanner;

public class hello {
    public static void main(String[] args) {
        String name;

        // System.out.println("Hello World!");

        Scanner input = new Scanner(System.in);
        System.out.println("What is your name?");

        name = input.nextLine();

        System.out.println("Nice to meet you " + name + "!");

        input.close(); 
    }
}
