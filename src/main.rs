extern crate connect4;
extern crate idata;
// extern crate stdweb;
extern crate yew;

use connect4::Model;
use yew::prelude::*;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}

//  ----------------------------
//  ----------------------------
//  ----------------------------

// extern crate connect4;

// use connect4::engine::minmax;
// use connect4::engine::patterns::PatternsCountPlayerPonderation as PCPP;
// use connect4::engine::patterns::PatternsCountPonderation as PCP;
// use connect4::engine::*;

// const HALF_GAMES_PER_MATCH: u32 = 2;
// const DEPTH: u8 = 3;
// const PCPP_INIT_CONF: PCPP = PCPP {
//     player_current: PCP {
//         next_move_wins: 1.0,
//         imposible_avoid: 10.0,
//         vert_consecutive_hole_3inline: 5.0,
//         line3: 1.0,
//         line2: 0.1,
//         line1: 0.01,
//     },
//     player_other: PCP {
//         next_move_wins: 100.0,
//         imposible_avoid: 10.0,
//         vert_consecutive_hole_3inline: 5.0,
//         line3: 1.0,
//         line2: 0.1,
//         line1: 0.01,
//     },
// };
// // const PCPP_INIT_CONF: PCPP = PCPP {
// //     player_current: PCP {
// //         next_move_wins: 0.0,
// //         imposible_avoid: 0.0,
// //         vert_consecutive_hole_3inline: 0.0,
// //         line3: 0.0,
// //         line2: 0.0,
// //         line1: 0.0,
// //     },
// //     player_other: PCP {
// //         next_move_wins: 0.0,
// //         imposible_avoid: 0.0,
// //         vert_consecutive_hole_3inline: 0.0,
// //         line3: 0.0,
// //         line2: 0.0,
// //         line1: 0.0,
// //     },
// // };

// fn main() {
//     let pcpp_winner = PCPP_INIT_CONF;

//     let rand_pcpp = || {
//         let rand_pcp = || PCP {
//             next_move_wins: f64::from(rand::thread_rng().gen_range(0, 10_000)) / 100.0,
//             imposible_avoid: f64::from(rand::thread_rng().gen_range(0, 10_000)) / 100.0,
//             vert_consecutive_hole_3inline: f64::from(rand::thread_rng().gen_range(0, 10_000))
//                 / 100.0,
//             line3: f64::from(rand::thread_rng().gen_range(0, 10_000)) / 100.0,
//             line2: f64::from(rand::thread_rng().gen_range(0, 10_000)) / 100.0,
//             line1: f64::from(rand::thread_rng().gen_range(0, 10_000)) / 100.0,
//         };
//         PCPP {
//             player_current: rand_pcp(),
//             player_other: rand_pcp(),
//         }
//     };
//     // let rand_pcpp = || {
//     //     let mut result = PCPP_INIT_CONF;
//     //     result.player_current.next_move_wins =
//     //         f64::from(rand::thread_rng().gen_range(0, 10_000)) / 100.0 - 50.0;
//     //     result.player_other.next_move_wins =
//     //         f64::from(rand::thread_rng().gen_range(0, 10_000)) / 100.0 - 50.0;
//     //     result
//     // };

//     use rand::Rng;
//     use std::io;
//     use std::io::prelude::Write;
//     let mut pcpp_winner_pos = (pcpp_winner, 0);
//     // println!("    >> provisional winner: {:#?}", pcpp_winner_pos.0);
//     for i in 0..10_000_000 {
//         pcpp_winner_pos = play_mini_match(pcpp_winner_pos.0, rand_pcpp());
//         // pcpp_winner_pos = play_mini_match(pcpp_winner_pos.0, pcpp_loser.clone());
//         if pcpp_winner_pos.1 < 0 {
//             println!(
//                 " <{}>   ({})>> provisional winner: {:#?}",
//                 pcpp_winner_pos.1, i, pcpp_winner_pos.0
//             );
//             panic!("stoped");
//         } else if pcpp_winner_pos.1 == 0 {
//             print!("=");
//             io::stdout().flush().expect("Could not flush stdout");
//         } else {
//             print!(".");
//             io::stdout().flush().expect("Could not flush stdout");
//         }
//         // println!("    >> provisional winner: {:#?}", pcpp_winner_pos.0);
//     }
//     println!("winner: {:?}", pcpp_winner_pos.0);
// }

// #[derive(Debug)]
// enum PccpWinner {
//     First,
//     Second,
//     Draw,
// }

// fn play_mini_match(pcpp1: PCPP, pcpp2: PCPP) -> (PCPP, i64) {
//     let (won_pcpp1, won_pcpp2) = play_half_match(&pcpp1, &pcpp2, (0, 0));
//     let (won_pcpp2, won_pcpp1) = play_half_match(&pcpp2, &pcpp1, (won_pcpp2, won_pcpp1));

