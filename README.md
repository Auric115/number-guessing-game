# Number Guessing Game

A fun and simple CLI-based game where you can test your luck by guessing a randomly selected number within a limited number of attempts.

## About the Project

In this project, you will build a **Number Guessing Game** where:
- The computer randomly selects a number between 1 and 100.
- The user has a limited number of chances to guess the number, determined by the selected difficulty level.
- Feedback is provided for every guess, and the game concludes when the number is guessed or the chances are exhausted.

[Project Link](https://roadmap.sh/projects/number-guessing-game)

---

### Gameplay

1. **Welcome Message**: The game starts with a welcome message and the rules.
2. **Difficulty Selection**:
   - **Easy**: 10 chances
   - **Medium**: 5 chances
   - **Hard**: 3 chances
3. **Make a Guess**:
   - Feedback is provided for each guess (e.g., "The number is greater than your guess").
   - Correct guesses end the game with a congratulatory message, including the number of attempts taken.
   - The game ends when the correct number is guessed or the chances run out.

### Features to Enhance the Game

- **Replay Option**: Allow the user to play multiple rounds.
- **Timer**: Add a timer to track how long it takes to guess the number.
- **Hints**: Implement a hint system to provide clues if the user is stuck.
- **High Scores**: Track the fewest number of attempts under each difficulty level.

### Example Output

```
Welcome to the Number Guessing Game!
I'm thinking of a number between 1 and 100.
You have 5 chances to guess the correct number.

Please select the difficulty level:
1. Easy (10 chances)
2. Medium (5 chances)
3. Hard (3 chances)

Enter your choice: 2

Great! You have selected the Medium difficulty level.
Let's start the game!

Enter your guess: 50
Incorrect! The number is less than 50.

Enter your guess: 25
Incorrect! The number is greater than 25.

Enter your guess: 35
Incorrect! The number is less than 35.

Enter your guess: 30
Congratulations! You guessed the correct number in 4 attempts.
```
