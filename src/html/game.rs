use crate::engine;
use crate::{idata, yew, Config, ConfigPlayers, Level};
use std::time::Duration;
use yew::prelude::*;
// use yew::services::ConsoleService;
use yew::services::{Task, TimeoutService};

pub struct Model {
    // console: ConsoleService,
    game: engine::Game,
    config: Config,

    timeout: TimeoutService,
    job: Option<Box<Task>>,
    on_new_game: Option<Callback<()>>,
    callback_comp_move: Callback<()>,
}

pub enum Msg {
    Click(u8),
    A(MsgAnalisys),
    NewGame,
    ComputerMove,
}

pub enum MsgAnalisys {
    MoveBack,
    ComputerPlay,
}

#[derive(PartialEq, Clone)]
pub struct Properties {
    pub config: Config,
    pub on_new_game: Option<Callback<()>>,
}

//  ----------

impl Default for Properties {
    fn default() -> Self {
        Properties {
            config: Config::init(),
            on_new_game: None,
        }
    }
}

impl Model {
    fn program_computer_move(mut self) -> Self {
        {
            let handle = self
                .timeout
                .spawn(Duration::from_millis(100), self.callback_comp_move.clone());
            self.job = Some(Box::new(handle));
        }
        self
    }

    fn get_depth(&self) -> u8 {
        let level = match self.config.players {
            ConfigPlayers::CMachine(ref cm) => cm.level,
            _ => Level::Easy,
        };
        match level {
            Level::Easy => 4,
            Level::WeakMind => 4,
            Level::Mosquito => 3,
        }
    }
}
impl Component for Model {
    type Message = Msg;
    type Properties = Properties;

    fn create(p: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let game = engine::Game::new(p.config.start);
        let config = p.config;
        let result = Model {
            game,
            config,
            timeout: TimeoutService::new(),
            job: None,
            on_new_game: p.on_new_game,
            callback_comp_move: link.send_back(|_| Msg::ComputerMove),
        };
        if is_computer_turn(&result.game, &result.config) {
            result.program_computer_move()
        } else {
            result
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let computer_move = |game: &mut engine::Game, depth| {
            idata::steal_borrow(game, &|s: engine::Game| match computer_play(s, depth) {
                Ok(game) => game,
                Err(game) => game,
            })
        };
        let try_play = |game: &mut engine::Game, c| {
            if let Some(col) = engine::Col::b(c) {
                idata::steal_borrow(game, &|s: engine::Game| match s.play(col) {
                    Ok(game) => game,
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

        let depth = self.get_depth();
        match msg {
            Msg::Click(col) => {
                try_play(&mut self.game, col);
                if is_computer_turn(&self.game, &self.config) {
                    idata::steal_borrow(self, &|s: Self| s.program_computer_move())
                }
            }
            Msg::A(MsgAnalisys::ComputerPlay) => computer_move(&mut self.game, depth),
            Msg::A(MsgAnalisys::MoveBack) => move_back(&mut self.game),
            Msg::NewGame => {
                if let Some(ref mut callback) = self.on_new_game {
                    callback.emit(())
                }
            }
            Msg::ComputerMove => computer_move(&mut self.game, depth),
        };

        true
    }
    fn change(&mut self, Self::Properties { config, .. }: Self::Properties) -> ShouldRender {
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
                // <p>{format!("{:?}", self.config.players)}</p>
                </>
            }
        };
        let analisys = || {
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
            if self.config.players == ConfigPlayers::Analisys {
                html! {
                    <>
                    <p>
                        <button onclick=|_| Msg::A(MsgAnalisys::MoveBack),>{"back"}</button>
                        <button onclick=|_| Msg::A(MsgAnalisys::ComputerPlay),>{"computer move"}</button>
                    </p>
                    </div>{pattern_count()}</div>
                    </>
                }
            } else {
                html! {<></>}
            }
        };

        let winner = || match self.game.turn {
            crate::engine::Turn::F(_) => html! {
                <>
                <p>
                    <div>{"Game over!!!"}</div>
                    <button onclick=|_| Msg::NewGame,>{"new game"}</button>
                </p>
                </>
            },
            _ => html! {<></>},
        };

        html! {
            <div class="game",>
            </div>{winner()}</div>
                {board()}
            </div>{analisys()}</div>
            </div>{moves()}</div>
            </div>
        }
    }
}

fn is_computer_turn(game: &engine::Game, config: &Config) -> bool {
    let finished_game = match game.turn {
        engine::Turn::P(_) => false,
        engine::Turn::F(_) => true,
    };

    if let ConfigPlayers::CMachine(ref mc) = config.players {
        (game.moves.len() % 2 == 0) == (config.start == mc.machine_player) && !finished_game
    } else {
        false
    }
}

fn computer_play(game: engine::Game, depth: u8) -> std::result::Result<engine::Game, engine::Game> {
    let (game, best_move, _eval) = crate::engine::minmax::get_best_move(game, depth)?;
    game.play(best_move)
}
