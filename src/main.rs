use std::io;
use std::cell::{Ref, RefCell, RefMut};

trait Component {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn push_none(&mut self);
}

impl<T: 'static> Component for Vec<Option<T>> {
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }

    fn push_none(&mut self) {
        self.push(None)
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
                println!("Component Exists");
                component_vec.borrow_mut()[entity] = Some(component);
                return;
            }
        }
        let mut new_component: Vec<Option<ComponentType>> = Vec::with_capacity(self.entities_count);
        println!("Entities Count {}", self.entities_count);

        for _ in 0..self.entities_count {
            new_component.push(None);
        }

        new_component[entity] = Some(component);

        print_type_of(&new_component);

        self.components.push(Box::new(new_component));
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
            print_type_of(&component_vec);
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


struct LocationComponent {
    x: u32,
    y: u32,
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
        print!("x: {}, y: {}", self.x, self.y);
    }
}
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}
struct PlayerComponent {
    location_component: LocationComponent,
    name: String,
}

impl PlayerComponent {
    fn new() -> Self {
        PlayerComponent {
            location_component: LocationComponent {
                x: 0,
                y: 0,
            },
            name: String::from("Player"),
        }
    }
}

fn input_system(buffer: &mut String) {
    get_input(buffer);
    process_string(buffer);
    println!("Buffer {}", buffer);
}

fn get_input(buffer: &mut String) {
    io::Write::flush(&mut io::stdout());

    io::stdin().read_line(buffer);
    print!("In function {}", buffer);

    //buffer.clear();
    //buffer.to_string()
}

fn process_string(buffer: &mut String) {
    if !buffer.to_lowercase().contains("move") {
        println!("Not the correct string");
    }

}

fn print_location_system(world: &World) {
    //let location_ref: RefMut<Vec<Option<LocationComponent>>> = world.borrow_component_mut::<LocationComponent>().unwrap_or(panic!("Component is none"));
    if world.borrow_component::<LocationComponent>().is_none() {
        println!("Unwrapped it is none");
        std::process::exit(1);
    }
    /*let location_iter = location_ref.iter();

    for location in location_iter {
        let un = location.as_ref().unwrap();
        un.print_location();
    }*/

}

fn main() {
    // Setup Initial Variables outside of main loop
    println!("Starting the basic ECS implementation");
    let mut buffer = String::new();

    let mut world = World::new();
    let location_entity = world.new_entity();
    world.add_component_to_entity(location_entity, LocationComponent{x: 0, y: 0});

    loop {

        //input_system(&mut buffer);
        print_location_system(&world);
        // handle input
        // run systems (that edit game state)
    
        println!("Buffer value after edits {}", buffer);
    
    }


}
