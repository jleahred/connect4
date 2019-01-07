use crate::engine;
use yew::prelude::*;
// use yew::services::ConsoleService;
use crate::{Config, ConfigMachine, ConfigPlayers, Level};

#[derive(PartialEq, Clone)]
pub struct Model {
    pub config: Config,
    pub onstart: Option<Callback<(Config)>>,
}

pub enum Msg {
    ChangeFirstPlayer,
    PlayersComputerPlays,
    PlayersTwoHumans,
    PlayersAnalisys,
    ComputerColorChange,
    StartGame,
    L(Level),
}

#[derive(PartialEq, Clone)]
pub struct Properties {
    // pub model: Model,
    pub onstart: Option<Callback<(Config)>>,
}

//  ----------

impl Default for Properties {
    fn default() -> Self {
        Properties { onstart: None }
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = Properties;

    fn create(p: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            config: Config::init(),
            onstart: p.onstart,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let mut computer_color_change = || match &self.config.players {
            ConfigPlayers::CMachine(cm) => {
                self.config.players = ConfigPlayers::CMachine(ConfigMachine {
                    machine_player: switch_player(cm.machine_player),
                    level: cm.level,
                })
            }
            _ => unreachable!(),
        };
        match msg {
            Msg::ChangeFirstPlayer => self.config.start = switch_player(self.config.start),
            Msg::PlayersComputerPlays => {
                self.config.players = ConfigPlayers::CMachine(ConfigMachine {
                    machine_player: engine::Player::O,
                    level: Level::Mosquito,
                })
            }
            Msg::PlayersTwoHumans => self.config.players = ConfigPlayers::TwoPlayers,
            Msg::PlayersAnalisys => self.config.players = ConfigPlayers::Analisys,
            Msg::ComputerColorChange => computer_color_change(),
            Msg::StartGame => {
                if let Some(ref mut callback) = self.onstart {
                    callback.emit(self.config.clone())
                }
            }
            Msg::L(level) => match self.config.players {
                ConfigPlayers::CMachine(ref mut cm) => cm.level = level,
                _ => unreachable!(),
            },
        }
        true
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let get_color_player = |player| -> &str {
            match player {
                engine::Player::O => "yellow",
                engine::Player::X => "red",
            }
        };
        let start_player = || {
            html! {
                <>
                <td> {"first move: "} </td>
                <td class=format!("select_color {}", get_color_player(self.config.start)), onclick=|_| Msg::ChangeFirstPlayer,></td>
                </>
            }
        };
        let option_players = || {
            let machine_plays = || match self.config.players {
                ConfigPlayers::CMachine(_) => true,
                _ => false,
            };
            html! {
                <>
                <td>
                    {"players: "}
                </td>
                <td>
                    <select>
                    <option value="I play", selected={machine_plays()}, onclick=|_| Msg::PlayersComputerPlays,>{"I play"}</option>
                    <option value="two players", selected={self.config.players == ConfigPlayers::TwoPlayers}, onclick=|_| Msg::PlayersTwoHumans,>{"two players"}</option>
                    // <option value="analisys", selected={self.config.players == ConfigPlayers::Analisys}, onclick=|_| Msg::PlayersAnalisys,>{"analisys"}</option>
                    </select>
                </td>
                </>
            }
        };
        let option_level = || {
            let opmachine_level = match self.config.players {
                ConfigPlayers::CMachine(ref cm) => Some(cm.level),
                _ => None,
            };
            match opmachine_level {
                Some(level) => html! {
                    <>
                    <td>
                        {"level: "}
                    </td>
                    <td>
                        <select>
                        <option value="Mosquito's brain", selected={level == Level::Mosquito}, onclick=|_| Msg::L(Level::Mosquito),>{"Mosquito's brain"}</option>
                        <option value="Weak mind", selected={level == Level::WeakMind}, onclick=|_| Msg::L(Level::WeakMind),>{"Weak mind"}</option>
                        <option value="just easy", selected={level == Level::Easy}, onclick=|_| Msg::L(Level::Easy),>{"just easy"}</option>
                        </select>
                    </td>
                    </>
                },
                None => {
                    html! { <></> }
                }
            }
        };
        let computer_color = || {
            let computer_plays = |cplayer: engine::Player| {
                html! {
                    <>
                    <tr>
                        <td>{"Me:"}</td>
                        <td class=format!("select_color {}", get_color_player(cplayer)), onclick=|_| Msg::ComputerColorChange,></td>
                    </tr>
                    <tr>
                        <td>{"You:"}</td>
                        <td class=format!("select_color {}", get_color_player(switch_player(cplayer))), onclick=|_| Msg::ComputerColorChange,></td>
                    </tr>
                    </>
                }
            };

            html! {
                <>
                    {
                        if let ConfigPlayers::CMachine(ref cm) = self.config.players {
                            computer_plays(cm.machine_player)
                        } else {
                            html!{<div></div>}
                        }
                    }
                </>
            }
        };

        html! {
            <div>
            <h2>{"Config:"}</h2>
            <table align="center",>
                <tr>{start_player()}</tr>
                <tr>{option_players()}</tr>
                <tr>{option_level()}</tr>
                {computer_color()}
            </table>
                <p><button onclick=|_| Msg::StartGame,>{"start game"}</button></p>
            </div>
        }
    }
}

fn switch_player(player: engine::Player) -> engine::Player {
    match player {
        engine::Player::O => engine::Player::X,
        engine::Player::X => engine::Player::O,
    }
}
