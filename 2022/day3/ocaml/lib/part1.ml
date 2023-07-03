let calculate_line line =
    let comp_size = (String.length line) / 2 in
    let first = String.sub line 0 comp_size
    |> Common.str_to_set_of_chars in
    let second = String.sub line comp_size comp_size
    |> Common.str_to_set_of_chars in

    Common.CS.inter first second
    |> Common.CS.choose
    |> Common.get_priority

let solve ic =
    let rec s acc =
        match input_line ic with
        | line -> s (acc + (calculate_line line))
        | exception End_of_file -> acc in
    s 0

