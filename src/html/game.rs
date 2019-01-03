use crate::engine;
use crate::{idata, yew, Config, ConfigPlayers};
use yew::prelude::*;
// use yew::services::ConsoleService;

const DEPTH: u8 = 2;

pub struct Model {
    // console: ConsoleService,
    game: engine::Game,
    config: Config,
}

pub enum Msg {
    Click(u8),
    A(MsgAnalisys),
}

pub enum MsgAnalisys {
    MoveBack,
    ComputerPlay,
}

#[derive(PartialEq, Clone)]
pub struct Properties {
    pub config: Config,
    // pub onnewgame: Option<Callback<(Config)>>,
}

//  ----------

impl Default for Properties {
    fn default() -> Self {
        Properties {
            config: Config::init(),
        }
    }
}

impl Model {}

impl Component for Model {
    type Message = Msg;
    type Properties = Properties;

    fn create(p: Self::Properties, _: ComponentLink<Self>) -> Self {
        let game = engine::Game::new(p.config.start);
        let config = p.config;
        let game = move_computer_if_turn(game, &config);
        Model { game, config }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let computer_move = |game: &mut engine::Game, depth| {
            idata::steal_borrow(game, &|s: engine::Game| match computer_play(s, depth) {
                Ok(game) => game,
                Err(game) => game,
            })
        };
        let try_play = |game: &mut engine::Game, c, config: &Config| {
            if let Some(col) = engine::Col::b(c) {
                idata::steal_borrow(game, &|s: engine::Game| match s.play(col) {
                    Ok(game) => move_computer_if_turn(game, config),
                    Err(game) => game,
                })
            }
        };
        let move_back = |game: &mut engine::Game| {
            idata::steal_borrow(game, &|s: engine::Game| match s.undo() {
                Ok(game) => game,
                Err(game) => game,
            })
        };

        match msg {
            Msg::Click(col) => try_play(&mut self.game, col, &self.config),
            Msg::A(MsgAnalisys::ComputerPlay) => computer_move(&mut self.game, DEPTH),
            Msg::A(MsgAnalisys::MoveBack) => move_back(&mut self.game),
        };

        true
    }
    fn change(&mut self, Self::Properties { config }: Self::Properties) -> ShouldRender {
        if config != self.config {
            self.config = config;
            true
        } else {
            false
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let get_color_cell = |r, c| -> &str {
            let get_color_rc = |r, c| match self.game.board.get_cell(c, r) {
                engine::Cell::P(engine::Player::O) => "yellow",
                engine::Cell::P(engine::Player::X) => "red",
                engine::Cell::Empty => "",
            };

            match (engine::Row::b(r), engine::Col::b(c)) {
                (Some(r), Some(c)) => get_color_rc(r, c),
                _ => "",
            }
        };

        let td = |r, c| {
            html! {
                <>
                <td class=format!("td_board {}", get_color_cell(r,c)), onclick=|_| Msg::Click(c),></td>
                </>
            }
        };

        let row = |r| {
            html! {
                <tr>
                    {td(r,0)}
                    {td(r,1)}
                    {td(r,2)}
                    {td(r,3)}
                    {td(r,4)}
                    {td(r,5)}
                    {td(r,6)}
                </tr>
            }
        };

        let board = || {
            html! {
                <table class="board",>
                    {row(0)}
                    {row(1)}
                    {row(2)}
                    {row(3)}
                    {row(4)}
                    {row(5)}
                </table>
            }
        };

        let pattern_count = || {
            let get_pos_eval = || format!("{:?}", self.game.eval());
            let get_pattern_debug_txt = || match &self.game.patterns {
                crate::engine::patterns::Patterns::P(pcp) => {
                    (format!("{:?}", pcp.player_o), format!("{:?}", pcp.player_x))
                }
                _ => ("".to_string(), "".to_string()),
            };
            html! {
                <>
                <p>{get_pos_eval()}</p>
                <p>{get_pattern_debug_txt().0}</p>
                <p>{get_pattern_debug_txt().1}</p>
                </>
            }
        };

        let moves = || {
            let moves_string = || -> String {
                self.game
                    .moves
                    .iter()
                    .fold("".to_string(), |acc, c| format!("{} {}", acc, c))
            };
            html! {
                <>
                <p>{moves_string()}</p>
                </>
            }
        };
        let analisys = || {
            if self.config.players == ConfigPlayers::Analisys {
                html! {
                    <>
                    <p>
                        <button onclick=|_| Msg::A(MsgAnalisys::MoveBack),>{"back"}</button>
                        <button onclick=|_| Msg::A(MsgAnalisys::ComputerPlay),>{"computer move"}</button>
                    </p>
                    </>
                }
            } else {
                html! {
                    <>
                    </>
                }
            }
        };

        html! {
            <div class="game",>
                {board()}
            </div>{analisys()}</div>
            </div>{moves()}</div>
            </div>{pattern_count()}</div>
            </div>
        }
    }
}

fn move_computer_if_turn(game: engine::Game, config: &Config) -> engine::Game {
    let finished_game = match game.turn {
        engine::Turn::P(_) => false,
        engine::Turn::F(_) => true,
    };

    let rgame = if let ConfigPlayers::CMachine(mp) = config.players {
        if (game.moves.len() % 2 == 0) == (config.start == mp) && !finished_game {
            computer_play(game, DEPTH)
        } else {
            Ok(game)
        }
    } else {
        Ok(game)
    };

    match rgame {
        Ok(game) => game,
        Err(_) => unreachable!(),
    }
}

fn computer_play(game: engine::Game, depth: u8) -> std::result::Result<engine::Game, engine::Game> {
    let (game, best_move, _eval) = crate::engine::minmax::get_best_move(game, depth)?;
    game.play(best_move)
}
