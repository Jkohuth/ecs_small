use std::io;
use std::cell::{Ref, RefCell, RefMut};

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
    Move,
    Check,
    Use,
}

impl Command {
    fn from_str(s: &str) -> Result<Command, ()> {
        match s {
            "move" => Ok(Command::Move),
            "check" => Ok(Command::Check),
            "use" => Ok(Command::Use),
            _ => Err(()),
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
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}
fn print_type_of_with_message<T>(message: &str, _: &T) {
    println!("{}: {}",message, std::any::type_name::<T>());
}

struct PlayerComponent {
    name: String,
}

impl PlayerComponent {
    fn new() -> Self {
        PlayerComponent {
            name: String::from("Player"),
        }
    }
}

fn input_system(buffer: &mut String) {
    get_input(buffer);
    process_string(buffer);
}

fn get_input(buffer: &mut String) {
    io::Write::flush(&mut io::stdout());

    io::stdin().read_line(buffer);

    //buffer.clear();
    //buffer.to_string()
}

fn process_string(buffer: &mut String) {
    if buffer.to_lowercase().contains("exit") {
        std::process::exit(0);
    }

    for iter in buffer.split_ascii_whitespace() {
        let result = Command::from_str(&iter).unwrap();
        println!("Result {:?}", result);
    }

    // Need to have a list of different strings that process string can reference
    // Commands like "move" "check" "push" and have this process call the requisite system

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

fn update_player_location_system(world: & World, buffer: &String) {
    let mut players = world.borrow_component_mut::<PlayerComponent>().unwrap();
    let mut locations = world.borrow_component_mut::<LocationComponent>().unwrap(); 
    

    let zip = players.iter_mut().zip(locations.iter_mut());
    let iter = zip.filter_map(|(player, location)| Some((player.as_mut()?, location.as_mut()?)));

    for (player, location) in iter {

    }

}

fn update_location_system(world: & World, buffer: &String) {

    let mut borrow_location_wrapped = world.borrow_component_mut::<LocationComponent>();

    // Need to make a trait that will check and confirm if it is unwrapped and return 
    // the internal vector
    if borrow_location_wrapped.is_none() {
        println!("LocationComponent is none");
        std::process::exit(1);
    }

    let mut location_ref = borrow_location_wrapped.unwrap();
    for location_wrapped in location_ref.iter_mut() {
        print_type_of_with_message("location_wrapped",&location_wrapped);

        let mut location_unwrapped = location_wrapped.as_mut().unwrap();
        print_type_of_with_message("location_unwrapped",&location_unwrapped);

        // brute force for now
        if buffer.to_lowercase().contains("right") {
            location_unwrapped.move_right();
        } else if buffer.to_lowercase().contains("left") {
            location_unwrapped.move_left();
        } else if buffer.to_lowercase().contains("forward") {
            location_unwrapped.move_forward();
        } else if buffer.to_lowercase().contains("back") {
            location_unwrapped.move_back();
        }
    }

}

fn main() {
    // Setup Initial Variables outside of main loop
    println!("Starting the basic ECS implementation");
    let mut buffer = String::new();

    let mut world = World::new();
    let player_entity = world.new_entity();
    world.add_component_to_entity(player_entity, PlayerComponent::new());
    world.add_component_to_entity(player_entity, LocationComponent{x: 0, y: 0});

    loop {

        input_system(&mut buffer);
        update_player_location_system(&world, &buffer);
        print_location_system(&world);
        // handle input
        // run systems (that edit game state)
    
        //std::process::exit(0);
    }


}
