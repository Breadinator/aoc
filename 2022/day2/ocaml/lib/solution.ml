type move = Rock | Paper | Scissors
type outcome = Win | Draw | Loss

let parse_move line =
    match line.[0] with
    | 'A' -> Rock
    | 'B' -> Paper
    | 'C' -> Scissors
    | _ -> print_endline "invalid character in input"; exit 1

let parse_outcome line =
    match line.[2] with
    | 'X' -> Loss
    | 'Y' -> Draw
    | 'Z' -> Win
    | _ -> print_endline "invalid character in input"; exit 1

let move_value player_move =
    match player_move with
    | Rock -> 1
    | Paper -> 2
    | Scissors -> 3

let outcome_value outcome =
    match outcome with
    | Win -> 6
    | Draw -> 3
    | Loss -> 0

let what_move_do enemy_move desired_outcome =
    if desired_outcome == Draw then enemy_move
    else
        match enemy_move with
        | Rock -> if desired_outcome == Win then Paper else Scissors
        | Paper -> if desired_outcome == Win then Scissors else Rock
        | Scissors -> if desired_outcome == Win then Rock else Paper

let handle_line acc line =
    let enemy_move = parse_move line in
    let desired_outcome = parse_outcome line in
    let my_move = what_move_do enemy_move desired_outcome in
    acc + (move_value my_move) + (outcome_value desired_outcome)

let solution ic =
    Misc.fold_lines ic 0 handle_line

