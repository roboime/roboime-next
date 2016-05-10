import java.util.*;
import java.io.*;
import java.math.*;

class Demo {
    public static void main(String args[]) {
        Scanner in = new Scanner(System.in);

        System.err.println("started");

        // Version check I/O

        final int compatVersion = 1;
        in.skip("ROBOIME_AI_PROTOCOL");
        int version = in.nextInt();
        if (version == compatVersion) {
            System.out.format("COMPATIBLE %d\n", compatVersion);
        } else {
            System.out.format("NOT_COMPATIBLE %d\n", compatVersion);
            return;
        }

        System.err.println("compatible");

        // Geometry input

        float fieldLength = in.nextFloat();
        float fieldWidth = in.nextFloat();
        float goalWidth = in.nextFloat();
        float centerCircleRadius = in.nextFloat();
        float defenseRadius = in.nextFloat();
        float defenseStretch = in.nextFloat();
        float freeKickFromDefenseDistance = in.nextFloat();
        float penaltySpotFromFieldLineDist = in.nextFloat();
        float penaltyLineFromSpotDist = in.nextFloat();

        System.err.println("initialized");

        // Game state I/O

        while (true) {

            // State

            Vector<Integer> ids = new Vector<Integer>();
            float x = 0.0f, y = 0.0f, w = 0.0f;
            float tx, ty, tw;

            // Input

            int counter = in.nextInt();
            float timestamp = in.nextFloat();
            char refereeState = in.next("[A-Z]").charAt(0);
            float refereeTimeLeft = in.nextFloat();
            int scorePlayer = in.nextInt();
            int scoreOpponent = in.nextInt();
            int goalkeeperIdPlayer = in.nextInt();
            int goalkeeperIdOpponent = in.nextInt();
            int robotCountPlayer = in.nextInt();
            int robotCountOpponent = in.nextInt();

            float ballX = in.nextFloat();
            float ballY = in.nextFloat();
            float ballVx = in.nextFloat();
            float ballVy = in.nextFloat();

            for (int i = 0; i < robotCountPlayer; i++) {
                int robotId = in.nextInt();
                float robotX = in.nextFloat();
                float robotY = in.nextFloat();
                float robotW = in.nextFloat();
                float robotVx = in.nextFloat();
                float robotVy = in.nextFloat();
                float robotVw = in.nextFloat();

                ids.add(robotId);
                if (robotId == 0) {
                    x = robotX;
                    y = robotY;
                    w = robotW;
                }
            }

            for (int i = 0; i < robotCountOpponent; i++) {
                int robotId = in.nextInt();
                float robotX = in.nextFloat();
                float robotY = in.nextFloat();
                float robotW = in.nextFloat();
                float robotVx = in.nextFloat();
                float robotVy = in.nextFloat();
                float robotVw = in.nextFloat();
            }

            tx = ballX;
            ty = ballY;
            tw = 0.0f;

            System.out.println(counter);

            for (int robotId : ids) {
                float vTan = 0.0f;
                float vNorm = 0.0f;
                float vAng = 0.0f;
                float kickX = 0.0f;
                float kickZ = 0.0f;
                int spin = 0;

                if (robotId == 0) {
                    final float PL = 0.40f;
                    final float PW = 0.80f;
                    vTan  = PL * (float)((tx - x) * Math.cos(w) + (ty - y) * Math.sin(w));
                    vNorm = PL * (float)((ty - y) * Math.cos(w) - (tx - x) * Math.sin(w));
                    vAng  = PW * (tw - w);
                    kickX = 4.0f;
                    kickZ = 0.0f;
                    spin = 1;
                }

                System.out.format("%f %f %f %f %f %d\n", vTan, vNorm, vAng, kickX, kickZ, spin);
            }
        }
    }
}
