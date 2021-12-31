use std::io;
use std::io::{BufRead, BufReader};
use std::cell::{Ref, RefCell, RefMut};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::time:: SystemTime;

// Component trait is used to have overlap between each component type supplied
trait Component {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn push_none(&mut self);
}

// Every component type needs to be known at run time and last the duration of the program
// Implemented for a mutable vectors that could contain the component
// All component vectors have the same size
impl<T: 'static> Component for RefCell<Vec<Option<T>>> {
    // Borrow the vector of component
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }
    // Mutability borrow the vector of compoent
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }
    // Push empty into that vector location
    fn push_none(&mut self) {
        self.get_mut().push(None)
    }
}

// Contains all of the data used by the program
struct World {
    entities_count: usize,
    components: Vec<Box<dyn Component>>
}

impl World {
    fn new() -> Self {
        Self {
            entities_count: 0,
            components: Vec::new(),
        }
    }

    fn new_entity(&mut self) -> usize {
        let entity_id = self.entities_count;
        // Append a new None entry to every component type we have available 
        // Which  will be the new entity id
        for component in self.components.iter_mut() {
            component.push_none();
        }
        self.entities_count += 1;
        entity_id
    }

    // Populate the component vector at entry "entity"
    fn add_component_to_entity<ComponentType: 'static>(
        &mut self,
        entity: usize,
        component: ComponentType,
    ) {
        for component_vec in self.components.iter_mut() {
            if let Some(component_vec) = component_vec.as_any_mut()
                .downcast_mut::<RefCell<Vec<Option<ComponentType>>>>() 
            {
                component_vec.borrow_mut()[entity] = Some(component);
                return;
            }
        }
        let mut new_component: Vec<Option<ComponentType>> = Vec::with_capacity(self.entities_count);

        // Whenever we add a new component type we need to make the vector the same length as the other components
        for _ in 0..self.entities_count {
            new_component.push(None);
        }

        new_component[entity] = Some(component);

        //print_type_of(&new_component);
        // Append the pointer to the refcell of the data 
        self.components.push(Box::new(RefCell::new(new_component)));
    }

    // Cycle through the components until you finally find the component tye. Mutably borrow that vector
    fn borrow_component_mut<ComponentType: 'static> (&self) -> Option<RefMut<Vec<Option<ComponentType>>>> {
        for component_vec in self.components.iter() {
            if let Some(component_vec) = component_vec
                .as_any()
                .downcast_ref::<RefCell<Vec<Option<ComponentType>>>>()
            {
                return Some(component_vec.borrow_mut());
            }
        }
        None
    }

    // Cycle through the components until you finally find the component tye. Borrow that vector
    fn borrow_component<ComponentType: 'static> (&self) -> Option<Ref<Vec<Option<ComponentType>>>> {
        for component_vec in self.components.iter() {
            if let Some(component_vec) = component_vec
                .as_any()
                .downcast_ref::<RefCell<Vec<Option<ComponentType>>>>()
            {
                return Some(component_vec.borrow());
            }
        }
        None
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum Command {
    Move, // Only takes one of the four directions forward/back/left/right
    Check, // Doesn't need to take anything else
    Use,  // Needs to list the various items in the Game
}

impl Command {
    fn from_str(s: &str) -> Result<Command, &str> {
        // Prepping the value before it's used
        match s.to_lowercase().as_str() {
            "move" => Ok(Command::Move),
            "check" => Ok(Command::Check),
            "use" => Ok(Command::Use),
            _ => Err(s),
        }
    }
}

#[derive(Debug)]
enum Direction {
    Forward,
    Back,
    Left,
    Right,
}

impl Direction {
    fn from_str(s: &str) -> Result<Direction, &str> {
        match s.to_lowercase().as_str() {
            "forward" => Ok(Direction::Forward),
            "back" => Ok(Direction::Back),
            "left" => Ok(Direction::Left),
            "right" => Ok(Direction::Right),
            _ => Err("Failed to find direction"),
        }
    }
}

