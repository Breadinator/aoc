open Aoc22d5
open Common

let () =
    let part, path = match get_opt Sys.argv 1, get_opt Sys.argv 2 with
    | Some "1", Some path -> 1, path
    | Some _, Some path -> 2, path
    | Some "1", None -> 1, "../input.txt"
    | _ -> 2, "../input.txt" in

    let crane = if part == 1 then Part1.crane else Part2.crane in
    let ic = open_in path in
    try
        (match Common.solve crane ic with
        | Ok s -> s
        | Error e -> sexp_of_error e |> Core.Sexp.to_string_hum)
        |> print_endline;
        close_in ic
    with e ->
        close_in_noerr ic;
        raise e

