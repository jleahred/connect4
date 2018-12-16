use crate::engine::patterns::PatternsCountPlayerPonderation as PCPP;
use crate::engine::patterns::PatternsCountPonderation as PCP;
use crate::engine::{Col, Game, Player, Turn};

fn pp() -> PCPP {
    PCPP {
        player_current: PCP {
            next_move_wins: 0.4,
            imposible_avoid: 1.0,
            vert_consecutive_hole_3inline: 0.3,
            line3: 0.1,
            line2: 0.01,
            line1: 0.001,
        },
        player_other: PCP {
            next_move_wins: 10.3,
            imposible_avoid: 1.0,
            vert_consecutive_hole_3inline: 0.3,
            line3: 0.1,
            line2: 0.01,
            line1: 0.001,
        },
    }
}

#[test]
fn test_4_in_line_vert_and_horizontal() {
    let play_col = |game: Game, col| -> Game {
        match game.play(Col::b(col).unwrap()) {
            Ok(game) => game,
            _ => panic!("error processing move"),
        }
    };

    {
        let game = Game::new(Player::O, pp());

        let game = play_col(game, 0);
        let game = play_col(game, 1);
        let game = play_col(game, 0);
        let game = play_col(game, 1);
        let game = play_col(game, 0);
        let game = play_col(game, 1);
        let game = play_col(game, 0);

        // println!("{}", game);
        assert!(game.turn == Turn::Won(Player::O))
    }

    {
        let game = Game::new(Player::X, pp());

        let game = play_col(game, 2);
        let game = play_col(game, 5);
        let game = play_col(game, 2);
        let game = play_col(game, 5);
        let game = play_col(game, 5);
        let game = play_col(game, 6);
        let game = play_col(game, 5);
        let game = play_col(game, 6);
        let game = play_col(game, 5);
        let game = play_col(game, 6);
        let game = play_col(game, 5);

        // println!("{}", game)
        assert!(game.turn == Turn::Won(Player::X))
    }
    {
        let game = Game::new(Player::O, pp());

        let game = play_col(game, 2);
        let game = play_col(game, 2);
        let game = play_col(game, 3);
        let game = play_col(game, 3);
        let game = play_col(game, 4);
        let game = play_col(game, 4);
        let game = play_col(game, 5);

        // println!("{}", game)
        assert!(game.turn == Turn::Won(Player::O))
    }
    {
        let game = Game::new(Player::O, pp());

        let game = play_col(game, 0);
        let game = play_col(game, 0);
        let game = play_col(game, 1);
        let game = play_col(game, 1);
        let game = play_col(game, 2);
        let game = play_col(game, 2);
        let game = play_col(game, 3);

        // println!("{}", game)
        assert!(game.turn == Turn::Won(Player::O))
    }
    {
        let game = Game::new(Player::O, pp());

        let game = play_col(game, 3);
        let game = play_col(game, 3);
        let game = play_col(game, 4);
        let game = play_col(game, 4);
        let game = play_col(game, 5);
        let game = play_col(game, 5);
        let game = play_col(game, 6);

        // println!("{}", game)
        assert!(game.turn == Turn::Won(Player::O))
    }
}

#[test]
fn test_4_in_line_diagonals() {
    let play_col = |game: Game, col| -> Game {
        match game.play(Col::b(col).unwrap()) {
            Ok(game) => game,
            _ => panic!("error processing move"),
        }
    };

    {
        let game = Game::new(Player::O, pp());

        let game = play_col(game, 0);
        let game = play_col(game, 1);
        let game = play_col(game, 2);
        let game = play_col(game, 3);
        let game = play_col(game, 4);

        let game = play_col(game, 0);
        let game = play_col(game, 1);
        let game = play_col(game, 2);
        let game = play_col(game, 3);
        let game = play_col(game, 4);

        let game = play_col(game, 0);
        let game = play_col(game, 1);
        let game = play_col(game, 2);
        let game = play_col(game, 3);
        let game = play_col(game, 4);

        let game = play_col(game, 0);

        println!("{}", game);
        assert!(game.turn == Turn::Won(Player::X))
    }

    {
        let game = Game::new(Player::O, pp());

        let game = play_col(game, 0);
        let game = play_col(game, 1);
        let game = play_col(game, 2);
        let game = play_col(game, 3);
        let game = play_col(game, 4);

        let game = play_col(game, 0);
        let game = play_col(game, 1);
        let game = play_col(game, 2);
        let game = play_col(game, 3);
        let game = play_col(game, 4);

        let game = play_col(game, 0);
        let game = play_col(game, 1);
        let game = play_col(game, 2);
        let game = play_col(game, 3);
        let game = play_col(game, 4);

        let game = play_col(game, 6);

        let game = play_col(game, 0);
        let game = play_col(game, 1);
        let game = play_col(game, 2);
        let game = play_col(game, 3);
        let game = play_col(game, 6);
        let game = play_col(game, 4);

        println!("{}", game);
        assert!(game.turn == Turn::Won(Player::X))
    }
}
