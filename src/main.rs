extern crate connect4;
extern crate idata;
extern crate yew;

use connect4::Model;
use yew::prelude::*;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}

// extern crate connect4;

// use connect4::engine::minmax;
// use connect4::engine::patterns::PatternsCountPlayerPonderation as PCPP;
// use connect4::engine::patterns::PatternsCountPonderation as PCP;
// use connect4::engine::*;

// const HALF_GAMES_PER_MATCH: u32 = 1;

// fn main() {
//     let pcpp_winner = PCPP {
//         player_current: PCP {
//             next_move_wins: 1.0,
//             imposible_avoid: 55.5,
//             vert_consecutive_hole_3inline: 0.3,
//             line3: 0.1,
//             line2: 0.01,
//             line1: 0.001,
//         },
//         player_other: PCP {
//             next_move_wins: 100.0,
//             imposible_avoid: 55.5,
//             vert_consecutive_hole_3inline: 0.3,
//             line3: 0.1,
//             line2: 0.01,
//             line1: 0.001,
//         },
//     };

//     // let pcpp_winner = PCPP {
//     //     player_current: PCP {
//     //         next_move_wins: 82.99,
//     //         imposible_avoid: 40.99,
//     //         vert_consecutive_hole_3inline: 66.39,
//     //         line3: 53.97,
//     //         line2: 58.12,
//     //         line1: 22.32,
//     //     },
//     //     player_other: PCP {
//     //         next_move_wins: 63.83,
//     //         imposible_avoid: 31.88,
//     //         vert_consecutive_hole_3inline: 62.24,
//     //         line3: 32.98,
//     //         line2: 63.29,
//     //         line1: 86.85,
//     //     },
//     // };
//     // let pcpp_loser = PCPP {
//     //     player_current: PCP {
//     //         next_move_wins: 47.62,
//     //         imposible_avoid: 25.69,
//     //         vert_consecutive_hole_3inline: 52.06,
//     //         line3: 50.33,
//     //         line2: 68.73,
//     //         line1: 28.51,
//     //     },
//     //     player_other: PCP {
//     //         next_move_wins: 86.61,
//     //         imposible_avoid: 86.9,
//     //         vert_consecutive_hole_3inline: 47.49,
//     //         line3: 76.02,
//     //         line2: 35.01,
//     //         line1: 50.07,
//     //     },
//     // };

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

//     use rand::Rng;
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
//         } else {
//             use std::io;
//             use std::io::prelude::Write;
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
//         match minmax::get_best_move(game1) {
//             Ok((game1, col, _eval)) => {
//                 let game1 = game1.try_play(col);
//                 let game2 = game2.try_play(col);
//                 rec_move(game2, game1)
//             }
//             Err(game1) => (game1, game2),
//         }
//     }

//     let (game1, _game2) = rec_move(game1, game2);

//     match game1.turn {
//         Turn::F(Finished::Won(Player::O)) => PccpWinner::First,
//         Turn::F(Finished::Won(Player::X)) => PccpWinner::Second,
//         Turn::F(Finished::Draw(_)) => PccpWinner::Draw,
//         _ => unreachable!(),
//     }
// }
