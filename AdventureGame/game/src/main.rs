use std::io;
use rand::Rng;
use colored::*;

pub mod NPC;
pub mod weapons;

extern crate rand;
extern crate colored;

//fail is red, sucess is green and Orange is an option could also have the option only appear if level required.
//Player needs to select a starting weapon

struct Character{
    //create character fields that can be filled in like DnD stats.
    name: String,
    armour_class: i32,  //the calculation for this can be implemented more later.
    experince_points: i32,
    level: i32,
    hitpoints: i32,
    strength: i32,
    dexterity: i32,
    constitution: i32,
    intelligence: i32,
    wisdom: i32,
    charisma: i32,
    weapon: weapons::MeleeWeapon,
    //if lie could have an alias to use?
}

impl Character{
    //methods which the player character can do.

    //create new player character with base stats of 8 for everything.
       pub fn new_player_character(name: String,selected_weapon: weapons::MeleeWeapon) -> Self {
        Character {
            name: name,
            armour_class: 15,
            experince_points: 0,
            level: 1, //update this by calling the experince
            hitpoints: 13,   //this will be updated based off level.
            strength: 15,
            dexterity: 14,
            constitution: 13,
            intelligence: 12,
            wisdom: 10,
            charisma: 8,
            weapon:selected_weapon,
        }
    }

//place holder values for now. Call this to update the level of the player with no gained xp.
    fn calculate_level(&mut self){
        let xp = self.experince_points;
        match xp{
             xp if xp < 300 => self.level = 1,
             xp if xp > 300 && xp < 900 => self.level = 2,
             xp if xp > 900 && xp < 2700 => self.level = 3,
             xp if xp > 2700 => self.level = 4,
             _ => return self.level = 0
        }
    }

    pub fn print_values(& self){
        println!("Name: {} Armour Class: {} Str: {} Dex: {} Con: {} Int: {} Wis: {} Char {} Level: {} Weapon: {}\n",
         self.name, self.armour_class, self.strength, self.dexterity, self.constitution, 
         self.intelligence, self.wisdom, self.charisma, self.level, self.weapon.name);
    }

    pub fn gain_xp(&mut self, xp_to_gain: i32){  //pass the defeated event/NPC, it will have an experince field.
        self.experince_points = self.experince_points + xp_to_gain;
        self.calculate_level(); //checks to see if a level has been gained.
    }

    //mofiying the values of the struct. By passing &mut self.
    //pub fn add_xp(&mut self){
      //  self.experince_points += 1000;
    //}
}

fn main() {
    let mut user_input = String::new();
    println!("Enter Name of your Character..");

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read input");
    let character_name = user_input.trim().to_string();

    println!("Select a starting weapon");
    println!("(1). Longsword");
    println!("(2). Hand Axe");
    println!("(3). Spear");
    let mut answer = question_answer_function();

    let mut personal_weapon;
    if  answer == 1{
        personal_weapon = weapons::MeleeWeaponTraits::new_longsword();
    } else if answer == 2{
        personal_weapon = weapons::MeleeWeaponTraits::new_hand_axe();
    } else{
        personal_weapon = weapons::MeleeWeaponTraits::new_spear();
    }

    let mut player = Character::new_player_character(character_name, personal_weapon);
    player.print_values();
    //new character setup finished.

    //move the entire player object into the town gate function.
    town_gate(player);
}
//easy to reuse code which returns a number from the terminal.
fn question_answer_function()-> u64{
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read");
    //throw an error the user inputted text isn't a unsigned int.
    let number: u64 = input.trim().parse().expect("Expected a Number");
    return number;
}

//first scene will always be a town gate. guard hails to you ask you're business here.
fn town_gate(player: Character){
    println!("You arrive outside of town after a long journey\nA guard calls down to you and asks what your business is, you reply:");
    //lie would require higher skills.
    println!("(1). {} {}", "(Lie)".cyan() , "Hello my name is Hagar, im a traveling merchant looking to purchase goods from the town.");
    println!("(2). I have no idea where I am, but my name is {}.\n", player.name);
    let mut answer = question_answer_function();

    //if option is lie can roll a random number for the guard and the player see who wins.
    if answer == 1{
        let mut rng = rand::thread_rng();
        let mut guard_roll = rng.gen_range(0, 10); //ignore stats for the moment just roll pure RNG
        let mut player_roll = rng.gen_range(0, 10);

            if guard_roll >= player_roll{  //could have genders.
                println!("{} {}", "(Fail)".bright_red() ,"You think im a fool boy? Do you think you look like a traveling merchant?");
                //guard knows its a lie so call them out on it.
                println!("I'll ask one more time or i'll have to deal with you, Whats your name and what is your business?\n");

                println!("(1). {} {}", "(Lie again)".cyan(), "I swear im a traveling merchant, I was robbed on the way..");
                println!("(2). Sorry sir I was scared, Im not sure where I am my name is {}\n", player.name);

                answer = question_answer_function();

                if answer == 1{
                    //roll against guard, this time with advantage as you've been caught once.
                    guard_roll = rng.gen_range(0, 10); 
                    player_roll = rng.gen_range(0, 8);

                    //lost another roll.
                    if guard_roll >= player_roll{
                        println!("{} {}", "(Fail)".bright_red() ,"Ahh thats it, im coming down to deal with you.");
                        //can run, fight or submit to arrest.
                         println!("The guard is coming down, what do you choose to do?\n");
                         println!("(1). Choose to run and hope nobody follows.");
                         println!("(2). Choose to submit to the guard, risking probable arrest.");
                         println!("(3). Fight the guard");
                         answer = question_answer_function();
                         //from here we will be changing scenes so regardless we will be moving from this function.

                         if answer == 1{
                            //run away new scene.
                         }else if answer == 2{
                            //jail, new scene
                         }else{
                             let guard: NPC::Guard =<NPC::Guard as NPC::Opponents>::new_easy_guard();
                             combat_sim(player, guard);
                             //we fight
                         }

                    }else {
                           println!("urgh another one fleeing, come on in then i'll set you up with someone who can help.");  
                    }

                }else if answer == 2{
                    //told the truth guard will deal with you normally.
                    println!("FILL IN ANSWER 2")                    
                }

            }else{  //have the alias here, allow for unique names.
                println!("{} {}", "(Success)".green() ,"Ahh yes welcome to the town of Gusthall sir, please enjoy your stay");
                //continue with the story, he lets the player in.
            }
         
    }else if answer == 2{
          println!("{} {}", "(Success)".green() ,"urgh another one fleeing, come on in then i'll set you up with someone who can help.");
    }

}


//should be able to pass any type as long as it has the opponenet trait. ie Guard has the trait so it can be accepted.
//combat function which simulates the fight between 2 characters? eg guard and player returns a victor. has combat choices etc..
fn combat_sim<T: NPC::Opponents> (player: Character, opp: T){  
    println!("<<STARTING FIGHT>>");
    let mut rng = rand::thread_rng();
    let opp_initiative = rng.gen_range(0, 20);
    let player_initiative = rng.gen_range(0, 20);

     //roll initiative, could also add a modifier here.
    if player_initiative > opp_initiative{
            //player attacks first
    }else{
            //opp attacks first
    }
}



//make a dice rolling method take 2 parameters, could possible take a player and opponenet to do more
//complex equation on their stats.  2 params could be the range to roll between.

//death method