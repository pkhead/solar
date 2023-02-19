
/*
use std::env;
use std::fs;
pub mod app;

fn main() {
    let args: Vec<String> = env::args().collect();

    // get file path argument
    let file_path = match args.get(1) {
        None => {
            println!("error: expected file path");
            std::process::exit(1);
        }
        Some(v) => v,
    };

    // open file
    let mut file = match fs::File::open(file_path) {
        Ok(v) => v,
        Err(e) => {
            println!("error: could not open file: {}", e);
            std::process::exit(1);
        }
    };

    // run application
    if let Err(e) = app::run(&mut file) {
        println!("error: {}", e);
        std::process::exit(1);
    }
}*/

use sb3::*;
//use std::env;

pub mod sb3;

fn main() {
    /*
    for (i, arg) in env::args().enumerate() {
        println!("arg {}: {}", i, arg);
    }
    */
    
    let mut project = Project::new();
    let sprite = project.create_sprite("Sprite1");

    let mut script = Script::new();

    // when green flag clicked
    script.push(Block::new(Opcode::WhenGreenFlagClicked()));

    // move 10 steps 
    script.push(Block::new(Opcode::MoveSteps(UserInput::new(InputType::Number, Value::Number(10.0), None))));

    // add script to sprite
    sprite.obj.scripts.push(script);

    // add costumes to sprite
    sprite.obj.costumes.push(Costume::new("costume1", "assets/cat1.svg", 48.0, 50.0));

    // add global "my variable" = 0
    project.data.borrow_mut().vars.push(Variable::new("my variable", Value::Number(0.0)));

    project.save("out.sb3");

    match project.serialize() {
        Ok(serialized) => {
            println!("{}", json::stringify_pretty(serialized, 4));
        },
        Err(e) => println!("error: {}", e)
    }
}
