<div align="center">

# old-dice
  <img src="https://github.com/user-attachments/assets/f0e04f8a-8b06-40c9-8938-0f21cb9b981f" alt="old-dice logo" width="300"/>

## Extremely simple dice simulator cli tool
</div>

### Usage:
```bash
old-dice --dice-sides <number_of_sides> --modifier <modifier_value> --times <number_of_times>
```
* `--dice-sides <number_of_sides>`:

  This argument specifies the number of sides on the dice. For example, 20 means a 20-sided dice.
  Example: --dice-sides 20 for a 20-sided dice.
* `--modifier <modifier_value>`:

  This argument is optional and allows you to add a modifier to the dice roll. If you specify a modifier, it will be added to the result of the dice roll.
  Example: --modifier 5 adds 5 to the result of the roll.
* `--times <number_of_times>`:

  This argument specifies how many times you want to roll the dice. For example, 6 means you want to roll the dice 6 times.
  Example: --times 6 means rolling the dice 6 times.

### Example:
```
old-dice --dice-sides 20 --modifier 5 --times 6
9 + 5 = 14
6 + 5 = 11
18 + 5 = 23
14 + 5 = 19
19 + 5 = 24
7 + 5 = 12
[14, 11, 23, 19, 24, 12]
```

### Installation
1. Clone the repository:
  ```
  git clone https://github.com/otaviocosta2110/old-dice.git
  cd old-dice
  ```

  2. Build the project:
  ```bash
  cargo build --release
  ```

3. Copy to path
  ```bash
  sudo cp ./target/release/old-dice /usr/bin
  ```
