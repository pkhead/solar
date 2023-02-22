use std::io::Write;
use std::collections::HashSet;
use std::{rc::Rc, fs::File};
use std::cell::RefCell;
use rand::Rng;
use std::fs;
use std::path::Path;
use json::{JsonValue};
pub use opcode::*;

mod opcode;

#[derive(Debug)]
pub enum SerializeError {
    JsonError(json::Error),
    IoError(std::io::Error),
    ZipError(zip::result::ZipError),
    NoHash,
    NoCostume,
    Unknown
}

// compatibility between json::Error and SerializeError
impl From<json::Error> for SerializeError {
    fn from(error: json::Error) -> Self {
        SerializeError::JsonError(error)
    }
}

// convert io::Error to SerializeError
impl From<std::io::Error> for SerializeError {
    fn from(value: std::io::Error) -> Self {
        SerializeError::IoError(value)
    }
}

// convert ZipErrors
impl From<zip::result::ZipError> for SerializeError {
    fn from(value: zip::result::ZipError) -> Self {
        SerializeError::ZipError(value)
    }
}

impl std::fmt::Display for SerializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SerializeError::JsonError(e) => write!(f, "{}", e),
            SerializeError::IoError(e) => write!(f, "{}", e),
            SerializeError::NoHash => write!(f, "hash not generated"),
            SerializeError::Unknown => write!(f, "unknown error"),
            SerializeError::NoCostume => write!(f, "no costume"),
            SerializeError::ZipError(e) => write!(f, "{}", e)
        }
    }
}

pub trait JsonSerialize {
    fn serialize(&self) -> Result<JsonValue, SerializeError>;
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

#[derive(Debug)]
pub struct Script {
    blocks: Vec<Block>
}

impl Script {
    pub fn new() -> Self {
        Self {
            blocks: Vec::new()
        }
    }

    pub fn push(&mut self, block: Block) {
        self.blocks.push(block);
    }

    /// Parses all the block in this script to JSON, adding it to the `block_list` JsonValue::Array.
    /// 
    /// On OK, it returns the UUID of the first block.
    pub fn serialize(&self, block_list: &mut JsonValue, root: Option<&String>) -> Result<&String, SerializeError> {
        let mut parent_uuid: Option<&String> = root;

        for (_, block) in self.blocks.iter().enumerate() {
            let mut res = block.serialize(block_list, parent_uuid)?;

            // set parent's "next" property to my uuid
            // and set my "parent" property to parent's uuid
            if let Some(parent_uuid) = parent_uuid {
                block_list[parent_uuid]["next"] = JsonValue::String(block.uuid.clone());
                res["parent"] = JsonValue::String(parent_uuid.clone());
                res["topLevel"] = JsonValue::Boolean(false);
            } else {
                // no parent, set x and y
                res["x"] = 0.into();
                res["y"] = 0.into();
            }
            
            block_list[block.uuid.clone()] = res;
            parent_uuid = Some(&block.uuid);
        }

        match self.blocks.first() {
            Some(v) => Ok(&v.uuid),
            None => Err(SerializeError::Unknown)
        }
    }
}

#[derive(Debug)]
// Input slot in a block
pub struct UserInput {
    pub value: Value,
    pub block: Option<Box<Block>>,
}

impl UserInput {
    pub fn new(value: Value, block: Option<Box<Block>>) -> Self {
        Self {
            value,
            block
        }
    }

