use super::*;

#[derive(Debug)]
pub enum StopOption {
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
pub enum KeyOption {
    Any,
    Space,
    UpArrow,
    DownArrow,
    RightArrow,
    LeftArrow,
    A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y,Z,
    Zero,One,Two,Three,Four,Five,Six,Seven,Eight,Nine
}

#[derive(Debug)]
pub enum GraphicEffect {
    Color,
    Fisheye,
    Whirl,
    Pixelate,
    Moasic,
    Brightness
}

#[derive(Debug)]
pub enum SpriteOption {
    Myself,
    Other(String)
}

#[derive(Debug)]
pub enum FrontBack {
    Front, Back
}

#[derive(Debug)]
pub enum ForwardBack {
    Forward, Back
}

#[derive(Debug)]
pub enum NumberName {
    Number, Nmae
}

#[derive(Debug)]
pub enum PointTarget {
    MousePointer,
    Sprite(String)
}

#[derive(Debug)]
pub enum PositionTarget {
    RandomPosition,
    MousePointer,
    Sprite(String),
}

#[derive(Debug)]
pub enum TouchingOption {
    MousePointer,
    Edge,
    Sprite(String)
}

#[derive(Debug)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64
}

#[derive(Debug)]
pub enum ObjectMenu {
    Stage,
    Sprite(String)
}

#[derive(Debug)]
pub enum ObjectProperty {
    XPosition,
    YPosition,
    Direction,
    CostumeNumber,
    CostumeName,
    Size,
    Volume,
    BackdropNumber,
    BackdropName,
    Data(String)
}

#[derive(Debug)]
pub enum TimeOption {
    Year,
    Month,
    Date,
    DayOfWeek,
    Hour,
    Minute,
    Second
}

#[derive(Debug)]
pub enum SoundEffect {
    Pan,
    Pitch
}

#[derive(Debug)]
pub enum MathOp {
    Abs,
    Floor,
    Ceiling,
    Sqrt,
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Ln,
    Log,
    NaturalExponent,
    Base10Exponent
}

#[derive(Debug)]
pub enum Opcode {
    // Control
    Forever(Script),
    Repeat(UserInput, Script),
    If(Box<Block>, Script),
    IfElse(Box<Block>, Script, Script),
    Stop(StopOption),
    Wait(UserInput),
    WaitUntil(Box<Block>),
    RepeatUntil(Box<Block>, Script),
    // obsolete: control_while
    // obsolete: control_for_each
    StartAsClone(),
    CreateCloneOf(Box<Block>), CreateCloneOfMenu(SpriteOption),
    DeleteThisClone(),
    // obsolete: control_get_counter
    // obsolete: control_incr_counter
    // obsolete: control_clear_counter
    // obsolete: control_all_at_once

    //event_whentouchingobject & event_touchingobjectmenu

    // Data
    Variable(String),
    SetVariableTo(String, UserInput),
    ChangeVariableBy(String, UserInput),
    ShowVariable(String),
    HideVariable(String),
    // ListContents(), // idk what this does
    // ListIndexAll(), // idk what this does
    // ListIndexRandom(), // idk what this does
    AddToList(UserInput, String),
    DeleteOfList(UserInput, String),
    DeleteAllOfList(String),
    InsertAtList(UserInput, UserInput, String),
    ReplaceItemOfList(UserInput, String, UserInput),
    ItemOfList(UserInput, String),
    ItemNumOfList(UserInput, String),
    LengthOfList(String),
    ListContainsItem(String, UserInput),
    ShowList(String),
    HideList(String),

    // Events
    WhenGreenFlagClicked(),
    WhenThisSpriteClicked(),
    WhenStageClicked(),
    WhenBroadcastReceived(String),
    WhenBackdropSwitchesTo(String),
    WhenGreaterThan(SensingType, UserInput),
    Broadcast(Box<Block>), // holds an event_broadcast_menu block by default
    BroadcastAndWait(Box<Block>), // holds an evente_broadcast_menu_block by default
    BroadcastMenu(String),
    WhenKeyPressed(KeyOption),

    // add extensions.js