#[derive(Debug)]
enum Inquire {
    Pocket,
    Area
}
impl Inquire {
    fn from_str(s: &str) -> Result<Inquire, &str> {
        match s {
            "pocket" => Ok(Inquire::Pocket),
            "area" => Ok(Inquire::Area),
            _ => Err("Failed to find something to inquire")
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum Item {
    Canister,
    Lighter,
    Watch,
    Rock,
}

impl Item {
    fn from_str(s: &str) -> Result<Item, &str> {
        match s {
            "canister" => Ok(Item::Canister),
            "lighter" => Ok(Item::Lighter),
            "watch" => Ok(Item::Watch),
            "rock" => Ok(Item::Rock),
            _ => Err("Failed to find the item")
        }
    }

    fn to_string(&self) -> &str {
        match self {
            Item::Canister => "Canister",
            Item::Lighter => "Lighter",
            Item::Watch => "Watch",
            Item::Rock => "Rock"
        }
    }
}

#[derive(Eq, Hash, Copy, Clone)]
struct LocationComponent {
    x: i32,
    y: i32,
 }

impl LocationComponent {
    fn move_forward(&mut self) {
        if self.y < 2 {
            self.y += 1;
            return;
        } 
    }

    fn move_back(&mut self) {
        if self.y > 0 {
            self.y -= 1;
            return;
        }
    }

    fn move_right(&mut self) {
        if self.x < 2 {
            self.x += 1;
            return;
        }
    }

    fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
            return;
        }
    }
    fn get_out_of_bounds(&self) -> String {
        String::from("I don't want to stray to far from my house")
    }
    fn update_location(&mut self, dir: Direction) {
        match dir {
            Direction::Forward => self.move_forward(),
            Direction::Right => self.move_right(),
            Direction::Left => self.move_left(),
            Direction::Back => self.move_back(),
        }
    }

    fn print_location(&self) {
        println!("x: {}, y: {}", self.x, self.y);
    }
    fn to_string(&self) -> String {
        return format!("x: {}, y:{}", self.x, self.y);
    }

    // This is a nightmare and I need to find a better way of doing this
    fn parse(input: &str) -> Result<LocationComponent, &str> {
        let mut x: i32 = 0;
        if !input.contains("x") || !input.contains("y") {
            return Err("No Coordinates found");
        }
        let mut digits = String::new();
        for c in input.chars() {
            if c == 'y' {
                x = digits.parse().unwrap();
                digits.clear();
                continue;
            }
            // Check if its a base 10 digit or if its a '-' for negative numbers
            if c.is_digit(10)  || (c == '-' && digits.is_empty()) {
                digits.push(c);
            }
        }
        let y = digits.parse().unwrap();

        Ok(LocationComponent { x: x, y: y})
    }
}
impl PartialEq for LocationComponent {
    fn eq(&self, other: &Self) -> bool {
        if self.x == other.x && self.y == other.y {
            return true;
        }
        false
    }
}

struct MapComponent {
    // Does this need to be part of the player? 
        // Having the player own the map seems odd it  should be a static variable instead
    // Needs to contain vec<>
    // I can have multiple maps based on who is holding it (enemy/player) so it should be
    // a component added to the player
    area: HashMap<LocationComponent, String>,
    item_locations: HashMap<LocationComponent, String>

}

#[allow(unused)]
impl MapComponent {
    fn new(contents: &str) -> Self {
        let mut area = HashMap::new();
        let mut item_locations = HashMap::new();
        for line in contents.lines() {
            let vec_string: Vec<_> = line.split("|").collect();
            // This code makes some assumptions about the strings provided
            let location: LocationComponent = LocationComponent::parse(vec_string[0]).unwrap();
            area.insert(location.clone(), String::from(vec_string[1]));
            if vec_string.len() > 2 {
                item_locations.insert(location.clone(), String::from(vec_string[2]));
            }
        }
        MapComponent {
            area,
            item_locations
        }
    }
    fn new_from_file(filename: &str) -> Self {
        let mut area = HashMap::new();
        let mut item_locations = HashMap::new();
        let open_file = fs::File::open(filename).unwrap();
        let reader = BufReader::new(open_file);
        for line in reader.lines() {
            let line = line.unwrap();
            let vec_string: Vec<_> = line.split("|").collect();
            // This code makes some assumptions about the strings provided
            let location: LocationComponent = LocationComponent::parse(vec_string[0]).unwrap();
            area.insert(location.clone(), String::from(vec_string[1]));
            if vec_string.len() > 2 {
                item_locations.insert(location.clone(), String::from(vec_string[2]));
            }
        }
        //let file = fs::read_to_string(filename).unwrap_or(String::from("Failed to find file"));
        //println!("File {} Contents {}", filename, file);
        MapComponent {
            area,
            item_locations
        }
    }

