use iced::{button, Button, Column, Element, Sandbox, Container, Settings, Text, Image};
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
    decrement_button : button::State,
    increment_button : button::State,
    new_map_button: button::State,
    pub iced_room_gen: IcedRoomGenerator,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
    NewMapPressed,
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
            decrement_button: button::State::new(),
            increment_button: button::State::new(),
            new_map_button: button::State::new(),
            iced_room_gen: IcedRoomGenerator::new()
        }
    }

    fn title(&self) -> String {
        format!("{} - Iced", "Cocaine")
    }

    fn update(&mut self, event: Message) {
        match event {
            Message::IncrementPressed => {
                self.value+=1;
            }
            Message::DecrementPressed => {
                self.value-=1;
            }
            Message::NewMapPressed => {
                let seed = create_hash(&thread_rng().sample_iter(&Alphanumeric).take(32).collect::<String>());
                let seed_u8 = array_ref!( seed.as_bytes(), 0, 32);
                let mut rng = SeedableRng::from_seed(  *seed_u8);
                let level = BspLevel::new(self.iced_room_gen.board_width, self.iced_room_gen.board_height, &seed, &mut rng);

                println!("{}", level);
                self.value += 1;
                draw(&level, "img", &format!("level{:x}", self.value)[..]).unwrap();
                ()
            }
        }
    }

    fn view(&mut self) -> Element<Message> {

        Column::new().padding(20)
            .push(
                Button::new(&mut self.increment_button, Text::new("Increment"))
                    .on_press(Message::IncrementPressed),
            )
            .push(Text::new(self.value.to_string()).size(50))
            .push(
                Button::new(&mut self.decrement_button, Text::new("Decrement"))
                    .on_press(Message::DecrementPressed),
            )
            .push(
                Button::new(&mut self.new_map_button, Text::new("New Map"))
                    .on_press(Message::NewMapPressed),
            )
            .push(Container::new(Image::new(format!("img/level{:x}.png", self.value))))
            .into()
        //let mut controls = Row::new();
    }
}
