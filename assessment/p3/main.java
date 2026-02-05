import java.util.Scanner;

public class main {
    public static void main(String[] args) {
        Scanner in = new Scanner(System.in);
        int num = in.nextInt();
        int[][] inputs = new int[num][3];
        for(var i = 0; i < num; i++) {
            inputs[i] = new int[] {in.nextInt(), in.nextInt(), in.nextInt()};
        }
        for(var i : inputs) {
            int k = i[2];
            int m = i[1];
            int n = i[0];
            if(k > 3) {
                System.out.println(0);
            } else if (k == 3) {
                if (m <= n) {
                    System.out.println(0);
                } else {
                    System.out.println(m - n - m/n + 1);
                }
            } else if (k == 2) {
                if (m <= n) {
                    System.out.println(m);
                } else {
                    System.out.println(n + m / n - 1);
                }
            } else {
                System.out.println(1);
            }
        }
    }
}