    fn print_entire_map(&self) {
        for i in self.area.iter() {
            println!("At Location {} the information is {}", i.0.to_string(), i.1);
        }
    }
    // May need to return Result and not Option, still mulling over if I want an Err message
    fn check_area(&self, location: &LocationComponent) -> Result<&String, &str> {
        if let Some(description) = self.area.get(location) {
            return Ok(description);
        } 
        Err("Player is out of bounds")
    }
    fn check_item_locations(&mut self, location: &LocationComponent) -> Result<String, &str> {
        if let Some(item) = self.item_locations.get(location) {
            let ret_item = item.clone(); // Cloning here since I want to remove the entry after taking the value
            self.item_locations.remove(location); // This works due to interior mutability
            return Ok(ret_item);
        }
        Err("There isn't anything else here")
    }
}

#[allow(unused)]
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}
#[allow(unused)]
fn print_type_of_with_message<T>(message: &str, _: &T) {
    println!("{}: {}",message, std::any::type_name::<T>());
}

#[allow(unused)]
struct PlayerComponent {
    name: String,
    inventory: HashSet<Item>,
    start_time: SystemTime,
    remaining_time: u64,
    is_alive: bool,
    is_game_over: bool
}

impl PlayerComponent {
    fn new(input: &str) -> Self {
        let mut inventory = HashSet::new();
        inventory.insert(Item::Lighter);
        inventory.insert(Item::Watch);
        let start_time = SystemTime::now();
        PlayerComponent {
            name: String::from(input),
            inventory,
            start_time,
            remaining_time: 0,
            is_alive: true,
            is_game_over: false
        }
    }
    fn get_inventory(&self) -> String {
        let mut list = String::new();
        list.push('{');
        self.inventory.iter().for_each(|x| {
            list.push_str(x.to_string());
            list.push_str(", ")
        });
        list.truncate(list.len() - 2); // Hard Number but I want to remove the ", "
        list.push('}');
        format!("I have {} in my pocket", list)
    }

    fn insert_item(&mut self, item: Item) {
        self.inventory.insert(item);
    }

    fn set_remaining_time(&mut self, remaining_time: u64) {
        self.remaining_time = remaining_time;
    }
    fn set_is_alive(&mut self, is_alive: bool) {
        self.is_alive = is_alive;
    }
    fn set_is_game_over(&mut self, is_game_over: bool) {
        self.is_game_over = is_game_over;
    }
}

struct DoorComponent {
    is_frozen: bool,
    is_window_intact: bool,
    is_gasolined: bool
}

impl DoorComponent {
    pub fn new() -> Self {
        DoorComponent {
            is_frozen: true,
            is_window_intact: true,
            is_gasolined: false,
        }
    }
    pub fn is_frozen(&self) -> bool {
        self.is_frozen
    }
    pub fn is_gasolined(&self) -> bool {
        self.is_gasolined
    }
    pub fn set_is_frozen(&mut self, frozen: bool) {
        self.is_frozen = frozen;
    }
    pub fn set_is_window_intact(&mut self, window_intact: bool) {
        self.is_window_intact = window_intact;
    }
    pub fn set_is_gasolined(&mut self, gasolined: bool) {
        self.is_gasolined = gasolined;
    }
}


// Ugly will fix later
const HELP_STRING: &'static str = "Availabile Commands {{Move, Check, Use}}
When I Move I need to decide on a Direction {{Forward, Back, Left, Right}}
I could Check my {{Pocket}} or the surrounding {{Area}} 
I can also {{Use}} items in my inventory";

const INTRO_STRING: &'static str = "I finally found my way out of the woods. I see the cabin in the distance.
I am freezing though and don't know how much longer I can stay out here. 
I'll keep an eye on my {{Watch}} to help me.";

