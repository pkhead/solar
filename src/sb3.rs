use std::rc::Rc;
use std::cell::RefCell;
use rand::Rng;
use json::{JsonValue};

pub trait JsonSerialize {
    fn serialize(&self) -> json::Result<JsonValue>;
}

#[derive(Debug)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
}

#[derive(Debug)]
pub enum InputType {
    Number, // 4
    PositiveNumber, // 5
    WholeNumber, // 6
    Integer, // 7
    Angle, // 8
    Color, // 9 #rrggbb
    String,
    Broadcast
    // Variable
    // List
}

#[derive(Debug)]
pub enum StopMode {
    All,
    Myself,
    Other
}

#[derive(Debug)]
pub enum SensingType {
    Loudness,
    Timer
}

#[derive(Debug)]
pub enum KeyCode {
    Any,
    Space,
    UpArrow,
    DownArrow,
    RightArrow,
    LeftArrow,
    // a .. z
}

#[derive(Debug)]
// Variable monitor mode
pub enum MonitorMode {
    Small,
    Large,
    Slider
}

#[derive(Debug)]
// Sprite rotation style
pub enum RotationStyle {
    None,
    LeftRight,
    AllAround,
}

pub type Script = Vec<Block>;

#[derive(Debug)]
// Input slot in a block
pub struct UserInput {
    pub input_type: InputType,
    pub value: Value,
    pub block: Option<Box<Block>>,
}

impl UserInput {
    pub fn new(input_type: InputType, value: Value, block: Option<Box<Block>>) -> Self {
        Self {
            input_type,
            value,
            block
        }
    }
}

impl JsonSerialize for UserInput {
    fn serialize(&self) -> json::Result<JsonValue> {
        let value = match &self.value {
            Value::Number(num) => JsonValue::Number((*num).into()),
            Value::String(s) => JsonValue::String(s.clone()),
            Value::Boolean(b) => JsonValue::Boolean(*b)
        };

        let input = match &self.input_type {
            InputType::Number => json::array![4, value],
            InputType::String => json::array![10, value],
            _ => panic!("unimplemented input type")
        };

        Ok(match &self.block {
            Some(v) => json::array! [3, v.uuid.clone(), input],
            None => json::array![1, input]
        })
    } 
}

// generate a UUID
fn uuid() -> String {
    let mut rng = rand::thread_rng();
    let mut res = String::with_capacity(20);

    for _i in 0..20 {
        res.push(rng.gen_range(33u8..127u8) as char);
    }

    res
}

#[derive(Debug)]
pub enum Opcode {
    Forever(Script),
    Repeat(UserInput, Script),
    If(Box<Block>, Script),
    IfElse(Box<Block>, Script, Script),
    Stop(StopMode),
    Wait(UserInput),
    WaitUntil(Box<Block>),
    RepeatUntil(Box<Block>, Script),
    //control_while
    //control_for_each
    WhenStartAsClone(),
    CreateClone(Box<Block>), CreateCloneMenu(String),
    DeleteThisClone(),
    //control_get_counter
    //control_incr_counter
    //control_clear_counter
    //control_all_at_once

    // add data.js
    // add default_toolbox.js
    //event_whentouchingobject & event_touchingobjectmenu
    WhenGreenFlagClicked(),
    // add event_whenthisspriteclicked(),
    WhenSpriteClicked(),
    WhenStageClicked(),
    WhenBroadcastReceived(String),
    WhenBackdropSwitchesTo(String),
    WhenGreaterThan(SensingType, UserInput),
    Broadcast(Box<Block>), // holds an event_broadcast_menu block by default
    BroadcastAndWait(Box<Block>), // holds an evente_broadcast_menu_block by default
    BroadcastMenu(String),
    WhenKeyPressed(KeyCode),

    // add extensions.js

    Say(UserInput),
    // add looks.js

    MoveSteps(UserInput),
    TurnRight(UserInput),
    TurnLeft(UserInput),
    PointInDirection(UserInput),
    PointTowards(Box<Block>), PointTowardsMenu(String),
    GotoXY(UserInput, UserInput),
    Goto(Box<Block>), GotoMenu(String),
    GlideToXY(UserInput, UserInput, UserInput),
    GlideTo(Box<Block>),
    GlideToMenu(String),
    ChangeX(UserInput),
    SetX(UserInput),
    ChangeY(UserInput),
    SetY(UserInput),
    BounceOnEdge(),
    SetRotationStyle(RotationStyle),
    GetX(),
    GetY(),
    GetDirection()
    // add operators.js
    // add procedures.js
    // add sensing.js
    // add sound.js
    // add vertical_extension.js
}

