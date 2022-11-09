use rand::Rng;

pub struct RollResult {
    pub rolls: Vec<i32>,
    pub total: i32,
}

#[derive(Copy, Clone)]
pub struct DiceRoll {
    dice: i32,
    sides: i32,
    target: i32,
    target_number_enabled: bool,
    ones_subtract_enabled: bool,
    explode_on: i32,
    exploding_dice_enabled: bool,
    modifier: i32,
}

impl DiceRoll {
    pub fn new() -> DiceRoll {
        DiceRoll {
            dice: 1,
            sides: 6,
            target: 0,
            target_number_enabled: false,
            ones_subtract_enabled: false,
            explode_on: 0,
            exploding_dice_enabled: false,
            modifier: 0,
        }
    }
    
    pub fn dice(mut self, amount: i32) -> DiceRoll {
        self.dice = amount;
        self
    }
    
    pub fn sides(mut self, amount: i32) -> DiceRoll {
        self.sides = amount;
        self
    }
    
    pub fn target(mut self, number: i32) -> DiceRoll {
        self.target = number;
        self.target_number_enabled = true;
        self
    }
    
    pub fn ones_subtract(mut self, enabled: bool) -> DiceRoll {
        self.ones_subtract_enabled = enabled;
        self
    }
    
    pub fn explode_on(mut self, number: i32) -> DiceRoll {
        self.explode_on = number;
        self.exploding_dice_enabled = true;
        self
    }
 
    pub fn modifier(mut self, number: i32) -> DiceRoll {
        self.modifier = number;
        self
    }
}

pub fn roll(configuration: DiceRoll) -> RollResult {
    if configuration.sides < 1 {
        panic!("dice must have at least one side.");
    }

    if configuration.target_number_enabled && configuration.target > configuration.sides {
        panic!("if target number is enabled, your target number must not be higher than the amount of sides on your dice.");
    }

    if configuration.target_number_enabled && configuration.target < 1 {
        panic!("if target number is enabled, your target number must not be lower than one.");
    }

    if configuration.exploding_dice_enabled && configuration.explode_on < configuration.target {
        panic!("you can't explode dice on a number smaller than the target number.");
    }

    if configuration.exploding_dice_enabled && configuration.explode_on == 1 {
        panic!("explode_on must be higher than one, otherwise exploding successes would continue infinitely.");
    }

    let mut rng = rand::thread_rng();
    let mut sum = 0;
    let mut successes = 0;
    let mut i = 0;
    let mut rolls = Vec::new();

    while i < configuration.dice {
        let result = rng.gen_range(1..=configuration.sides);
        rolls.push(result);
        if result >= configuration.target {
            successes = successes + 1;
            if configuration.exploding_dice_enabled && result >= configuration.explode_on {
                i = i - 1;
            } 
        } else if configuration.ones_subtract_enabled && result == 1 {
            successes = successes - 1;
        }
        sum = sum + result;
        i = i + 1;
    }

    let total = if configuration.target_number_enabled {
        successes
    } else {
        sum + configuration.modifier
    };

    RollResult { total: total, rolls: rolls }
}

pub fn roll_with_advantage(configuration: DiceRoll) -> [RollResult; 2] {
    let left_roll = roll(configuration);
    let right_roll = roll(configuration);

    let results: [RollResult; 2] = if left_roll.total > right_roll.total {
        [left_roll, right_roll]
    } else {
        [right_roll, left_roll]
    };

    results
}

pub fn roll_with_disadvantage(configuration: DiceRoll) -> [RollResult; 2] {
    let left_roll = roll(configuration);
    let right_roll = roll(configuration);

    let results: [RollResult; 2] = if left_roll.total < right_roll.total {
        [left_roll, right_roll]
    } else {
        [right_roll, left_roll]
    };

    results
}