    // Looks
    SayForSecs(UserInput, UserInput),
    Say(UserInput),
    ThinkForSecs(UserInput, UserInput),
    Think(UserInput),
    Show(),
    Hide(),
    // obsolete: HideAllSprites
    ChangeGraphicEffectBy(GraphicEffect, UserInput),
    SetGraphicEffectTo(GraphicEffect, UserInput),
    ClearGraphicEffects(),
    ChangeSizeBy(UserInput),
    SetSizeTo(UserInput),
    Size(),
    // obsolete: ChangeStretchBy
    // obsolete: SetStretchTo
    SwitchCostumeTo(Box<Block>), Costume(String),
    NextCostume(),
    SwitchBackdropTo(Box<Block>), Backdrops(String),
    GoToFrontBack(FrontBack),
    GoForwardBackwardLayers(ForwardBack),
    BackdropNumberName(NumberName),
    CostumeNumberName(NumberName),
    SwitchBackdropToAndWait(Box<Block>),
    NextBackdrop(),
    
    // Motion
    MoveSteps(UserInput),
    TurnRight(UserInput),
    TurnLeft(UserInput),
    PointInDirection(UserInput),
    PointTowards(Box<Block>), PointTowardsMenu(PointTarget),
    GoToXY(UserInput, UserInput),
    GoTo(Box<Block>), GoToMenu(PositionTarget),
    GlideSecsToXY(UserInput, UserInput, UserInput),
    GlideToMenu(PositionTarget),
    GlideTo(UserInput, Box<Block>),
    ChangeXBy(UserInput),
    SetX(UserInput),
    ChangeYBy(UserInput),
    SetY(UserInput),
    IfOnEdgeBounce(),
    SetRotationStyle(RotationStyle),
    XPosition(),
    YPosition(),
    Direction(),
    // obsolete: ScrollRight
    // obsolete: ScrollUp
    // obsolete: AlignScene
    // obsolete: XScroll
    // obsolete: YScroll

    // Operators
    Add(UserInput, UserInput),
    Subtract(UserInput, UserInput),
    Multiply(UserInput, UserInput),
    Divide(UserInput, UserInput),
    Random(UserInput, UserInput),
    Lt(UserInput, UserInput),
    Equals(UserInput, UserInput),
    Gt(UserInput, UserInput),
    And(Box<Block>, Box<Block>),
    Or(Box<Block>, Box<Block>),
    Not(Box<Block>),
    Join(UserInput, UserInput),
    LetterOf(UserInput, UserInput),
    Length(UserInput),
    Contains(UserInput, UserInput),
    Mod(UserInput, UserInput),
    Round(UserInput),
    MathOp(MathOp, UserInput),
    
    // Procedures
    // TODO how do custom procedures work?
    ProceduresDefinition(),
    ProceduresCall(),
    ProceduresPrototype(),
    ProceduresDeclaration(),
    ArgumentReporterBoolean(),
    ArgumentReporterStringNumber(),
    ArgumentEditorBoolean(),
    ArgumentEditorStringNumber(),

    // Sensing
    TouchingObject(Box<Block>),
    TouchingObjectMenu(TouchingOption),
    TouchingColor(Color),
    ColorIsTouchingColor(Color, Color),
    DistanceTo(Box<Block>),
    DistanceToMenu(String),
    AskAndWait(UserInput),
    Answer(),
    KeyPressed(Box<Block>),
    KeyOptions(KeyOption),
    MouseDown(),
    MouseX(),
    MouseY(),
    SetDragMode(bool),
    Loudness(),
    // obsolete: Loud
    Timer(),
    ResetTimer(),
    OfObjectMenu(ObjectMenu),
    Of(ObjectProperty, Box<Block>),
    Current(TimeOption),
    DaysSince2000(),
    Username(),
    // obsolete: UserID

    // Sound
    SoundsMenu(String),
    Play(Box<Block>),
    PlayUntilDone(Box<Block>),
    StopAllSounds(),
    SetSoundEffectTo(SoundEffect, UserInput),
    ChangeSoundEffectBy(SoundEffect, UserInput),
    ClearSoundEffects(),
    ChangeVolumeBy(UserInput),
    SetVolumeTo(UserInput),
    Volume(),

    // Pen
    PenDown(),
    PenUp(),
    // TODO more pen blocks (where are they?)

    // Music
    MusicDrum(),
    MusicPlayNote()
    // TODO more music blocks (where are they?)
}

