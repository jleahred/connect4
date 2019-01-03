use crate::engine;
use yew::prelude::*;
// use yew::services::ConsoleService;
use crate::{Config, ConfigPlayers};

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
        let mut computer_color_change = || match self.config.players {
            ConfigPlayers::CMachine(cp) => {
                self.config.players = ConfigPlayers::CMachine(switch_player(cp))
            }
            _ => unreachable!(),
        };
        match msg {
            Msg::ChangeFirstPlayer => self.config.start = switch_player(self.config.start),
            Msg::PlayersComputerPlays => {
                self.config.players = ConfigPlayers::CMachine(engine::Player::O)
            }
            Msg::PlayersTwoHumans => self.config.players = ConfigPlayers::TwoPlayers,
            Msg::PlayersAnalisys => self.config.players = ConfigPlayers::Analisys,
            Msg::ComputerColorChange => computer_color_change(),
            Msg::StartGame => {
                if let Some(ref mut callback) = self.onstart {
                    callback.emit(self.config.clone())
                }
            }
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
            let machine_plays = || {
                use std::mem::discriminant;
                discriminant(&ConfigPlayers::CMachine(engine::Player::O))
                    == discriminant(&self.config.players)
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
                    <option value="analisys", selected={self.config.players == ConfigPlayers::Analisys}, onclick=|_| Msg::PlayersAnalisys,>{"analisys"}</option>
                    </select>
                </td>
                </>
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
                        if let ConfigPlayers::CMachine(cp) = self.config.players {
                            computer_plays(cp)
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
