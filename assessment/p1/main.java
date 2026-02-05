import java.util.Scanner;

public class main {
    public static void main(String[] args) {
        Scanner in = new Scanner(System.in);
        int num = in.nextInt();
        for (int count = 0; count < num; count++){
            int n = in.nextInt();
            if ((n / 2) % 2 == 1) {
                System.out.println("NO");
            } else {
                System.out.println("YES");
                // for (long i = 0; i < ((n / 2) / 2); i++) {
                //     System.out.print((i + 1) * 8);
                //     System.out.print(" ");
                //     System.out.print((i + 1) * 8 + 2);
                //     System.out.print(" ");
                // }
                // for (int i = 0; i < ((n / 2) / 2); i++) {
                //     System.out.print((i + 1) * 8 - 1);
                //     System.out.print(" ");
                //     System.out.print((i + 1) * 8 + 2 + 1);
                //     System.out.print(" ");
                // }
                for (int i = 0; i < (n / 2); i++) {
                    System.out.println((i + 1) * 2);
                }
                for (int i = 0; i < (n / 2) - 1; i++) {
                    System.out.println((i + 1) * 2 - 1);
                }
                System.out.println(n + (n / 2) - 1);
            }
        }
    }
}