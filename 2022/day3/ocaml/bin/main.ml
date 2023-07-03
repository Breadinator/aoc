open Aoc2022day3

let () =
    if Array.length Sys.argv == 1 then begin
        print_endline "Expected input file as command-line argument";
        exit 1
    end;
    let part_unparsed = if Array.length Sys.argv > 2 then Some (Array.get Sys.argv 2) else None in
    let part = match part_unparsed with
    | Some "1" -> 1
    | Some "2" -> 2
    | _ -> 0 in

    if part != 1 then begin
        print_endline "Part 2 not supported, please put \"1\" in the argument after the path.";
        exit 1
    end;

    let path = Array.get Sys.argv 1 in
    let ic = open_in path in
        try
            if part == 0 || part == 1 then begin
                let answer = Part1.solve(ic) in
                print_int answer;
                print_newline ()
            end;

            close_in ic;
            exit 0
        with e ->
            close_in_noerr ic;
            raise e

