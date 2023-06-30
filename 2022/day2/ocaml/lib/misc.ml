let get_path =
    if Array.length Sys.argv == 1 then begin
        print_endline "Expected input file as command-line argument";
        exit 1
    end;
    Array.get Sys.argv 1

let run_solution path f =
    let ic = open_in path in
    try
        let value = f ic in
        close_in ic;
        Some value
    with
    | _ -> close_in_noerr ic; None

let fold_lines ic value f =
    let rec read_lines acc =
        match input_line ic with
        | line -> read_lines (f acc line)
        | exception End_of_file -> acc
    in
    read_lines value

