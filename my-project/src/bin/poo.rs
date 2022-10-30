fn main() {
    let screen = Screen {
        componentes: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("hola"),
                    String::from("adios"),
                    String::from("no"),
                ],
            }),
            Box::new(Button {
                width: 22,
                height: 23,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub componentes : Vec <Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.componentes.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("draw a button {}x{} {}.", self.width, self.height, self.label);
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw (&self) {
        println!("draw a selectBox");
    }
}