    pub fn serialize(&self, input_type: InputType) -> Result<JsonValue, SerializeError> {
        let value = match &self.value {
            Value::Number(num) => JsonValue::String((*num).to_string().clone()),
            Value::String(s) => JsonValue::String(s.clone()),
            Value::Boolean(b) => JsonValue::Boolean(*b)
        };
    
        let input = match input_type {
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

#[derive(Debug)]
pub struct ScriptInput {
    pub script: Option<Script>
}

impl ScriptInput {
    pub fn serialize(&self, block_list: &mut JsonValue/*::Array*/, parent_uuid: &String) -> Result<JsonValue, SerializeError> {
        match &self.script {
            Some(script) => {
                // this is the uuid of the first block
                let uuid = script.serialize(block_list, Some(parent_uuid))?;
                Ok(json::array![2, uuid.clone()])
            },

            None => Ok(json::array![1, json::Null])
        }
    }
}

#[derive(Debug)]
pub struct ReporterInput {
    pub block: Option<Box<Block>>
}

impl ReporterInput {
    pub fn serialize(&self, block_list: &mut JsonValue/*::Array*/, parent_uuid: &String) -> Result<JsonValue, SerializeError> {
        match &self.block {
            Some(block) => {
                block_list[block.uuid.clone()] = block.serialize(block_list, Some(parent_uuid))?;
                Ok(json::array![2, block.uuid.clone()])
            }

            None => Ok(json::array![1, json::Null])
        }
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
    fn serialize(&self) -> Result<JsonValue, SerializeError> {
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
            id: uuid() + name,
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

    pub fn serialize_monitor(&self) -> Result<JsonValue, SerializeError> {
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
    fn serialize(&self) -> Result<JsonValue, SerializeError> {
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

    pub md5: Option<String>
}

impl Costume {
    pub fn new(name: &str, path: &str, rot_cx: f64, rot_cy: f64) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_string(),
            rot_cx,
            rot_cy,
            md5: None
        }
    }

    pub fn read_file(&self) -> Result<Vec<u8>, std::io::Error> {
        fs::read(&self.path)
    }
}

impl JsonSerialize for Costume {
    fn serialize(&self) -> Result<JsonValue, SerializeError> {
        // the file id will be the md5 checksum of the file contents
        match &self.md5 {
            Some(hash) => {
                match Path::new(&self.path).extension().and_then(std::ffi::OsStr::to_str) {
                    Some(ext) => {
                        Ok(json::object! {
                            "name": self.name.clone(),
                            //"bitmapResolution": 1, // TODO i think it's 2
                            "dataFormat": ext.clone(),
                            "assetId": hash.clone(),
                            "md5ext": format!("{}.{}", hash, &ext),
                            "rotationCenterX": self.rot_cx,
                            "rotationCenterY": self.rot_cy
                        })
                    },
                    None => {
                        Err(SerializeError::Unknown)
                    }
                }
            },
            None => Err(SerializeError::NoHash) 
        }
    }
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

    pub layer: usize
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
            layer: 0
        }
    }

    pub fn add_costume(&mut self, costume: Costume) {
        self.costumes.push(costume);
    }
}

impl JsonSerialize for Object {
    fn serialize(&self) -> Result<JsonValue, SerializeError> {
        let mut vars = JsonValue::new_object();
        let mut costumes = JsonValue::new_array();

        // serialize data
        // if is stage, store the global data in this
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

        // serialize costumes
        if self.costumes.len() == 0 {
            return Err(SerializeError::NoCostume);
        }

        for (_, costume) in self.costumes.iter().enumerate() {
            let json = costume.serialize()?;
            costumes.push(json)?;
        }

        let mut block_list = JsonValue::new_object();

        // serialize scripts
        for (_, script) in self.scripts.iter().enumerate() {
            script.serialize(&mut block_list, None)?;
        }

        Ok(json::object! {
            "isStage": self.is_stage,
            "name": self.name.clone(),
            "variables": vars,
            "lists": {}, // TODO
            "broadcasts": {}, // TODO
            "blocks": block_list,
            "comments": {},
            "currentCostume": self.costume_index,
            "costumes": costumes, // TODO
            "sounds": [], // TODO
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
    pub rotation_style: RotationStyle,
    pub visible: bool
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
            rotation_style: RotationStyle::AllAround,
            visible: true
        }
    }
}

impl JsonSerialize for Sprite {
    fn serialize(&self) -> Result<JsonValue, SerializeError> {
        let mut res = self.obj.serialize()?;
        
        res["visible"] = JsonValue::Boolean(self.visible);
        res["x"] = self.x.into();
        res["y"] = self.y.into();
        res["size"] = (self.size * 100.0).into();
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
/// The root of a Scratch project
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
            video: true,
        }
    }

    pub fn create_sprite(&mut self, name: &str) -> &mut Sprite {
        let mut sprite = Sprite::new(name, Rc::clone(&self.data));
        sprite.obj.layer = self.sprites.len() + 1;

        self.sprites.push(sprite);
        self.sprites.last_mut().unwrap() // return reference to pushed sprite
    }

    // TODO remove_sprite

    pub fn save(&mut self, path_str: &str) -> Result<(), SerializeError> {
        // begin zip file
        let file = File::create(path_str)?;
        let mut zip = zip::ZipWriter::new(file);

        let options = zip::write::FileOptions::default();

        let mut assets_visited: HashSet<String> = HashSet::new();

        let mut write_costume = |costume: &mut Costume| -> Result<(), SerializeError> {
            // get file contents and calculate md5 hash
            let file_contents = costume.read_file()?;
            let md5 = format!("{:x}", md5::compute(&file_contents));

            if !assets_visited.contains(&md5) {
                assets_visited.insert(md5.clone());

                let path = Path::new(&costume.path);

                // extract file extension
                let ext = match path.extension().and_then(std::ffi::OsStr::to_str) {
                    Some(v) => String::from(".") + v,
                    None => String::from(".")
                };

                
                // begin writing file to zip
                zip.start_file(md5.clone() + &ext, options)?;
                zip.write_all(&file_contents)?;
            }
            
            costume.md5 = Some(md5);
            Ok(())
        };

        // begin writing asset files
        for (_, costume) in self.stage.costumes.iter_mut().enumerate() {
            write_costume(costume)?;
        }

        for (_, sprite) in self.sprites.iter_mut().enumerate() {
            for (_, costume) in sprite.obj.costumes.iter_mut().enumerate() {
                write_costume(costume)?;
            }
        }

        // begin writing project data
        let json_val = self.serialize()?;
        let json = json::stringify(json_val);
        zip.start_file("project.json", options)?;
        zip.write_all(json.as_bytes())?;

        zip.finish()?;

        Ok(())
    }
}

impl JsonSerialize for Project {
    fn serialize(&self) -> Result<JsonValue, SerializeError> {
        let mut targets = JsonValue::new_array();
        let mut monitors = JsonValue::new_array();
        
        // serialize stage
        let mut stage = self.stage.serialize()?;

        // add some global properties to the stage json
        stage["tempo"] = self.tempo.into();
        stage["videoTransparency"] = (self.video_transparency * 100.0).into();
        stage["videoState"] = if self.video { "on".into() } else { "off".into() };
        stage["textToSpeechLanguage"] = json::Null;

        targets.push(stage)?;

        for (_, sprite) in self.sprites.iter().enumerate() {
            targets.push(sprite.serialize()?)?;
            
            // serialize variable monitors
            for (_, var) in sprite.obj.data.vars.iter().enumerate() {
                if var.visible {
                    monitors.push(var.serialize_monitor()?)?;
                }
            }

            // TODO serialize list monitors
            // for (_, list) in sprite.obj.data.lists.iter().enumerate() {
            //     monitors.push(list.serialize_monitor()?)?;
            // }
        }

        // serialize global variable monitors
        for (_i, var) in self.data.borrow().vars.iter().enumerate() {
            if var.visible {
                monitors.push(var.serialize_monitor()?)?;
            }
        }

        Ok(json::object! {
            "targets": targets,
            "monitors": monitors,
            "extensions": [],
            "meta": {
                "semver": "3.0.0",
                "vm": "1.3.18",
                // TODO different user-agent
                "agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36 Edg/110.0.1587.46"
            }
        })
    }
}