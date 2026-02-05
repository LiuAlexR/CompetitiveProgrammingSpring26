import java.util.Scanner;

public class main {
    public static void main(String[] args) {
        Scanner in = new Scanner(System.in);
        int num = in.nextInt();
        
        for(int i = 0; i < num; i++){
            int n = in.nextInt();
            int prev = 0;
            boolean des = false;
            boolean success = true;
            for(int j = 0; j < n; j++) {
                // System.out.println("BOOM" + j);
                int cur = in.nextInt();
                if(!des) {
                    if(cur < prev) {
                        des = true;
                    }
                } else {
                    if (cur > prev) {
                        if(success) {
                            System.out.println("NO");
                        }
                        success = false;
                        
                    }
                }
                prev = cur;
            }
            if(success) {
                System.out.println("YES");
            }
        }
    }
}
