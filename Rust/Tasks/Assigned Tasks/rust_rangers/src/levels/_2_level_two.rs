use tokio::time::{sleep, Duration};

#[tokio::main]
pub async fn start_level() {
    println!("** Game Level 2 – Loops of the River **");
    println!("You, the brave adventurer, find yourself at the edge of the great River of Code. To cross it and continue your quest, you must master the river’s three magical currents: for, while, and loop. Each current has its own rhythm, and you need to choose the right one for each challenge. Along the way, you’ll learn how to control your journey using break and continue.");

    sleep(Duration::from_millis(4000)).await;

    println!("The first current you encounter is the for current. This one flows in a predictable pattern, taking you from one shore to the other, visiting each stone in the river. You know how many stones there are, so you can count every step along the way. Here's how you use the for loop to cross the river by stepping on the stones:");

    sleep(Duration::from_millis(2000)).await;

    for steps in 1..=10 {
        println!("You step on the {} stone!", steps);
    }

    sleep(Duration::from_millis(2000)).await;

    println!("As you continue downstream, you encounter the while current. This current is tricky—it only flows when the water level is low. If the water rises too high, you have to wait. You can only proceed while the river remains calm. Here's how you control your movement with a while loop:");

    sleep(Duration::from_millis(3000)).await;

    let mut water_level = 1;

    while water_level <= 5 {
        println!("You swim through calm waters at level {}", water_level);
        water_level += 1;
    }
    println!("** The water is too high now, you need to stop. **");

    sleep(Duration::from_millis(2000)).await;

    println!("Next, you encounter the loop current, a never-ending spiral that flows endlessly around the river. This current keeps swirling in circles, but you at circle 4 you need to stop. You can keep going forever, or use the break command to halt your journey. You get curious and use the loop to explore the river’s depths:");

    sleep(Duration::from_millis(4000)).await;

    let mut circle = 0;
    loop {
        circle += 1;

        println!("You are spinning in circle number {}", circle);

        if circle == 4 {
            println!("You need to stop after 4 circles.");
            break;
        }
    }

    sleep(Duration::from_millis(2000)).await;

    println!("Finally, you find yourself encountering obstacles in the river, like logs, fish, culverts, vortexes, and branches. You want to stop your journey when a 'vortex' occurs, and you use continue to skip over the obstacles and keep paddling forward. Here's how you skip an obstacle using continue & break:");
    sleep(Duration::from_millis(3000)).await;

    let obstacles = [
        "log", "branch", "fish", "culverts", "vortex", "dams", "weirs",
    ];

    for obstacle in obstacles.iter() {
        if *obstacle == "branch" {
            println!("You skip the branch!");
            continue;
        }

        if *obstacle == "vortex" {
            println!("You need to stop when {obstacle} occurs!");
            break;
        }
        println!("You encounter a {} in the river.", obstacle);
    }

    sleep(Duration::from_millis(2000)).await;

    println!("With each current you mastered, you became more confident navigating the River of Code. You crossed it using the for, while, and loop currents, controlling your flow with break and continue. The river’s mysteries no longer posed a challenge, and you knew that no matter what came next, you could handle it.");
}
