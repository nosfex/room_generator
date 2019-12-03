use iced::{button, Button, Column, Element, Sandbox, Settings, Text};
use rand::{rngs::StdRng, SeedableRng};
use crate::roomscorridors::RoomsCorridors;

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
    iced_room_gen: IcedRoomGenerator,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
    NewMapPressed,
}

trait RoomGeneratorApplication {
    const val: [u8;32];
    fn new() -> Self;
    fn set_from_console(&mut self, board_width: i32, board_height: i32, seed: String, rng: StdRng);
}


impl RoomGeneratorApplication for IcedRoomGenerator
{
    const val : [u8;32] = [1;32];
    fn new() -> IcedRoomGenerator {
        IcedRoomGenerator {
            board_width: 0,
            board_height: 0,
            seed: 0.to_string(),
            rng: SeedableRng::from_seed(Self::val)
        }
    }
    fn set_from_console(&mut self, board_width: i32, board_height: i32, seed: String, rng: StdRng) {
        self.board_width = board_width;
        self.board_height = board_height;
        self.seed = seed;
        self.rng = rng;
    }

}
impl Sandbox for IcedSandbox {
    type Message = Message;
    fn new() -> Self {
        IcedSandbox {
            value : 0,
            decrement_button: button::State::new(),
            increment_button: button::State::new(),
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
                RoomsCorridors::new(self.iced_room_gen.board_width, self.iced_room_gen.board_height, &self.iced_room_gen.seed, &mut self.iced_room_gen.rng);
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
            .into()
        //let mut controls = Row::new();
    }
}
