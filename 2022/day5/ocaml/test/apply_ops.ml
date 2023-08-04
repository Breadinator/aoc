open Core
open Aoc22d5

let%test_unit "take_n" =
    let input = [1; 2; 3; 4; 5] in
    let modified_input, taken = Common.take_n input 3 in
    [%test_eq: int list] modified_input [4; 5];
    [%test_eq: int list] taken [1; 2; 3]

let%test_unit "push_to" =
    let modified_input, taken = [4; 5], [1; 2; 3] in
    let res = Common.push_to taken modified_input in
    [%test_eq: int list] res [5; 4; 1; 2; 3]

