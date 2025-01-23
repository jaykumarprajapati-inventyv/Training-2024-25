use rand::Rng;
use tokio::time::{sleep, Duration};

#[tokio::main]

pub async fn start_level() {
    println!("** Game Level 4 – The Dark Mines of Errors **");
    println!("\n \nIn this level, you face two challenges: crossing a shaky bridge and navigating a collapsing tunnel. Both are decided by chance, with a 50/50 chance of success. The bridge might hold, or it could break, and the tunnel could either be safe or collapse. Your survival depends on the luck of the roll—will you make it through, or will the dangers end your journey?");

    sleep(Duration::from_millis(3000)).await;

    match the_shaky_bridge().await {
        Ok(_) => {
            println!("\n ** You successfully crossed the bridge! **")
        }
        Err(_) => {
            println!("** The bridge broke. You need to find another way... **");
            panic!("** The bridge was too broken to fix! Game over! **");
        }
    }

    sleep(Duration::from_millis(4000)).await;

    match the_collapsing_tunnel().await {
        Ok(_) => {
            println!("\n ** You successfully crossed the tunnel! **")
        }
        Err(_) => {
            println!("** The tunnel is collapsing. You need to save your self... **");
            panic!("** The tunnel is collapsed Game over! **");
        }
    }
}
async fn the_shaky_bridge() -> Result<(), String> {
    println!("\n \n You step onto the shaky bridge, feeling it creak and sway under your weight. The air is tense, and your fate rests on one thing: luck. Will the bridge hold steady, letting you cross safely, or will it snap and send you plummeting into the darkness below? With every step, the outcome is uncertain—survival or disaster, all decided by chance!");

    sleep(Duration::from_millis(4000)).await;

    let bridge_condition: u8 = rand::thread_rng().gen_range(0..2);

    match bridge_condition {
        0 => Ok(()),
        1 => Err("** The bridge is too broken to cross. **".to_string()),
        _ => {
            unreachable!("The value should always be 0 or 1!!")
        }
    }
}
async fn the_collapsing_tunnel() -> Result<(), String> {
    println!("\n \n You step into the collapsing tunnel, where the walls groan and rocks shift ominously above. The ground feels unstable, and each step could be your last. Will the tunnel hold, allowing you to escape safely, or will it crumble, trapping you in the dark abyss? Your survival is at the mercy of fate—will you make it through, or will the tunnel's collapse be your end?");

    sleep(Duration::from_millis(4000)).await;

    let collapsing_tunnel = rand::thread_rng().gen_range(0..2);

    match collapsing_tunnel {
        0 => Ok(()),
        1 => Err("** The tunnel is collapsing. **".to_string()),
        _ => {
            unreachable!("The value should always be 0 or 1!!")
        }
    }
}
