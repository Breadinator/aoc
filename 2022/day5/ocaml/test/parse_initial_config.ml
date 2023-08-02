open Core
open Aoc22d5

let%test_unit "basic" =
    let res = Common.parse_crates [
        "[A] [B]";
        "[C] [D]";
        " 1   2 "
    ] in
    let expected = Ok (Array.of_list [['A'; 'C']; ['B'; 'D']]) in
    [%test_eq: (char list array, Common.parse_error) Result.t] res expected

let%test_unit "real" =
    let res = Common.parse_crates [
        "                [M]     [W] [M]    ";
        "            [L] [Q] [S] [C] [R]    ";
        "            [Q] [F] [F] [T] [N] [S]";
        "    [N]     [V] [V] [H] [L] [J] [D]";
        "    [D] [D] [W] [P] [G] [R] [D] [F]";
        "[T] [T] [M] [G] [G] [Q] [N] [W] [L]";
        "[Z] [H] [F] [J] [D] [Z] [S] [H] [Q]";
        "[B] [V] [B] [T] [W] [V] [Z] [Z] [M]";
        " 1   2   3   4   5   6   7   8   9 "
    ] in
    let expected = Ok (Array.of_list [
        ['T'; 'Z'; 'B'];
        ['N'; 'D'; 'T'; 'H'; 'V'];
        ['D'; 'M'; 'F'; 'B'];
        ['L'; 'Q'; 'V'; 'W'; 'G'; 'J'; 'T'];
        ['M'; 'Q'; 'F'; 'V'; 'P'; 'G'; 'D'; 'W'];
        ['S'; 'F'; 'H'; 'G'; 'Q'; 'Z'; 'V'];
        ['W'; 'C'; 'T'; 'L'; 'R'; 'N'; 'S'; 'Z'];
        ['M'; 'R'; 'N'; 'J'; 'D'; 'W'; 'H'; 'Z'];
        ['S'; 'D'; 'F'; 'L'; 'Q'; 'M']
    ]) in
    [%test_eq: (char list array, Common.parse_error) Result.t] res expected