const GAME_MAX_DURATION: u64 = 120; 

fn input_system(buffer: &mut String) -> Vec<&str> {
    get_input(buffer);
    let command_vec = process_string(buffer);

    command_vec
}

#[allow(unused)]
fn get_input(buffer: &mut String) {
    io::Write::flush(&mut io::stdout());
    buffer.clear();

    io::stdin().read_line(buffer);
}

fn process_string(buffer: &mut String) -> Vec<&str> {
    if buffer.to_lowercase().contains("exit") {
        // Just terminate the program here if requested
        std::process::exit(0);
    }
    let mut command_vec: Vec<&str> = vec![];

    if buffer.to_lowercase().contains("help") {
        println!("{}", HELP_STRING);
        return command_vec;
    }

    command_vec.extend(buffer.split_ascii_whitespace());

    command_vec
}
// Didn't need to be it's own system, however in the future I will need a startup system that runs once
fn print_introduction_system() {
    println!("{}", INTRO_STRING);
}
fn print_location_system(world: &World) {
    let borrow_location_wrapped = world.borrow_component::<LocationComponent>();
    if borrow_location_wrapped.is_none() {
        println!("LocationComponent is none");
        std::process::exit(1);
    }
    let location_ref: Ref<Vec<Option<LocationComponent>>> = borrow_location_wrapped.unwrap();
    let location_iter = location_ref.iter();

    for location in location_iter {
        location.as_ref().unwrap().print_location();
    }
}

#[allow(unused)]
fn print_map_system(world: &World) {
    let borrow_map_wrapped = world.borrow_component::<MapComponent>();
    if borrow_map_wrapped.is_none() {
        println!("MapComponent is none");
        std::process::exit(1);
    }
    let map_ref: Ref<Vec<Option<MapComponent>>> = borrow_map_wrapped.unwrap();
    let map_iter = map_ref.iter();
    for map in map_iter {
        map.as_ref().unwrap().print_entire_map();
    }
}

fn update_door_system(world: &World, command_vec: &Vec<&str>, player_entity: usize, door_entity: usize, game_output: &mut String) {
    if command_vec.is_empty() {
        return;
    }
    let locations = world.borrow_component::<LocationComponent>().unwrap();
    let mut players = world.borrow_component_mut::<PlayerComponent>().unwrap();
    let mut player_self = players[player_entity].as_mut().expect("Failed to find Player entity");
    let player_location = locations[player_entity].as_ref().expect("Player does not have a location");
    let door_location = locations[door_entity].as_ref().expect("Door does not have a location");


    if !player_location.eq(door_location) {
        return;
    }
    let mut iter = command_vec.iter();
    let command = Command::from_str(iter.next().unwrap_or(&"Command Required to act {{Move, Check, Use}}"));

    match command {
        Ok(Command::Use) => {
            if let Ok(item) = Item::from_str(iter.next().unwrap_or(&"Item required to use, maybe I should {{Check Pocket}}")) {
                let mut doors = world.borrow_component_mut::<DoorComponent>().unwrap();
                let door = doors[door_entity].as_mut().expect("Could not find a door component");
                match item {
                    Item::Canister => {
                        if !player_self.inventory.contains(&Item::Canister) {
                            return;
                        } 
                        if door.is_frozen() {
                            game_output.clear();
                            game_output.push_str("The contents of the canister were poured on the doorknob");
//                            println!("*You poured the contents of the canister on the doorknob");
                            door.set_is_gasolined(true);
                        } else {
                            game_output.push_str("The canister is already empty");

//                            println!("The canister is already empty");
                        }
                    }
                    Item::Lighter => {
                        if door.is_gasolined() {
                            game_output.clear();
                            game_output.push_str("Looks like I can melt the doorknob now\n");
                            game_output.push_str("*Lights doorknob aflame*\n");
                            player_self.set_is_game_over(true);
                            door.set_is_frozen(false);
                        } else {
                            game_output.clear();
                            game_output.push_str("I'll run out of fuel in my lighter before I finish melting the doorknob"); 
                        }
                    }
                    Item::Rock => {
                        if !player_self.inventory.contains(&Item::Rock) {
                            return;
                        } 
                        game_output.clear();
                        game_output.push_str("I can smash the window using this rock\n");
                        game_output.push_str("*Smashes window with rock*\n");
                        door.set_is_window_intact(false);
                        player_self.set_is_game_over(true);
                    }
                    _ => {
                    }
                }
            }
        }
        _ => (),
    }
}

