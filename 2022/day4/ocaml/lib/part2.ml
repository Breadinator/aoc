let handle_line line =
    if Common.parse line
    |> Option.get
    |> Common.has_any_overlap then
        1
    else
        0

let rec solve acc ic =
    match input_line ic with
    | line -> solve (acc + (handle_line line)) ic
    | exception End_of_file -> acc
