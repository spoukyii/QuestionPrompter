import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.util.Collections;
import java.util.List;
import java.util.Scanner;

public class QuestionChecker {

    public static void main(String[] args) {
        try {
            List<String> lines = Files.readAllLines(new File("src/question.txt").toPath());

            Collections.shuffle(lines);

            Scanner scanner = new Scanner(System.in);
            int score = 0;
            int questions = 0;
            for (String line : lines) {
                String[] parts = line.split(":");
                if (parts.length != 2) {
                    System.out.println("Invalid format: " + line);
                    continue;
                }

                String question = parts[0].trim();
                String correctAnswer = parts[1].trim();

                while (true) {
                    System.out.println(question);
                    questions += 1;
                    System.out.print("Your answer: ");
                    String userAnswer = scanner.nextLine().trim();

                    if (userAnswer.equalsIgnoreCase(correctAnswer)) {
                        System.out.println("Correct!\n");
                        score += 1;
                        break;
                    } else {
                        System.out.println("Incorrect. Try again.\n");
                    }
                }
            }
            System.out.println("Final score is" + score + "/"+questions);
        } catch (IOException e) {
            System.out.println("Error reading the file.");
            e.printStackTrace();
        }
    }
}
