open Core
open Aoc22d5
open Aoc22d5.Common

let%test_unit "basic" =
    let res = Common.parse_operation "move 1 from 7 to 4" in
    let expected = Ok { n=1; from_idx=7; to_idx=4 } in
    [%test_eq: (Common.crane_operation, Common.parse_operation_error) Result.t] res expected

(** Not actually used but nice that it works *)
let%test_unit "mutliple_digits" =
    let res = Common.parse_operation "move 10102 from 789274 to 40" in
    let expected = Ok { n=10102; from_idx=789274; to_idx=40 } in
    [%test_eq: (Common.crane_operation, Common.parse_operation_error) Result.t] res expected

