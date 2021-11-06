use std::io;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};

trait Component {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn push_none(&mut self);
}

impl<T: 'static> Component for RefCell<Vec<Option<T>>> {
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }

    fn push_none(&mut self) {
        self.get_mut().push(None)
    }
}

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
        for component in self.components.iter_mut() {
            component.push_none();
        }
        self.entities_count += 1;
        entity_id
    }

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

        for _ in 0..self.entities_count {
            new_component.push(None);
        }

        new_component[entity] = Some(component);

        //print_type_of(&new_component);

        self.components.push(Box::new(RefCell::new(new_component)));
    }

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
            _ => Err("Failed to find command"),
        }
    }
}

#[allow(dead_code)]
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

struct LocationComponent {
    x: i32,
    y: i32,
 }

impl LocationComponent {
    fn move_forward(&mut self) {
        self.y += 1;
    }

    fn move_back(&mut self) {
        self.y -= 1;
    }

    fn move_right(&mut self) {
        self.x += 1;
    }

    fn move_left(&mut self) {
        self.x -= 1;
    }

    fn print_location(&self) {
        println!("x: {}, y: {}", self.x, self.y);
    }
}

struct MapComponent {
    // Does this need to be part of the player? 
        // Having the player own the map seems odd it  should be a static variable instead
    // Needs to contain vec<>
    // I can have multiple maps based on who is holding it (enemy/player) so it should be
    // a component added to the player
    area: HashMap<LocationComponent, String>,

}

impl MapComponent {
    fn new(filename: &str) -> Self {
        let area = HashMap::new();
        /*let open_file = fs::File::open(filename).unwrap();
        let reader = BufReader::new(open_file);
        for (index, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            println!("{}, {}", index +1, line);
        }*/
        let file = fs::read_to_string(filename).unwrap_or(String::from("Failed to find file"));
        println!("File {} Contents {}", filename, file);
        MapComponent {
            area
        }
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

struct PlayerComponent {
    name: String,
}

impl PlayerComponent {
    fn new(input: &str) -> Self {
        PlayerComponent {
            name: String::from(input),
        }
    }
}

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
    command_vec.extend(buffer.split_ascii_whitespace());

    command_vec
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
        let un = location.as_ref().unwrap();
        un.print_location();
    }
}

fn update_player_system(world: & World, command_vec: &Vec<&str>) {
    let mut players = world.borrow_component_mut::<PlayerComponent>().unwrap();
    let mut locations = world.borrow_component_mut::<LocationComponent>().unwrap(); 
    

    let zip = players.iter_mut().zip(locations.iter_mut());
    let iter = zip.filter_map(|(player, location)| Some((player.as_mut()?, location.as_mut()?)));

    for (player, mut location) in iter {
        print!("Player {} ", player.name);
        handle_player_commands((&player, &mut location), command_vec);
    }

}

fn handle_player_commands(player: (&PlayerComponent, &mut LocationComponent), command_vec: &Vec<&str>) {
    if command_vec.is_empty() {
        return;
    }
    let mut iter = command_vec.iter();

    let command = Command::from_str(iter.next().unwrap_or(&"Failed to find next entry in vector"));
    match command {
        Ok(Command::Move) => {
            let dir_wrapped = Direction::from_str(iter.next().unwrap_or(&"Failed to find next entry in vector"));
            if let Ok(dir) = dir_wrapped {
                update_location(player.1, dir);
            }
        },
        Ok(Command::Check) => {

        }
        Ok(Command::Use) => {

        }
        Err(e) => {
            println!("Error bad input: {}", e);
        }
    }
}

fn update_location(location: &mut LocationComponent, dir: Direction) {
    match dir {
        Direction::Forward => location.move_forward(),
        Direction::Right => location.move_right(),
        Direction::Left => location.move_left(),
        Direction::Back => location.move_back(),
    }
}

fn main() {
    // Setup Initial Variables outside of main loop
    println!("Starting the basic ECS implementation");
    let mut buffer = String::new();

    let mut world = World::new();
    let player_entity = world.new_entity();
    world.add_component_to_entity(player_entity, PlayerComponent::new("Jakob"));
    world.add_component_to_entity(player_entity, LocationComponent{x: 0, y: 0});
    world.add_component_to_entity(player_entity, MapComponent::new("src/player_map.txt"));
    std::process::exit(0);

    let second_location = world.new_entity();
    world.add_component_to_entity(second_location, LocationComponent{x: 0, y: 0});

    loop {
        let command_vec = input_system(&mut buffer);
        update_player_system(&world, &command_vec);
        print_location_system(&world);
    }


}
