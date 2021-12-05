use super::*;

pub fn find_last_winner_board() -> Board<FinalState> {
    let mut input = Input::new(FILE_NAME);
    let mut bingo_input = BingoInput::parse(&mut input);

    let last_winner_board = bingo_input
        .boards
        .iter_mut()
        .map(|board| board.play(&bingo_input.numbers))
        .min_by(|board_1, board_2| {
            if board_1.is_bingo() && !board_2.is_bingo() {
                return Ordering::Greater;
            }
            if board_2.is_bingo() && !board_1.is_bingo() {
                return Ordering::Less;
            }
            if board_1.get_bingo_round() < board_2.get_bingo_round() {
                return Ordering::Greater;
            }
            if board_2.get_bingo_round() < board_1.get_bingo_round() {
                return Ordering::Less;
            }
            if board_1.get_points() < board_2.get_points() {
                return Ordering::Greater;
            }
            if board_2.get_points() < board_1.get_points() {
                return Ordering::Less;
            }
            Ordering::Equal
        });

    last_winner_board.unwrap()
}