//     // let won_pcpp1 = won_pcpp1 + HALF_GAMES_PER_MATCH / 2;
//     if won_pcpp1 < won_pcpp2 {
//         //     println!("_____{} , {}   ~ {}", won_pcpp1, won_pcpp2, won_pcpp1 + 5 < won_pcpp2);
//         // if won_pcpp1 + 5 < won_pcpp2 {
//         (pcpp2, i64::from(won_pcpp1) - i64::from(won_pcpp2))
//     } else {
//         (pcpp1, i64::from(won_pcpp1) - i64::from(won_pcpp2))
//     }
// }

// fn play_half_match(pcpp1: &PCPP, pcpp2: &PCPP, wins: (u32, u32)) -> (u32, u32) {
//     let (mut won_ppc1, mut won_ppc2) = wins;

//     for _ in 0..HALF_GAMES_PER_MATCH {
//         let r = play_game_pcpp(&pcpp1, &pcpp2);
//         match r {
//             PccpWinner::First => won_ppc1 += 1,
//             PccpWinner::Second => won_ppc2 += 1,
//             _ => (),
//         }
//     }

//     (won_ppc1, won_ppc2)
// }

// fn play_game_pcpp(pcpp1: &PCPP, pcpp2: &PCPP) -> PccpWinner {
//     let game1 = Game::new(Player::O).set_patterns_pond(pcpp1.clone());
//     let game2 = Game::new(Player::O).set_patterns_pond(pcpp2.clone());

//     fn rec_move(game1: Game, game2: Game) -> (Game, Game) {
//         match minmax::get_best_move(game1, DEPTH) {
//             Ok((game1, col, _eval)) => {
//                 let game1 = game1.try_play(col);
//                 let game2 = game2.try_play(col);
//                 rec_move(game2, game1)
//             }
//             Err(game1) => (game1, game2),
//         }
//     }

//     let (game1, _game2) = rec_move(game1, game2);

//     match (game1.moves.len() > 30, game1.turn) {
//         (true, _) => (),
//         (false, Turn::F(Finished::Won(Player::O))) => {
//             println!("moves: {:?}\n{}", game1.moves, game1.board)
//         }
//         (false, Turn::F(Finished::Won(Player::X))) => {
//             println!("moves: {:?}\n{}", game1.moves, game1.board)
//         }
//         (false, Turn::F(Finished::Draw(_))) => (),
//         _ => unreachable!(),
//     };

//     match (game1.moves.len() > 30, game1.turn) {
//         (true, _) => PccpWinner::Draw,
//         (false, Turn::F(Finished::Won(Player::O))) => PccpWinner::First,
//         (false, Turn::F(Finished::Won(Player::X))) => PccpWinner::Second,
//         (false, Turn::F(Finished::Draw(_))) => PccpWinner::Draw,
//         _ => unreachable!(),
//     }
// }

// //  ------------------------------------------
// //  ------------------------------------------
// //  ------------------------------------------
// //  ------------------------------------------
// //  ------------------------------------------

// // extern crate connect4;

// // use connect4::engine::minmax;
// // use connect4::engine::patterns::PatternsCountPlayerPonderation as PCPP;
// // use connect4::engine::patterns::PatternsCountPonderation as PCP;
// // use connect4::engine::*;

// // const DEPTH: u8 = 2;
// // const PCPP_INIT_CONF: PCPP = PCPP {
// //     player_current: PCP {
// //         next_move_wins: 1.0,
// //         imposible_avoid: 10.0,
// //         vert_consecutive_hole_3inline: 5.0,
// //         line3: 1.0,
// //         line2: 0.1,
// //         line1: 0.01,
// //     },
// //     player_other: PCP {
// //         next_move_wins: 100.0,
// //         imposible_avoid: 10.0,
// //         vert_consecutive_hole_3inline: 5.0,
// //         line3: 1.0,
// //         line2: 0.1,
// //         line1: 0.01,
// //     },
// // };

// // fn main() {
// //     let play_col = |game: Game, col| -> Game {
// //         match game.play(Col::b(col).unwrap()) {
// //             Ok(game) => game,
// //             _ => panic!("error processing move"),
// //         }
// //     };

// //     let game = Game::new(Player::O);
// //     let game = play_col(game, 3);
// //     let game = play_col(game, 3);
// //     let game = play_col(game, 2);
// //     let game = play_col(game, 4);
// //     let game = play_col(game, 3);
// //     let game = play_col(game, 4);
// //     let game = play_col(game, 3);
// //     let game = play_col(game, 2);
// //     let game = play_col(game, 2);
// //     let game = play_col(game, 2);
// //     let game = play_col(game, 0);
// //     let game = play_col(game, 0);

// //     println!("{}", game);

// //     match minmax::get_best_move(game, DEPTH) {
// //         Ok((_game, col, _eval)) => println!("{:?}", col),
// //         _ => (),
// //     }
// // }
