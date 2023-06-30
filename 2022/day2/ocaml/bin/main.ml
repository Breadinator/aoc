open Aoc2022day2

let () =
    let path = Misc.get_path in
    let answer = Misc.run_solution path Solution.solution in
    match answer with
    | Some a -> print_int a; print_newline ()
    | None -> print_endline "An error occurred"

