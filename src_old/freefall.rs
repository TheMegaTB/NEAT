
use nn::NeuralNetwork;
use rand::{Rng, self};

pub fn ff(nn: &NeuralNetwork, user_friendly: bool) -> usize {
    let name = "Karsten";

    let mut rng_thread = rand::thread_rng();

    let terminal_velocity = 56.0; // Maximum velocity
    let acceleration = 9.80665; // Gravity     <-|
    // let deceleration = -8.9408; // Chute       <-| m/s^2
    let deceleration = -((rng_thread.next_f64() * 9.0 + 1.0).abs());
    let weight = 75.0; //kg
    let chute_speed = 5.1815; // Maximum speed when opened
    let interval = 0.1; // interval of checks. May not be bigger than 1!
    let crater_depth = 17.0; //cm

    let mut height = 1000.0;
    let mut velocity = 0.0;
    let mut time = 0.0;
    let mut chute_opened = false;
    let mut chute_pulled_at = height+1.0;

    while height > 0.0 {
        velocity += if !(chute_opened) { acceleration * interval } else { deceleration * interval };
        if chute_opened && velocity < chute_speed {
            velocity = chute_speed;
        } else if !(chute_opened) && velocity > terminal_velocity {
            velocity = terminal_velocity;
        }
        height += -(velocity * interval);

        let nn_res = nn.calculate(vec![height/1000.0, velocity, deceleration], 1.0).unwrap()[0];
        if !(chute_opened) && nn_res > 1.0 {
            chute_opened = true;
            chute_pulled_at = height;
            if user_friendly { println!("CHUTE PULLED AT {} meters!", height) };
        };

        time += interval;
    }

    let kinetic_energy = (1.0 / 2.0) * velocity * weight;
    let force = kinetic_energy / crater_depth;
    let perfect_height = terminal_velocity*terminal_velocity / (2.0*(-deceleration));
    if user_friendly {
        if force > 85.0 {
            println!("{name} faceplanted into the ground after {:.*} seconds creating a {} cm deep crater whilst experiencing a force of {:.*} N.", 0, time, crater_depth, 0, force, name = name);
            println!("Unfortunally he was too stupid to figure out how to use this thing people call parachute... how sad...");
        } else if force > 50.0 {
            println!("{name} landed hardly on the ground after {:.*} seconds experiencing a force of {:.*} N.", 0, time, 0, force, name = name);
            println!("Even though it was a very rough landing {name} survived with many broken bones. It's at least something.", name = name);
        } else if force > 20.0 {
            println!("{name} actually managed to pull his chute {:.*} seconds into the dive!", 0, time, name = name);
            println!("Somehow he managed not to break any bones or injure himself even though his legs feel somewhat jelly now...");
            println!("May've been due to the whopping {:.*} Newtons of force stopping his 'majestic' glide.", 0, force);
        } else if force > 0.0 {
            println!("WOW! {name} actually landed! And very softly as well! Only a mere {:.*} Newtons were required to stop him.", 0, force, name = name);
            println!("Regardless he really enjoyed his {:.*} seconds in air and pulled early! Room for improvements!", 0, time);
        } else {
            println!("WHUUUT?! {name} managed to defy the laws of physiz! He actually experienced {:.*} N of force!", 0, force, name = name);
            println!("He just hit the ground and got even more magical energy pushing him deeper into the ground...");
        }
        println!("ph: {}, dc: {}", perfect_height, deceleration);
    }
    if force <= 20.0 { chute_pulled_at as usize - perfect_height as usize } else { 1001 }
}
