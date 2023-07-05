open Aoc2022day4

let () =
    let part, path = match Common.get_opt Sys.argv 1, Common.get_opt Sys.argv 2 with
    | Some "1", Some path -> 1, path
    | Some _, Some path -> 2, path
    | Some "1", None -> 1, "../input.txt"
    | _ -> 2, "../input.txt" in

    let solver = if part == 1 then Part1.solve else Part2.solve in
    let ic = open_in path in
    try
        let answer = solver 0 ic in
        print_int answer;
        print_newline ();
        close_in ic
    with e ->
        close_in_noerr ic;
        raise e

