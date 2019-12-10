use iced::{button, Button, Column, Row,
     Element, Sandbox, Container, Settings, Text, Image};
use rand::{rngs::StdRng, SeedableRng};
use crate::roomscorridors::RoomsCorridors;
use crate::bsp::BspLevel;
use rand::distributions::Alphanumeric;
use rand::prelude::*;
use sha2::{Digest, Sha256};
use rand;
use crate::draw::draw;
pub struct IcedRoomGenerator {
    board_width: i32,
    board_height: i32,
    seed: String,
    rng: StdRng,
}

pub struct IcedSandbox {
    value: i32,
    current_map: i32,
    decrement_button: button::State,
    increment_button: button::State,
    new_map_button: button::State,
    algo_toggle_button: button::State,
    current_algorithm: IcedAlgorithm,
    current_image: String,
    pub iced_room_gen: IcedRoomGenerator,
    force_clear: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
    NewMapPressed,
    AlgoSelection,
}

trait RoomGeneratorApplication {
    const VAL: [u8;32];

    fn new() -> Self;
    fn set_from_console(&mut self, board_width: i32, board_height: i32, seed: String, rng: StdRng);
}


impl RoomGeneratorApplication for IcedRoomGenerator
{
    const VAL : [u8;32] = [0;32];

    fn new() -> IcedRoomGenerator {
        IcedRoomGenerator {
            board_width: 48,
            board_height: 40,
            seed: 0.to_string(),
            rng: SeedableRng::from_seed(Self::VAL)
        }
    }
    fn set_from_console(&mut self, board_width: i32, board_height: i32, seed: String, rng: StdRng) {
        self.board_width = board_width;
        self.board_height = board_height;
        self.seed = seed;
        self.rng = rng;
    }

}

enum IcedAlgorithm {
    Bsp,
    Rooms
}

fn create_hash(text: &str) -> String {
    let mut hasher = Sha256::default();
    hasher.input(text.as_bytes());
    format!("{:x}", hasher.result())
}
impl Sandbox for IcedSandbox {
    type Message = Message;
    fn new() -> Self {
        IcedSandbox {
            value : 0,
            current_map: 0,
            decrement_button: button::State::new(),
            increment_button: button::State::new(),
            algo_toggle_button : button::State::new(),
            new_map_button: button::State::new(),
            iced_room_gen: IcedRoomGenerator::new(),
            current_algorithm: IcedAlgorithm::Bsp,
            current_image: format!("img/level{}.png", 0),
            force_clear: false,
        }
    }

    fn title(&self) -> String {
        format!("{} - Iced", "RoomGen")
    }

    fn update(&mut self, event: Message) {
        match event {
            Message::IncrementPressed => {
                self.current_map+=1;
                self.current_image = format!("img/level{}.png", self.current_map);
            }
            Message::DecrementPressed => {

                if self.current_map > 0
                {
                    self.current_map-=1;
                    self.current_image = format!("img/level{}.png", self.current_map);
                }
            }
            Message::NewMapPressed => {
                let seed = create_hash(&thread_rng().sample_iter(&Alphanumeric).take(32).collect::<String>());
                let seed_u8 = array_ref!( seed.as_bytes(), 0, 32);
                let mut rng = SeedableRng::from_seed(  *seed_u8);
                match self.current_algorithm {
                    IcedAlgorithm::Bsp =>
                    {
                        let level = BspLevel::new(self.iced_room_gen.board_width, self.iced_room_gen.board_height, &seed, &mut rng);

                        println!("{}", level);
                       
                        draw(&level, "img", &format!("level{}", self.value)[..]).unwrap();
                        self.current_image = format!("img/level{}.png", self.value);
                        self.value += 1;
                        self.force_clear = true;
                     
                    },
                    IcedAlgorithm::Rooms => {
                        let level = RoomsCorridors::new(self.iced_room_gen.board_width, self.iced_room_gen.board_height, &seed, &mut rng);

                        println!("{}", level);
                       
                        draw(&level, "img", &format!("level{}", self.value)[..]).unwrap();
                        self.current_image = format!("img/level{}.png", self.value);
                        self.value += 1;
                        self.force_clear = true;
                    }

                };
                ()
            }
            Message::AlgoSelection => {
                self.current_algorithm = match self.current_algorithm {
                    IcedAlgorithm::Bsp => IcedAlgorithm::Rooms,
                    IcedAlgorithm::Rooms => IcedAlgorithm::Bsp,
                };
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        //let image = self.current_image;
        println!("{}" ,self.current_image.clone());
        Column::new().padding(20)
            .push(Row::new().padding(10)
                .push(
                    Button::new(&mut self.decrement_button, Text::new("Previous Map"))
                        .on_press(Message::DecrementPressed)
                )
                .push(Text::new(self.current_map.to_string()).size(20))
                .push(
                    Button::new(&mut self.increment_button, Text::new("Next Map"))
                        .on_press(Message::IncrementPressed))
            )   
            .push(
                Button::new(&mut self.algo_toggle_button, Text::new("Algo toggle"))
                    .on_press(Message::AlgoSelection),
            )
            .push(
                Button::new(&mut self.new_map_button, Text::new("New Map"))
                    .on_press(Message::NewMapPressed),
            )
            .push(Image::new(self.current_image.clone()))
            .into()
        
    }
}