fn update_player_system(world: & World, command_vec: &Vec<&str>, player_entity: usize, game_output: &mut String) {
    if command_vec.is_empty() {
        game_output.push_str("I may need some {{help}} with what to do next");
        //println!("Require a command to know what to do next");
        return;
    }

    // Im not fully grasping the ECS system yet since Im editing on the player variables based on input
    // Perhaps if I add other entities into this world I will better understand how to break out the logic
    let mut players = world.borrow_component_mut::<PlayerComponent>().unwrap();
    let mut locations = world.borrow_component_mut::<LocationComponent>().unwrap(); 
    let mut map = world.borrow_component_mut::<MapComponent>().unwrap();
    
    let player_location = locations[player_entity].as_mut().expect("Player does not have a location");
    let player_map = map[player_entity].as_mut().expect("Player does not have a map");
    let player_self = players[player_entity].as_mut().expect("Player does not exist");

    let mut iter = command_vec.iter();
    let command = Command::from_str(iter.next().unwrap_or(&"Command Required to act {{Move, Check, Use}}"));
    match command {
        Ok(Command::Move) => {
            if let Ok(dir) = Direction::from_str(iter.next().unwrap_or(&"Failed to find next entry in vector")) {
                let player_location_old = player_location.clone();
                player_location.update_location(dir);
                // If the location hasn't changed don't change the map data
                if player_location_old.eq(player_location) {
                    game_output.push_str(player_location.get_out_of_bounds().as_str());
                    return;
                }
                match player_map.check_area(player_location) {
                    Ok(result) => {
                        if !player_location_old.eq(player_location) {
                            game_output.push_str(result);
                            //println!("{}", result);
                        }
                    }
                    _ => ()
                }
                
            } else {
                // TODO - Make this more immersive "I'm not sure which direction to go"
                game_output.push_str("Failed to find a direction to move to {{Forward, Back, Left, Right}}");
                // println!("Failed to find a direction to move to {{Forward, Back, Left, Right}}");
            }
        },
        Ok(Command::Check) => {
            if let Ok(inq) = Inquire::from_str(iter.next().unwrap_or(&"Failed to find next entry in the vector")) {
                match inq {
                    Inquire::Area => {
                        match player_map.check_item_locations(player_location) {
                            Ok(item) => {
                                //println!("Looks like there's {} here. I'll hold on to it for later", item);
                                game_output.push_str(format!("Looks like there's {} here. I'll hold on to it for later", item).as_str());
                                player_self.insert_item(Item::from_str(&item).unwrap());
                            }
                            _ => { 
                                game_output.push_str("Looks like there's nothing of interest here");
                            } // Do nothing if we already found the item
                            //Err(err) => println!("{}", err),
                        }
                    }
                    Inquire::Pocket => {
                        game_output.push_str(player_self.get_inventory().as_str());
                    } 
                }
            }  else {
                game_output.push_str("I'm not sure what to check, all I see is the {{Area}} and all I have are what's in my {{Pocket}}");
                //println!("I'm not sure what to check, all I see is the {{Area}} and all I have are what's in my {{Pocket}}");

            }
        }
        Ok(Command::Use) => {
            if let Ok(item) = Item::from_str(iter.next().unwrap_or(&"Failed to find next entry in the vector")) {
                match item {
                    Item::Canister => {
                        if !player_self.inventory.contains(&Item::Canister) {
                            game_output.push_str("I don't have that on me right now");
                            return;
                        } 
                        game_output.push_str("I have no use for this canister right now");

                    }
                    Item::Lighter => {
                        game_output.push_str("I have no use for this lighter right now");
                    }
                    Item::Watch => {
                        game_output.push_str(format!("It's so cold, I only have {} seconds before my watch dies", player_self.remaining_time).as_str()) //println!("Using Watch"),
                    }
                    Item::Rock => {
                        if !player_self.inventory.contains(&Item::Rock) {
                            game_output.push_str("I don't have that on me right now");
                            return;
                        } 
                        game_output.push_str("I have no use for this rock right now");

                    }
                }
            } else {
                game_output.push_str("Not sure what I should use. Perhaps I should {{check pocket}}");
                //println!("Not sure what I should use. Perhaps I should {{check pocket}}");
            }
        }
        Err(e) => {
            // TODO - Make this more immersive "I'm not sure which direction to go"
            game_output.push_str(format!("Error bad input: \"{}\" is not a command\nTry asking for {{Help}}", e).as_str());
            //println!("Error bad input: \"{}\" is not a command\nTry asking for {{Help}}", e);
        }
    }
}

