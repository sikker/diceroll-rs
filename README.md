# Diceroll
A simple dice rolling lib for RPG purposes in Rust.

## Usage
Install the package by adding it to Cargo.toml:
```toml
[dependencies]
quote = "1.0"
```

The basic way you use the library is creating a diceroll using `DiceRoll::new()`, configuring it with the builder pattern with the parameters you want. Then you pass said configuration to the `roll()` function, giving you a `RollResult` struct containing an i32 `total` and a Vec<i32> of individual `rolls`. It is also possible to pass the configured diceroll to `roll_with_advantage` or to `roll_with_disadvantage`, which will return an array of two roll results with index 0 always containing the "winning" roll (with disadvantage this means the lowest result, with advantage this means the highest), and with index 1 always containing the discarded roll. 

The following parameters are supported:
 - dice [i32] (mandatory): The amount of dice to roll.
 - sides [i32] (mandatory): How many sides each die should have.
 - target [i32]: Off by default. If this parameter is set, instead of the total containing the sum of the dicerolls, the total will show how many individual dice yielded a result higher than or equal to the target number. This is useful for any RPG that uses dicepool rolling, such as World of Darkness and Shadowrun.
 - ones_subtract [bool]: Off by default. If a target number is set, enabling this will remove a success for every die that turns up 1. 
 - explode_on [i32]: Off by default. Will record the result and then reroll any dice that rolls higher than or equal to the explode_on number. 
 - modifier [i32]: Off by default. Will add itself to (or subtract from, if the number is negative) the total sum of the roll. Has no effect on rolls that use a target number.

## Examples

### A basic roll:
```rust
use diceroll::*;

fn main() {
    let amount_of_dice = 2;
    let sides = 6;
    let modifier = 2;
    let result = roll(DiceRoll::new()
                 .dice(amount_of_dice)
                 .sides(sides)
                 .modifier(modifier))
    println!("We rolled {}d{}+{}, which yielded a total of {}.", amount_of_dice, sides, modifier, result.total);
}
```
This will give an output looking something like this:
```
We rolled 2d6+2, which yielded a total of 9.
```

### Roll with advantage
Note that we are using the itertools crate for joining the roll results in this example.
```rust
use itertools::Itertools;
use diceroll::*;

fn main() {
    let amount_of_dice = 1;
    let sides = 20;
    let modifier = 2;
    let result = roll_with_advantage(DiceRoll::new()
        .dice(amount_of_dice)
        .sides(sides)
        .modifier(modifier));
    println!("We rolled {}d{}+{} with advantage", amount_of_dice, sides, modifier);
    println!("--- The winning roll ({}) yielded a total of {}", Itertools::join(&mut result[0].rolls.iter(), ","), result[0].total);
    println!("--- The discarded roll ({}) yielded a total of {}", Itertools::join(&mut result[1].rolls.iter(), ","), result[1].total);
}
```

### New World of Darkness/Chronicles of Darkness dice pool roll
Note that we are using the itertools crate for joining the roll results in this example.
```rust
use itertools::Itertools;
use diceroll::*;

fn main() {
    let amount_of_dice = 5;
    let sides = 10;
    let target = 8;
    let explode_on = 10;
    let result = roll(DiceRoll::new()
        .dice(amount_of_dice)
        .sides(sides)
        .target(target)
        .explode_on(explode_on));
    println!("We rolled a dice pool of {}d{} with a target number of {} and exploding successes on {}", amount_of_dice, sides, target, explode_on);
    println!("--- The result of the rolls ({}) yielded a total of {}", Itertools::join(&mut result[0].rolls.iter(), ","), result.total);
}
```