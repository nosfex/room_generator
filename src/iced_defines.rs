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
use std::fs::File;
use crate::level::Level;
use regex::Regex;
use std::process::Command;
use sedregex::{find_and_replace, ReplaceCommand};
use duct::cmd;
use std::io::prelude::*;
use std::io::BufReader;

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

pub struct AlgoSelectionState {
    current_algorithm : IcedAlgorithm,
    current_text : String
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
}

trait SaveLevelSandbox {
    fn save_level(&mut self, level :Level);
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
   
}

impl SaveLevelSandbox for IcedSandbox {
    fn save_level(&mut self, level :Level) {
        
       
        let serialised = serde_json::to_string(&level).unwrap();
        let out = format!("{}", serialised);

      
        let repl =  find_and_replace(&out, &["s/\\/ /ig"]).unwrap();
       
        let var = String::from(&repl[..]);
        draw(&level, "img", &format!("level{}", self.value)[..]).unwrap();
        let data_name = format!("json/level_data{}.json", self.value);
        let file = File::create( data_name ).unwrap();
        let mut bytes_vec = var.as_bytes().to_vec();
        // let mut i = 0;

        // while i < bytes_vec.len() {
        //     if bytes_vec[i] == 34 {
        //         bytes_vec.remove(i);
        //         bytes_vec.insert(i, r#"""#.as_bytes().to_vec()[0]);
        //     }
        //     i+=1;
        // }s
        let utf8 = String::from_utf8(bytes_vec).unwrap();
        serde_json::to_writer(file, &utf8).unwrap();        
        self.current_image = format!("img/level{}.png", self.value);

        Command::new("./clean_file.sh").arg(format!("json/level_data{}.json", self.value) );
      //  let big_cmd = cmd!("bash", "clean_file.sh", format!("level_data{}.json", self.value) );
      //  powershell -Command "(gc myFile.txt) -replace 'foo', 'bar' | Out-File -encoding ASCII myFile.txt"
        let big_cmd = cmd!("powershell", "shell.ps1" );
        let reader = big_cmd.stderr_to_stdout().reader();
        
        println!("{:?}", reader);
        self.value += 1;
        self.force_clear = true;
  
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
                let level = match self.current_algorithm {
                    IcedAlgorithm::Bsp =>
                    {
                        BspLevel::new(self.iced_room_gen.board_width, self.iced_room_gen.board_height, &seed, &mut rng)
                    },
                    IcedAlgorithm::Rooms => {
                        RoomsCorridors::new(self.iced_room_gen.board_width, self.iced_room_gen.board_height, &seed, &mut rng)
                    }
                };
                self.save_level(level);
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
