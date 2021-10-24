use std::io;

struct LocationComponent {
    x: u32,
    y: u32,
 }

impl LocationComponent {
    fn moveForward(&mut self) {
        self.y += 1;
    }
    fn moveBack(&mut self) {
        self.y -= 1;
    }

    fn moveRight(&mut self) {
        self.x += 1;
    }

    fn moveLeft(&mut self) {
        self.x -= 1;
    }

    fn printLocation(&self) {
        print!("x: {}, y: {}", self.x, self.y);
    }
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


fn getInput(buffer: &mut String) {
    io::Write::flush(&mut io::stdout());

    io::stdin().read_line(buffer);
    print!("In function {}", buffer);

    //buffer.clear();
    //buffer.to_string()
}

fn processString(buffer: &mut String) {
    if !buffer.to_lowercase().contains("move") {
        panic!("Bad input fail");
    }

}

fn main() {
    println!("Starting the basic ECS implementation");
    let mut buffer = String::new();
    //buffer = 
    getInput(&mut buffer);
    processString(&mut buffer);

    println!("Buffer value after edits {}", buffer);


    /*
        loop {
            // handle input
            // run systems (that edit game state)
        }
    */ 


}
