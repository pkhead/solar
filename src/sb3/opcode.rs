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
    Forever(ScriptInput),
    Repeat(UserInput, ScriptInput),
    If(ReporterInput, ScriptInput),
    IfElse(ReporterInput, ScriptInput, ScriptInput),
    Stop(StopOption),
    Wait(UserInput),
    WaitUntil(ReporterInput),
    RepeatUntil(ReporterInput, ScriptInput),
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

impl Block {
    pub fn serialize(&self, block_list: &mut JsonValue, parent_uuid: Option<&String>) -> Result<JsonValue, SerializeError> {
        // this function is a big todo, so disable warnings on unused variables for now
        #![allow(unused_variables)]

        let mut res = JsonValue::new_object();

        let opcode_str: &str;
        let mut inputs = JsonValue::new_object();
        let fields = JsonValue::new_object();

        match &self.opcode {
            // CONTROL //
            Opcode::Forever(substack) => {
                opcode_str = "control_forever";
                inputs["SUBSTACK"] = substack.serialize(block_list, &self.uuid)?;
            },

            Opcode::Repeat(times, substack) => {
                opcode_str = "control_repeat";
                inputs["TIMES"] = times.serialize(InputType::WholeNumber)?;
                inputs["SUBSTACK"] = substack.serialize(block_list, &self.uuid)?;
            },

            Opcode::If(condition, substack) => {
                opcode_str = "control_if";
                inputs["CONDITION"] = condition.serialize(block_list, &self.uuid)?;
                inputs["SUBSTACK"] = substack.serialize(block_list, &self.uuid)?;
            },

            Opcode::IfElse(condition, substack, substack2) => {
                opcode_str = "control_if_else";
                inputs["CONDITION"] = condition.serialize(block_list, &self.uuid)?;
                inputs["SUBSTACK"] = substack.serialize(block_list, &self.uuid)?;
                inputs["SUBSTACK2"] = substack2.serialize(block_list, &self.uuid)?;
            },

            Opcode::Stop(option) => { // TODO
                panic!("control_stop unimplemented");
            },

            Opcode::Wait(duration) => {
                opcode_str = "control_wait";
                inputs["DURATION"] = duration.serialize(InputType::PositiveNumber)?;
            },

            Opcode::WaitUntil(condition) => {
                opcode_str = "control_wait_until";
                inputs["CONDITION"] = condition.serialize(block_list, &self.uuid)?;
            },

            Opcode::RepeatUntil(condition, substack) => {
                opcode_str = "control_repeat_until";
                inputs["CONDITION"] = condition.serialize(block_list, &self.uuid)?;
                inputs["SUBSTACK"] = substack.serialize(block_list, &self.uuid)?;
            },

            // obsolete: control_while
            // obsolete: control_for_each
            
            Opcode::StartAsClone() => {
                opcode_str = "control_start_as_clone";
            },

            Opcode::CreateCloneOfMenu(clone_option) => { // TODO
                todo!("control_createcloneof_menu");
            },

            Opcode::CreateCloneOf(clone_option) => { // TODO
                todo!("control_createcloneof");
            },

            Opcode::DeleteThisClone() => {
                opcode_str = "control_delete_this_clone";
            },

            // obsolete: control_get_counter
            // obsolete: control_incr_counter
            // obsolete: control_clear_counter
            // obsolete: control_clear_counter
            // obsolete: control_all_at_once



            // DATA //
            Opcode::Variable(variable) => { // TODO
                todo!("data_variable");
            },

            Opcode::SetVariableTo(variable, value) => { // TODO
                todo!("data_setvariableto");
            },

            Opcode::ChangeVariableBy(variable, value) => { // TODO
                todo!("data_changevariableby");
            },

            Opcode::ShowVariable(variable) => { // TODO
                todo!("data_showvariable");
            },

            Opcode::HideVariable(variable) => { // TODO
                todo!("data_hidevariable");
            },

            Opcode::AddToList(item, list) => { // TODO
                todo!("data_addtolist");
            },

            Opcode::DeleteOfList(index, list) => { // TODO
                todo!("data_deleteoflist");
            },

            Opcode::DeleteAllOfList(list) => { // TODO
                todo!("data_deletealloflist");
            },

            Opcode::InsertAtList(item, index, list) => { // TODO
                todo!("data_insertatlist");
            },

            Opcode::ReplaceItemOfList(index, list, item) => {
                todo!("data_replaceitemoflist");
            },

            Opcode::ItemOfList(index, list) => {
                todo!("data_itemoflist");
            },

            Opcode::ItemNumOfList(item, list) => {
                todo!("data_itemnumoflist");
            },

            Opcode::LengthOfList(list) => {
                todo!("data_lengthoflist");
            },

            Opcode::ListContainsItem(list, item) => {
                todo!("data_listcontainsitem");
            },

            Opcode::ShowList(list) => {
                todo!("data_showlist");
            },

            Opcode::HideList(list) => {
                todo!("data_hidelist");
            },

            // EVENT //
            // TODO event_whentouchingobject
            // TODO touchingobjectmenu
            Opcode::WhenGreenFlagClicked() => {
                opcode_str = "event_whenflagclicked";
            },

            Opcode::WhenThisSpriteClicked() => {
                opcode_str = "event_whenthisspriteclicked";
            },

            Opcode::WhenStageClicked() => {
                opcode_str = "event_whenstageclicked";
            },

            Opcode::WhenBroadcastReceived(broadcast_option) => {
                todo!("event_whenbroadcastreceived");
            },

            Opcode::WhenBackdropSwitchesTo(backdrop) => {
                todo!("event_whenbackdropswitchesto");
            },

            Opcode::WhenGreaterThan(when_greater_than_menu, value) => {
                todo!("event_whengreaterthan");
            },

            Opcode::BroadcastMenu(broadcast_option) => {
                todo!("event_broadcast_menu");
            },

            Opcode::Broadcast(broadcast_input) => {
                todo!("event_broadcast");
            },

            Opcode::BroadcastAndWait(broadcast_input) => {
                todo!("event_broadcastandwait");
            },

            Opcode::WhenKeyPressed(key_option) => {
                todo!("event_whenkeypressed");
            },


            // LOOKS //
            Opcode::SayForSecs(message, secs) => {
                opcode_str = "looks_sayforsecs";
                inputs["MESSAGE"] = message.serialize(InputType::String)?;
                inputs["SECS"] = message.serialize(InputType::PositiveNumber)?;
            },

            Opcode::Say(message) => {
                opcode_str = "looks_say";
                inputs["MESSAGE"] = message.serialize(InputType::String)?;
            },

            Opcode::ThinkForSecs(message, secs) => {
                opcode_str = "looks_thinkforsecs";
                inputs["MESSAGE"] = message.serialize(InputType::String)?;
                inputs["SECS"] = secs.serialize(InputType::PositiveNumber)?;
            },

            Opcode::Think(message) => {
                opcode_str = "looks_think";
                inputs["THINK"] = message.serialize(InputType::String)?;
            },

            Opcode::Show() => {
                opcode_str = "looks_show";
            },

            Opcode::Hide() => {
                opcode_str = "looks_hide";
            },

            // obsolete: looks_hideallsprites
            
            Opcode::ChangeGraphicEffectBy(effect, change) => {
                todo!("looks_changeeffectby");
            },

            Opcode::SetGraphicEffectTo(effect, change) => {
                todo!("looks_seteffectto");
            },

            Opcode::ClearGraphicEffects() => {
                opcode_str = "looks_cleargraphiceffects";
            },

            Opcode::ChangeSizeBy(change) => {
                opcode_str = "looks_changesizeby";
                inputs["CHANGE"] = change.serialize(InputType::Number)?;
            },

            Opcode::SetSizeTo(size) => {
                opcode_str = "looks_setsizeto";
                inputs["SIZE"] = size.serialize(InputType::Number)?;
            },

            Opcode::Size() => {
                opcode_str = "looks_size";
            },

            // obsolete: looks_changestretchby
            // obsolete: looks_setstretchto

            Opcode::Costume(costume) => {
                todo!("looks_costume");
            },

            Opcode::SwitchCostumeTo(costume) => {
                todo!("looks_switchcostumeto");
            },

            Opcode::NextCostume() => {
                opcode_str = "looks_nextcostume";
            },

            Opcode::SwitchBackdropTo(backdrop) => {
                todo!("looks_switchbackdropto");
            },

            Opcode::Backdrops(backdrop) => {
                todo!("looks_backdrops");
            },

            Opcode::GoToFrontBack(front_back) => {
                todo!("looks_gotofrontback");
            },

            Opcode::GoForwardBackwardLayers(forward_backward) => {
                todo!("looks_goforwardbackwardlayers");
            },

            Opcode::BackdropNumberName(number_name) => {
                todo!("looks_backdropnumbername");
            },

            Opcode::CostumeNumberName(number_name) => {
                todo!("looks_costumenumbername");
            },

            Opcode::SwitchBackdropToAndWait(backdrop) => {
                todo!("looks_switchbackdroptoandwait");
            },

            Opcode::NextBackdrop() => {
                opcode_str = "looks_nextbackdrop";
            },


            // MOTION //

            Opcode::MoveSteps(steps) => {
                opcode_str = "motion_movesteps";
                inputs["STEPS"] = steps.serialize(InputType::Number)?;
            },
            
            Opcode::TurnRight(degrees) => {
                opcode_str = "motion_turnright";
                inputs["DEGREES"] = degrees.serialize(InputType::Number)?;
            },

            Opcode::TurnLeft(degrees) => {
                opcode_str = "motion_turnleft";
                inputs["DEGREES"] = degrees.serialize(InputType::Number)?;
            },

            Opcode::PointInDirection(direction) => {
                opcode_str = "motion_pointindirection";
                inputs["DIRECTION"] = direction.serialize(InputType::Angle)?;
            },

            Opcode::PointTowardsMenu(towards) => {
                todo!("motion_pointtowards_menu");
            },

            Opcode::PointTowards(towards) => {
                todo!("motion_pointtowards");
            },

            Opcode::GoToMenu(to) => {
                todo!("motion_goto_menu");
            },

            Opcode::GoToXY(x, y) => {
                opcode_str = "motion_gotoxy";
                inputs["X"] = x.serialize(InputType::Number)?;
                inputs["Y"] = y.serialize(InputType::Number)?;
            },

            Opcode::GoTo(to) => {
                todo!("motion_goto");
            },

            Opcode::GlideSecsToXY(secs, x, y) => {
                opcode_str = "motion_glidesecstoxy";
                inputs["SECS"] = secs.serialize(InputType::PositiveNumber)?;
                inputs["X"] = x.serialize(InputType::Number)?;
                inputs["Y"] = y.serialize(InputType::Number)?;
            },

            Opcode::GlideToMenu(to) => {
                todo!("motion_glidetomenu");
            },

            Opcode::GlideTo(secs, to) => {
                todo!("motion_glideto");
            },

            Opcode::ChangeXBy(dx) => {
                opcode_str = "motion_changexby";
                inputs["DX"] = dx.serialize(InputType::Number)?;
            },

            Opcode::SetX(x) => {
                opcode_str = "motion_setx";
                inputs["X"] = x.serialize(InputType::Number)?;
            },

            Opcode::ChangeYBy(dy) => {
                opcode_str = "motion_changeyby";
                inputs["DY"] = dy.serialize(InputType::Number)?;
            },

            Opcode::SetY(y) => {
                opcode_str = "motion_sety";
                inputs["Y"] = y.serialize(InputType::Number)?;
            },

            Opcode::IfOnEdgeBounce() => {
                opcode_str = "motion_ifonedgebounce";
            },

            Opcode::SetRotationStyle(style) => {
                todo!("looks_setrotationstyle");
            },

            Opcode::XPosition() => {
                opcode_str = "motion_xposition";
            },

            Opcode::YPosition() => {
                opcode_str = "motion_yposition";
            },

            Opcode::Direction() => {
                opcode_str = "motion_direction";
            },


            // Operators //
            Opcode::Add(num1, num2) => {
                opcode_str = "operator_add";
                inputs["NUM1"] = num1.serialize(InputType::Number)?;
                inputs["NUM2"] = num2.serialize(InputType::Number)?;
            },

            Opcode::Subtract(num1, num2) => {
                opcode_str = "operator_add";
                inputs["NUM1"] = num1.serialize(InputType::Number)?;
                inputs["NUM2"] = num2.serialize(InputType::Number)?;
            },

            Opcode::Multiply(num1, num2) => {
                opcode_str = "operator_multiply";
                inputs["NUM1"] = num1.serialize(InputType::Number)?;
                inputs["NUM2"] = num2.serialize(InputType::Number)?;
            },

            Opcode::Divide(num1, num2) => {
                opcode_str = "operator_divide";
                inputs["NUM1"] = num1.serialize(InputType::Number)?;
                inputs["NUM2"] = num2.serialize(InputType::Number)?;
            },

            Opcode::Random(from, to) => {
                opcode_str = "operator_random";
                inputs["FROM"] = from.serialize(InputType::Number)?;
                inputs["TO"] = to.serialize(InputType::Number)?;
            },

            Opcode::Lt(operand1, operand2) => {
                opcode_str = "operator_lt";
                inputs["OPERAND1"] = operand1.serialize(InputType::Number)?;
                inputs["OPERAND2"] = operand2.serialize(InputType::Number)?;
            },

            Opcode::Equals(operand1, operand2) => {
                opcode_str = "operator_equals";
                inputs["OPERAND1"] = operand1.serialize(InputType::Number)?;
                inputs["OPERAND2"] = operand2.serialize(InputType::Number)?;
            },

            Opcode::Gt(operand1, operand2) => {
                opcode_str = "operator_gt";
                inputs["OPERAND1"] = operand1.serialize(InputType::Number)?;
                inputs["OPERAND2"] = operand2.serialize(InputType::Number)?;
            },

            Opcode::And(operand1, operand2) => {
                todo!("operator_and");
            },

            Opcode::Or(operand1, operand2) => {
                todo!("operator_or");
            },

            Opcode::Not(operand) => {
                todo!("operator_not");
            },

            Opcode::Join(string1, string2) => {
                opcode_str = "operator_join";
                inputs["STRING1"] = string1.serialize(InputType::Number)?;
                inputs["STRING2"] = string2.serialize(InputType::Number)?;
            },

            Opcode::LetterOf(letter, string) => {
                opcode_str = "operator_letter_of";
                inputs["LETTER"] = letter.serialize(InputType::WholeNumber)?;
                inputs["STRING"] = string.serialize(InputType::String)?;
            },

            Opcode::Length(string) => {
                opcode_str = "operator_length";
                inputs["STRING"] = string.serialize(InputType::String)?;
            },

            Opcode::Contains(string1, string2) => {
                opcode_str = "operator_contains";
                inputs["STRING1"] = string1.serialize(InputType::String)?;
                inputs["STRING2"] = string2.serialize(InputType::String)?;
            },

            Opcode::Mod(num1, num2) => {
                opcode_str = "operator_mod";
                inputs["NUM1"] = num1.serialize(InputType::Number)?;
                inputs["NUM2"] = num2.serialize(InputType::Number)?;
            },

            Opcode::Round(num) => {
                opcode_str = "operator_round";
                inputs["NUM"] = num.serialize(InputType::Number)?;
            },

            Opcode::MathOp(operator, num) => {
                todo!("operator_mathop");
            },

            // TODO how do custom procedures work?

            // Sensing //
            Opcode::TouchingObject(touchingobjectmenu) => {
                todo!("sensing_touchingobject");
            },

            Opcode::TouchingObjectMenu(touchingobjectmenu) => {
                todo!("sensing_touchingobjectmenu");
            },

            Opcode::TouchingColor(color) => {
                todo!("sensing_touchingcolor");
            },

            Opcode::ColorIsTouchingColor(color, color2) => {
                todo!("sensing_coloristouchingcolor");
            },

            Opcode::DistanceTo(distancetomenu) => {
                todo!("sensing_distanceto");
            },

            Opcode::DistanceToMenu(distancetomenu) => {
                todo!("sensing_distancetomenu");
            },

            Opcode::AskAndWait(question) => {
                opcode_str = "sensing_askandwait";
                inputs["QUESTION"] = question.serialize(InputType::String)?;
            },

            Opcode::Answer() => {
                opcode_str = "sensing_answer";
            },

            Opcode::KeyPressed(key_option) => {
                todo!("sensing_keypressed");
            },

            Opcode::KeyOptions(key_option) => {
                todo!("sensing_keyoptions");
            },

            Opcode::MouseDown() => {
                opcode_str = "sensing_mousedown";
            },

            Opcode::MouseX() => {
                opcode_str = "sensing_mousex";
            },

            Opcode::MouseY() => {
                opcode_str = "sensing_mousey";
            },

            Opcode::SetDragMode(drag_mode) => {
                todo!("sensing_setdragmode");
            },

            Opcode::Loudness() => {
                opcode_str = "sensing_loudness";
            },

            // obsolete: sensing_loud

            Opcode::Timer() => {
                opcode_str = "sensing_timer";
            },

            Opcode::OfObjectMenu(object) => {
                todo!("sensing_of_object_menu");
            },

            Opcode::Of(property, object) => {
                todo!("sensing_of");
            },

            Opcode::Current(currentmenu) => {
                todo!("sensing_current");
            },

            Opcode::DaysSince2000() => {
                opcode_str = "sensing_dayssince2000";
            },

            Opcode::Username() => {
                opcode_str = "sensing_username";
            },

            // obsolete: sensing_userid

            // Sound //
            // TODO what is sound_sounds_menu?

            Opcode::SoundsMenu(sound) => {
                todo!("sound_sounds_menu");
            },

            Opcode::Play(sound_menu) => {
                todo!("sound_play");
            },

            Opcode::PlayUntilDone(sound_menu) => {
                todo!("sound_playuntilmode");
            },

            Opcode::StopAllSounds() => {
                opcode_str = "sound_stopallsounds";
            },

            Opcode::SetSoundEffectTo(effect, value) => {
                todo!("sound_seteffectto");
            },

            Opcode::ChangeSoundEffectBy(effect, value) => {
                todo!("sound_changeeffectby");
            },

            Opcode::ClearSoundEffects() => {
                opcode_str = "sound_cleareffects";
            },

            Opcode::ChangeVolumeBy(volume) => {
                opcode_str = "sound_changevolumeby";
                inputs["VOLUME"] = volume.serialize(InputType::Number)?;
            },

            Opcode::SetVolumeTo(volume) => {
                opcode_str = "sound_setvolumeto";
                inputs["VOLUME"] = volume.serialize(InputType::Number)?;
            },

            Opcode::Volume() => {
                opcode_str = "sound_volume";
            },



            _ => {
                panic!("unimplemented opcode");
            }
        }

        res["opcode"] = JsonValue::String(opcode_str.to_string());
        res["next"] = json::Null;
        res["parent"] = match parent_uuid {
            Some(v) => v.clone().into(),
            None => json::Null
        };
        res["inputs"] = inputs;
        res["fields"] = fields;
        res["shadow"] = JsonValue::Boolean(false);
        res["topLevel"] = JsonValue::Boolean(true);

        Ok(res)
    }
}