// Clunky systems requires me to add additional variables for each
fn game_ending_system(world: &World, player_entity: usize, door_entity: usize, game_output: &mut String) {    
    let player_components = world.borrow_component::<PlayerComponent>().unwrap();
    let door_components = world.borrow_component::<DoorComponent>().unwrap();
    let player_self = player_components[player_entity].as_ref().unwrap();
    let door = door_components[door_entity].as_ref().unwrap();
    if player_self.is_game_over {
        if !player_self.is_alive {
            game_output.clear();
            // Losing: boolean flag to check if time has run out
            game_output.push_str("I feel my eyelids getting heavy...\nPerhaps I should rest for a bit...");
        } else if !door.is_frozen {
        // Winning: boolean flag to check if correct item has been used different message
            game_output.push_str("Looks like the doorknob has thawed and I can get in");

        } else if !door.is_window_intact {
        // Winning: boolean flag to check if correct item has been used
            game_output.push_str("There's a hole in the window I can climb through now");
        } 
        game_output.push_str("\nGame Over");
        // Easier to have this handle the program exit
        render_system(game_output);
        std::process::exit(0);
    }
}

// This function needs to return some form output
fn entity_logic_system(world: &World, command_vec: &Vec<&str>, player_entity: usize, door_entity: usize) -> String {
        let mut game_output = String::new();
        update_player_system(&world, &command_vec, player_entity, &mut game_output);
        update_door_system(&world, &command_vec, player_entity, door_entity, &mut game_output);
        game_ending_system(world, player_entity, door_entity, &mut game_output);
        //print_location_system(&world);
        game_output
}

fn render_system(display_text: &str) {
    println!("{}", display_text);
}

fn time_system(world: &World, player_entity: usize) {
    let mut player_components = world.borrow_component_mut::<PlayerComponent>().unwrap();
    let player_self = player_components[player_entity].as_mut().unwrap();
    let now = SystemTime::now();
    let duration = now.duration_since(player_self.start_time).unwrap();


    if duration.as_secs() > GAME_MAX_DURATION {
        //println!("I feel my eyelids getting heavy...\nPerhaps I should rest for a bit...");
        //println!("Game Over");
        player_self.set_is_alive(false);
    } else {
        let remaining_time = GAME_MAX_DURATION - duration.as_secs();
        player_self.set_remaining_time(remaining_time);
    }
}

fn main() {
    // Setup Initial Variables outside of main loop
    let mut buffer = String::new();

    let mut world = World::new();
    let player_entity = world.new_entity();
    world.add_component_to_entity(player_entity, PlayerComponent::new("Jakob"));
    world.add_component_to_entity(player_entity, LocationComponent{x: 0, y: 0});
 
    world.add_component_to_entity(player_entity, MapComponent::new(include_str!("player_map.txt")));

    let door_entity = world.new_entity();
    world.add_component_to_entity(door_entity, LocationComponent{x: 2, y: 2});
    world.add_component_to_entity(door_entity, DoorComponent::new());
    
    print_introduction_system();
    // TODO: Give intro sequence, explaining situation goal and timelimit
    // TODO: Need to make a door component that reacts when a flag is trigger by the player ie, used Canister at a certain location
    loop {
        let command_vec = input_system(&mut buffer);
        time_system(&world, player_entity);
        let output = entity_logic_system(&world, &command_vec, player_entity, door_entity);
        render_system(&output);
    }
}