#[derive(Debug)]
pub struct Block {
    uuid: String,
    opcode: Opcode,
}

impl Block {
    pub fn new(opcode: Opcode) -> Self {
        Self {
            uuid: uuid(),
            opcode
        }
    }
}

impl JsonSerialize for Block {
    fn serialize(&self) -> json::Result<JsonValue> {
        let res = json::object! {
            "next": null,
            "parent": null,
            "inputs": {},
            "fields": {},
            "shadow": false,
            "topLevel": false
        };

        /*
        switch self.opcode {
            Opcode::Say(input) => {

            }
        }*/
        Ok(res)
    }
}

#[derive(Debug)]
pub struct Variable {
    pub id: String,
    pub name: String,
    pub value: Value,
    pub sprite_name: Option<String>,

    // monitor properties
    pub mode: MonitorMode,
    pub x: f64,
    pub y: f64,
    pub visible: bool,
    pub min: f64,
    pub max: f64,
    // is discrete?
}

impl Variable {
    pub fn new(name: &str, value: Value) -> Self {
        Variable {
            id: uuid(),
            name: String::from(name),
            value,
            sprite_name: None,
            mode: MonitorMode::Small,
            x: 0.0,
            y: 0.0,
            visible: false,
            min: 0.0,
            max: 100.0
        }
    }

    pub fn serialize_monitor(&self) -> json::Result<JsonValue> {
        let mode = match self.mode {
            MonitorMode::Small => "default",
            MonitorMode::Large => "large", // TODO idk if this is the right string
            MonitorMode::Slider => "slider", // TODO idk if this is the right string
        };

        let sprite_name: JsonValue = match &self.sprite_name {
            Some(v) => JsonValue::String(v.clone()),
            None => JsonValue::Null,
        };

        let value: JsonValue = match &self.value {
            Value::String(s) => JsonValue::String(s.clone()),
            Value::Number(v) => JsonValue::Number((*v).into()),
            Value::Boolean(v) => JsonValue::Boolean(*v)
        };

        Ok(json::object! {
            "id": uuid(),
            "mode": mode,
            "opcode": "data_variable",
            "params": {
                "VARIABLE": self.name.clone()
            },
            "spriteName": sprite_name,
            "value": value,
            "width": 0,
            "height": 0,
            "x": self.x,
            "y": self.y,
            "visible": self.visible,
            "sliderMin": self.min,
            "sliderMax": self.max,
            "isDiscrete": true
        })
    }
}

impl JsonSerialize for Variable {
    fn serialize(&self) -> json::Result<JsonValue> {
        let val = match &self.value {
            Value::String(s) => JsonValue::String(s.clone()),
            Value::Number(v) => JsonValue::Number((*v).into()),
            Value::Boolean(v) => JsonValue::Boolean(*v)
        };

        Ok(json::array! [
            self.name.clone(), val
        ])
    }
}

#[derive(Debug)]
pub struct List {
    pub id: String,
    pub name: String,
    pub value: Vec<Value>,
    pub sprite_name: Option<String>,

    // monitor properties
    pub x: f64,
    pub y: f64,
    pub visible: bool
}

#[derive(Debug)]
pub struct Costume {
    // id is the MD5 checksum of the file contents
    pub name: String,
    pub path: String,
    pub rot_cx: f64,
    pub rot_cy: f64,
}

#[derive(Debug)]
pub struct Sound {
    // id is the MD5 checksum of the file contents
    pub name: String,
    pub path: String,
}

#[derive(Debug)]
pub struct Data {
    pub vars: Vec<Variable>,
    pub lists: Vec<List>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            vars: Vec::new(),
            lists: Vec::new()
        }
    }
}

#[derive(Debug)]
pub struct Object {
    pub is_stage: bool,
    pub id: String,
    pub name: String,
    pub data: Data,
    pub global_data: Rc<RefCell<Data>>,
    pub scripts: Vec<Script>,

    pub costumes: Vec<Costume>,
    pub sounds: Vec<Sound>,

    pub volume: f64,
    pub costume_index: usize,

    pub layer: usize,
}

impl Object {
    pub fn new(name: &str, global_data: Rc<RefCell<Data>>) -> Self {
        Self {
            id: uuid(),
            is_stage: false,
            name: name.to_string(),
            data: Data::new(),
            global_data,
            scripts: Vec::new(),
            costumes: Vec::new(),
            sounds: Vec::new(),
            volume: 1.0,
            costume_index: 0,
            layer: 0,
        }
    }
}

