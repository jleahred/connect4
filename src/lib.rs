extern crate stdweb;

pub mod engine;

#[macro_use]
extern crate yew;
// use yew::services::ConsoleService;

extern crate idata;

mod html;
use crate::html::config::Model as HConfig;
use crate::html::game::Model as HGame;

use yew::prelude::*;

pub enum Model {
    // console: ConsoleService,
    Config,
    Game(Config),
}

pub enum Msg {
    StartGame(Config),
    NewGame(()),
}

//  ----------

#[derive(PartialEq, Clone)]
pub struct Config {
    start: engine::Player,
    players: ConfigPlayers,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Level {
    Mosquito,
    WeakMind,
    Easy,
}

impl Config {
    fn init() -> Self {
        Config {
            start: engine::Player::O,
            players: ConfigPlayers::CMachine(ConfigMachine {
                machine_player: engine::Player::O,
                level: Level::Easy,
            }),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum ConfigPlayers {
    CMachine(ConfigMachine),
    TwoPlayers,
    Analisys,
}

#[derive(Debug, PartialEq, Clone)]
struct ConfigMachine {
    machine_player: engine::Player,
    level: Level,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model::Config
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::StartGame(cfg) => std::mem::swap(self, &mut Model::Game(cfg)),
            Msg::NewGame(()) => std::mem::swap(self, &mut Model::Config),
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let view_config_and_game = || match self {
            Model::Config => html! {
                <><HConfig:  onstart= Msg::StartGame,/></>
            },
            Model::Game(cfg) => html! {
                <><HGame: config=cfg, on_new_game= Msg::NewGame,/></>
            },
        };

        html! {
            <div><h1>{"Connect 4"}</h1></div>
            <div>{view_config_and_game()}</div>
            // <HGame:/>
        }
    }
}