impl JsonSerialize for Opcode {
    fn serialize(&self) -> Result<JsonValue, SerializeError> {
        // this function is a big todo, so disable warnings on unused variables for now
        #![allow(unused_variables)]

        let mut res = JsonValue::new_object();

        let opcode_str: &str;
        let mut inputs = JsonValue::new_object();
        let mut fields = JsonValue::new_object();

        match self {
            // CONTROL //
            Self::Forever(substack) => {
                opcode_str = "control_forever";
                inputs["SUBSTACK"] = substack.serialize()?;
            },

            Self::Repeat(times, substack) => {
                opcode_str = "control_repeat";
                inputs["TIMES"] = times.serialize(InputType::WholeNumber)?;
                inputs["SUBSTACK"] = substack.serialize()?;
            },

            Self::If(condition, substack) => {
                opcode_str = "control_if";
                inputs["CONDITION"] = condition.serialize()?;
                inputs["SUBSTACK"] = substack.serialize()?;
            },

            Self::IfElse(condition, substack, substack2) => {
                opcode_str = "control_if_else";
                inputs["CONDITION"] = condition.serialize()?;
                inputs["SUBSTACK"] = substack.serialize()?;
            },

            Self::Stop(option) => { // TODO
                panic!("control_stop unimplemented");
            },

            Self::Wait(duration) => {
                opcode_str = "control_wait";
                inputs["DURATION"] = duration.serialize(InputType::PositiveNumber)?;
            },

            Self::WaitUntil(condition) => {
                opcode_str = "control_wait_until";
                inputs["CONDITION"] = condition.serialize()?;
            },

            Self::RepeatUntil(condition, substack) => {
                opcode_str = "control_repeat_until";
                inputs["CONDITION"] = condition.serialize()?;
                inputs["SUBSTACK"] = substack.serialize()?;
            },

            // obsolete: control_while
            // obsolete: control_for_each
            
            Self::StartAsClone() => {
                opcode_str = "control_start_as_clone";
            },

            Self::CreateCloneOfMenu(clone_option) => { // TODO
                todo!("control_createcloneof_menu");
            },

            Self::CreateCloneOf(clone_option) => { // TODO
                todo!("control_createcloneof");
            },

            Self::DeleteThisClone() => {
                opcode_str = "control_delete_this_clone";
            },

            // obsolete: control_get_counter
            // obsolete: control_incr_counter
            // obsolete: control_clear_counter
            // obsolete: control_clear_counter
            // obsolete: control_all_at_once



            // DATA //
            Self::Variable(variable) => { // TODO
                todo!("data_variable");
            },

            Self::SetVariableTo(variable, value) => { // TODO
                todo!("data_setvariableto");
            },

            Self::ChangeVariableBy(variable, value) => { // TODO
                todo!("data_changevariableby");
            },

            Self::ShowVariable(variable) => { // TODO
                todo!("data_showvariable");
            },

            Self::HideVariable(variable) => { // TODO
                todo!("data_hidevariable");
            },

            Self::AddToList(item, list) => { // TODO
                todo!("data_addtolist");
            },

            Self::DeleteOfList(index, list) => { // TODO
                todo!("data_deleteoflist");
            },

            Self::DeleteAllOfList(list) => { // TODO
                todo!("data_deletealloflist");
            },

            Self::InsertAtList(item, index, list) => { // TODO
                todo!("data_insertatlist");
            },

            Self::ReplaceItemOfList(index, list, item) => {
                todo!("data_replaceitemoflist");
            },

            Self::ItemOfList(index, list) => {
                todo!("data_itemoflist");
            },

            Self::ItemNumOfList(item, list) => {
                todo!("data_itemnumoflist");
            },

            Self::LengthOfList(list) => {
                todo!("data_lengthoflist");
            },

            Self::ListContainsItem(list, item) => {
                todo!("data_listcontainsitem");
            },

            Self::ShowList(list) => {
                todo!("data_showlist");
            },

            Self::HideList(list) => {
                todo!("data_hidelist");
            },

            // EVENT //
            // TODO event_whentouchingobject
            // TODO touchingobjectmenu
            Self::WhenGreenFlagClicked() => {
                opcode_str = "event_whenflagclicked";
            },

            Self::WhenThisSpriteClicked() => {
                opcode_str = "event_whenthisspriteclicked";
            },

            Self::WhenStageClicked() => {
                opcode_str = "event_whenstageclicked";
            },

            Self::WhenBroadcastReceived(broadcast_option) => {
                todo!("event_whenbroadcastreceived");
            },

            Self::WhenBackdropSwitchesTo(backdrop) => {
                todo!("event_whenbackdropswitchesto");
            },

            Self::WhenGreaterThan(when_greater_than_menu, value) => {
                todo!("event_whengreaterthan");
            },

            Self::BroadcastMenu(broadcast_option) => {
                todo!("event_broadcast_menu");
            },

            Self::Broadcast(broadcast_input) => {
                todo!("event_broadcast");
            },

            Self::BroadcastAndWait(broadcast_input) => {
                todo!("event_broadcastandwait");
            },

            Self::WhenKeyPressed(key_option) => {
                todo!("event_whenkeypressed");
            },


            // LOOKS //
            Self::SayForSecs(message, secs) => {
                opcode_str = "looks_sayforsecs";
                inputs["MESSAGE"] = message.serialize(InputType::String)?;
                inputs["SECS"] = message.serialize(InputType::PositiveNumber)?;
            },

            Self::Say(message) => {
                opcode_str = "looks_say";
                inputs["MESSAGE"] = message.serialize(InputType::String)?;
            },

            Self::ThinkForSecs(message, secs) => {
                opcode_str = "looks_thinkforsecs";
                inputs["MESSAGE"] = message.serialize(InputType::String)?;
                inputs["SECS"] = secs.serialize(InputType::PositiveNumber)?;
            },

            Self::Think(message) => {
                opcode_str = "looks_think";
                inputs["THINK"] = message.serialize(InputType::String)?;
            },

            Self::Show() => {
                opcode_str = "looks_show";
            },

            Self::Hide() => {
                opcode_str = "looks_hide";
            },

            // obsolete: looks_hideallsprites
            
            Self::ChangeGraphicEffectBy(effect, change) => {
                opcode_str = "looks_changeffectby";
                todo!("looks_changeeffectby");
            },

            Self::SetGraphicEffectTo(effect, change) => {
                todo!("looks_seteffectto");
            },

            Self::ClearGraphicEffects() => {
                opcode_str = "looks_cleargraphiceffects";
            },

            Self::ChangeSizeBy(change) => {
                opcode_str = "looks_changesizeby";
                inputs["CHANGE"] = change.serialize(InputType::Number)?;
            },

            Self::SetSizeTo(size) => {
                opcode_str = "looks_setsizeto";
                inputs["SIZE"] = size.serialize(InputType::Number)?;
            },

            Self::Size() => {
                opcode_str = "looks_size";
            },

            // obsolete: looks_changestretchby
            // obsolete: looks_setstretchto

            Self::Costume(costume) => {
                todo!("looks_costume");
            },

            Self::SwitchCostumeTo(costume) => {
                todo!("looks_switchcostumeto");
            },

            Self::NextCostume() => {
                opcode_str = "looks_nextcostume";
            },

            Self::SwitchBackdropTo(backdrop) => {
                todo!("looks_switchbackdropto");
            },

            Self::Backdrops(backdrop) => {
                todo!("looks_backdrops");
            },

            Self::GoToFrontBack(front_back) => {
                todo!("looks_gotofrontback");
            },

            Self::GoForwardBackwardLayers(forward_backward) => {
                todo!("looks_goforwardbackwardlayers");
            },

            Self::BackdropNumberName(number_name) => {
                todo!("looks_backdropnumbername");
            },

            Self::CostumeNumberName(number_name) => {
                todo!("looks_costumenumbername");
            },

            Self::SwitchBackdropToAndWait(backdrop) => {
                todo!("looks_switchbackdroptoandwait");
            },

            Self::NextBackdrop() => {
                opcode_str = "looks_nextbackdrop";
            },


            // MOTION //

            Self::MoveSteps(steps) => {
                opcode_str = "motion_movesteps";
                inputs["STEPS"] = steps.serialize(InputType::Number)?;
            },
            
            Self::TurnRight(degrees) => {
                opcode_str = "motion_turnright";
                inputs["DEGREES"] = degrees.serialize(InputType::Number)?;
            },

            Self::TurnLeft(degrees) => {
                opcode_str = "motion_turnleft";
                inputs["DEGREES"] = degrees.serialize(InputType::Number)?;
            },

            Self::PointInDirection(direction) => {
                opcode_str = "motion_pointindirection";
                inputs["DIRECTION"] = direction.serialize(InputType::Angle)?;
            },

            Self::PointTowardsMenu(towards) => {
                todo!("motion_pointtowards_menu");
            },

            Self::PointTowards(towards) => {
                todo!("motion_pointtowards");
            },

            Self::GoToMenu(to) => {
                todo!("motion_goto_menu");
            },

            Self::GoToXY(x, y) => {
                opcode_str = "motion_gotoxy";
                inputs["X"] = x.serialize(InputType::Number)?;
                inputs["Y"] = y.serialize(InputType::Number)?;
            },

            Self::GoTo(to) => {
                todo!("motion_goto");
            },

            Self::GlideSecsToXY(secs, x, y) => {
                opcode_str = "motion_glidesecstoxy";
                inputs["SECS"] = secs.serialize(InputType::PositiveNumber)?;
                inputs["X"] = x.serialize(InputType::Number)?;
                inputs["Y"] = y.serialize(InputType::Number)?;
            },

            Self::GlideToMenu(to) => {
                todo!("motion_glidetomenu");
            },

            Self::GlideTo(secs, to) => {
                todo!("motion_glideto");
            },

            Self::ChangeXBy(dx) => {
                opcode_str = "motion_changexby";
                inputs["DX"] = dx.serialize(InputType::Number)?;
            },

            Self::SetX(x) => {
                opcode_str = "motion_setx";
                inputs["X"] = x.serialize(InputType::Number)?;
            },

            Self::ChangeYBy(dy) => {
                opcode_str = "motion_changeyby";
                inputs["DY"] = dy.serialize(InputType::Number)?;
            },

            Self::SetY(y) => {
                opcode_str = "motion_sety";
                inputs["Y"] = y.serialize(InputType::Number)?;
            },

            Self::IfOnEdgeBounce() => {
                opcode_str = "motion_ifonedgebounce";
            },

            Self::SetRotationStyle(style) => {
                todo!("looks_setrotationstyle");
            },

            Self::XPosition() => {
                opcode_str = "motion_xposition";
            },

            Self::YPosition() => {
                opcode_str = "motion_yposition";
            },

            Self::Direction() => {
                opcode_str = "motion_direction";
            },


            // Operators //
            Self::Add(num1, num2) => {
                opcode_str = "operator_add";
                inputs["NUM1"] = num1.serialize(InputType::Number)?;
                inputs["NUM2"] = num2.serialize(InputType::Number)?;
            },

            Self::Subtract(num1, num2) => {
                opcode_str = "operator_add";
                inputs["NUM1"] = num1.serialize(InputType::Number)?;
                inputs["NUM2"] = num2.serialize(InputType::Number)?;
            },

            Self::Multiply(num1, num2) => {
                opcode_str = "operator_multiply";
                inputs["NUM1"] = num1.serialize(InputType::Number)?;
                inputs["NUM2"] = num2.serialize(InputType::Number)?;
            },

            Self::Divide(num1, num2) => {
                opcode_str = "operator_divide";
                inputs["NUM1"] = num1.serialize(InputType::Number)?;
                inputs["NUM2"] = num2.serialize(InputType::Number)?;
            },

            Self::Random(from, to) => {
                opcode_str = "operator_random";
                inputs["FROM"] = from.serialize(InputType::Number)?;
                inputs["TO"] = to.serialize(InputType::Number)?;
            },

            Self::Lt(operand1, operand2) => {
                opcode_str = "operator_lt";
                inputs["OPERAND1"] = operand1.serialize(InputType::Number)?;
                inputs["OPERAND2"] = operand2.serialize(InputType::Number)?;
            },

            Self::Equals(operand1, operand2) => {
                opcode_str = "operator_equals";
                inputs["OPERAND1"] = operand1.serialize(InputType::Number)?;
                inputs["OPERAND2"] = operand2.serialize(InputType::Number)?;
            },

            Self::Gt(operand1, operand2) => {
                opcode_str = "operator_gt";
                inputs["OPERAND1"] = operand1.serialize(InputType::Number)?;
                inputs["OPERAND2"] = operand2.serialize(InputType::Number)?;
            },

            Self::And(operand1, operand2) => {
                todo!("operator_and");
            },

            Self::Or(operand1, operand2) => {
                todo!("operator_or");
            },

            Self::Not(operand) => {
                todo!("operator_not");
            },

            Self::Join(string1, string2) => {
                opcode_str = "operator_join";
                inputs["STRING1"] = string1.serialize(InputType::Number)?;
                inputs["STRING2"] = string2.serialize(InputType::Number)?;
            },

            Self::LetterOf(letter, string) => {
                opcode_str = "operator_letter_of";
                inputs["LETTER"] = letter.serialize(InputType::WholeNumber)?;
                inputs["STRING"] = string.serialize(InputType::String)?;
            },

            Self::Length(string) => {
                opcode_str = "operator_length";
                inputs["STRING"] = string.serialize(InputType::String)?;
            },

            Self::Contains(string1, string2) => {
                opcode_str = "operator_contains";
                inputs["STRING1"] = string1.serialize(InputType::String)?;
                inputs["STRING2"] = string2.serialize(InputType::String)?;
            },

            Self::Mod(num1, num2) => {
                opcode_str = "operator_mod";
                inputs["NUM1"] = num1.serialize(InputType::Number)?;
                inputs["NUM2"] = num2.serialize(InputType::Number)?;
            },

            Self::Round(num) => {
                opcode_str = "operator_round";
                inputs["NUM"] = num.serialize(InputType::Number)?;
            },

            Self::MathOp(operator, num) => {
                todo!("operator_mathop");
            },

            // TODO how do custom procedures work?

            // Sensing //
            Self::TouchingObject(touchingobjectmenu) => {
                todo!("sensing_touchingobject");
            },

            Self::TouchingObjectMenu(touchingobjectmenu) => {
                todo!("sensing_touchingobjectmenu");
            },

            Self::TouchingColor(color) => {
                todo!("sensing_touchingcolor");
            },

            Self::ColorIsTouchingColor(color, color2) => {
                todo!("sensing_coloristouchingcolor");
            },

            Self::DistanceTo(distancetomenu) => {
                todo!("sensing_distanceto");
            },

            Self::DistanceToMenu(distancetomenu) => {
                todo!("sensing_distancetomenu");
            },

            Self::AskAndWait(question) => {
                opcode_str = "sensing_askandwait";
                inputs["QUESTION"] = question.serialize(InputType::String)?;
            },

            Self::Answer() => {
                opcode_str = "sensing_answer";
            },

            Self::KeyPressed(key_option) => {
                todo!("sensing_keypressed");
            },

            Self::KeyOptions(key_option) => {
                todo!("sensing_keyoptions");
            },

            Self::MouseDown() => {
                opcode_str = "sensing_mousedown";
            },

            Self::MouseX() => {
                opcode_str = "sensing_mousex";
            },

            Self::MouseY() => {
                opcode_str = "sensing_mousey";
            },

            Self::SetDragMode(drag_mode) => {
                todo!("sensing_setdragmode");
            },

            Self::Loudness() => {
                opcode_str = "sensing_loudness";
            },

            // obsolete: sensing_loud

            Self::Timer() => {
                opcode_str = "sensing_timer";
            },

            Self::OfObjectMenu(object) => {
                todo!("sensing_of_object_menu");
            },

            Self::Of(property, object) => {
                todo!("sensing_of");
            },

            Self::Current(currentmenu) => {
                todo!("sensing_current");
            },

            Self::DaysSince2000() => {
                opcode_str = "sensing_dayssince2000";
            },

            Self::Username() => {
                opcode_str = "sensing_username";
            },

            // obsolete: sensing_userid

            // Sound //
            // TODO what is sound_sounds_menu?

            Self::SoundsMenu(sound) => {
                todo!("sound_sounds_menu");
            },

            Self::Play(sound_menu) => {
                todo!("sound_play");
            },

            Self::PlayUntilDone(sound_menu) => {
                todo!("sound_playuntilmode");
            },

            Self::StopAllSounds() => {
                opcode_str = "sound_stopallsounds";
            },

            Self::SetSoundEffectTo(effect, value) => {
                todo!("sound_seteffectto");
            },

            Self::ChangeSoundEffectBy(effect, value) => {
                todo!("sound_changeeffectby");
            },

            Self::ClearSoundEffects() => {
                opcode_str = "sound_cleareffects";
            },

            Self::ChangeVolumeBy(volume) => {
                opcode_str = "sound_changevolumeby";
                inputs["VOLUME"] = volume.serialize(InputType::Number)?;
            },

            Self::SetVolumeTo(volume) => {
                opcode_str = "sound_setvolumeto";
                inputs["VOLUME"] = volume.serialize(InputType::Number)?;
            },

            Self::Volume() => {
                opcode_str = "sound_volume";
            },



            _ => {
                panic!("unimplemented opcode");
            }
        }

        res["opcode"] = JsonValue::String(opcode_str.to_string());
        res["inputs"] = inputs;
        res["fields"] = fields;

        Ok(res)
    }
}