impl JsonSerialize for Object {
    fn serialize(&self) -> json::Result<JsonValue> {
        let mut vars = JsonValue::new_object();

        if self.is_stage {
            let data = self.global_data.borrow();

            for (_i, var) in data.vars.iter().enumerate() {
                vars[var.id.clone()] = var.serialize()?;
            }
        } else {
            for (_i, var) in self.data.vars.iter().enumerate() {
                vars[var.id.clone()] = var.serialize()?;
            }
        }

        Ok(json::object! {
            "isStage": self.is_stage,
            "name": self.name.clone(),
            "variables": vars,
            "lists": {}, // TODO
            "broadcasts": {}, // TODO
            "blocks": {}, // TODO
            "comments": {}, // TODO
            "currentCostume": self.costume_index,
            "costumes": {}, // TODO
            "sounds": {}, // TODO
            "volume": self.volume * 100.0,
            "layerOrder": self.layer
        })
    }
}

#[derive(Debug)]
pub struct Sprite {
    pub obj: Object,

    pub x: f64,
    pub y: f64,
    pub size: f64,
    pub dir: f64,
    pub draggable: bool,
    pub rotation_style: RotationStyle
}

impl Sprite {
    pub fn new(name: &str, global_data: Rc<RefCell<Data>>) -> Self {
        Self {
            obj: Object::new(name, global_data),
            x: 0.0,
            y: 0.0,
            size: 1.0,
            dir: 90.0,
            draggable: false,
            rotation_style: RotationStyle::AllAround
        }
    }
}

impl JsonSerialize for Sprite {
    fn serialize(&self) -> json::Result<JsonValue> {
        let mut res = self.obj.serialize()?;
        
        res["x"] = self.x.into();
        res["y"] = self.y.into();
        res["size"] = self.size.into();
        res["direction"] = self.dir.into();
        res["draggable"] = JsonValue::Boolean(self.draggable);
        res["rotationStyle"] = match self.rotation_style {
            RotationStyle::None => "don't rotate".into(),
            RotationStyle::LeftRight => "left-right".into(),
            RotationStyle::AllAround => "all around".into()
        };

        Ok(res)
    }
}

#[derive(Debug)]
pub struct Project {
    pub name: String,

    pub stage: Object,
    pub sprites: Vec<Sprite>,

    pub tempo: f64,
    pub video_transparency: f64,
    pub video: bool,

    pub data: Rc<RefCell<Data>>,
}

impl Project {
    pub fn new() -> Self {
        let data = Data {
            vars: Vec::new(),
            lists: Vec::new()
        };

        let cell = Rc::new(RefCell::new(data));
        let mut stage = Object::new("Stage", Rc::clone(&cell));
        stage.is_stage = true;

        Self {
            name: "Project".to_string(),
            stage,
            sprites: Vec::new(),
            data: Rc::clone(&cell),
            tempo: 60.0,
            video_transparency: 0.5,
            video: false,
        }
    }

    pub fn create_sprite(&mut self, name: &str) -> &mut Sprite {
        let mut sprite = Sprite::new(name, Rc::clone(&self.data));
        sprite.obj.layer = self.sprites.len() + 1;

        self.sprites.push(sprite);
        self.sprites.last_mut().unwrap() // return reference to pushed sprite
    }
}

impl JsonSerialize for Project {
    fn serialize(&self) -> json::Result<JsonValue> {
        let mut targets = JsonValue::new_array();
        let mut monitors = JsonValue::new_array();
        
        // serialize stage
        let mut stage = self.stage.serialize()?;

        // add some global properties to the stage json
        stage["tempo"] = self.tempo.into();
        stage["videoTransparency"] = (self.video_transparency * 100.0).into();
        stage["videoState"] = if self.video { "on".into() } else { "off".into() };

        targets.push(stage)?;

        for (_, sprite) in self.sprites.iter().enumerate() {
            targets.push(sprite.serialize()?)?;
            
            // serialize variable monitors
            for (_, var) in sprite.obj.data.vars.iter().enumerate() {
                monitors.push(var.serialize_monitor()?)?;
            }

            // TODO serialize list monitors
            for (_, list) in sprite.obj.data.lists.iter().enumerate() {
                // monitors.push(list.serialize_monitor()?)?;
            }
        }

        // serialize global variable monitors
        for (_i, var) in self.data.borrow().vars.iter().enumerate() {
            monitors.push(var.serialize_monitor()?)?;
        }

        Ok(json::object! {
            "targets": targets,
            "monitors": monitors,
            "extensions": [],
            "meta": {
                "semver": "3.0.0",
                "vm": "0.2.0-prerelease.20201112030151",
                "agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/87.0.4280.88 Safari/537.36"
            }
        })
    }
}