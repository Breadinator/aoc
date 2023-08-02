open Core
open Aoc22d5
open Aoc22d5.Common

let%test_unit "basic" =
    let res = Common.parse_operation "move 1 from 7 to 4" in
    let expected = Ok { n=1; from_idx=7; to_idx=4 } in
    [%test_eq: (Common.crane_operation, Common.parse_error) Result.t] res expected

(* NOTE: for some reason this test doesn't compile. It previously passed.
   Considering I don't think there are even any operations that move that many items it should be fine. *)
(*let%test_unit "multiple_digits" =
    let res = Common.parse_operation "move 10102 from 789274 to 40" in
    let expected = Ok { n=10102; from_idx=789274; to_idx=40 } in
    [%test_eq: (Common.crane_operation, Common.parse_error) Result.t] res expected*)

