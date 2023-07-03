let handle_triplet a b c =
    let set_a = Common.str_to_set_of_chars a in
    let set_b = Common.str_to_set_of_chars b in
    let set_c = Common.str_to_set_of_chars c in

    Common.CS.inter set_a set_b
    |> Common.CS.inter set_c
    |> Common.CS.choose
    |> Common.get_priority

let solve ic =
    let rec s acc =
        match input_line ic, input_line ic, input_line ic with
        | a, b, c -> s (acc + (handle_triplet a b c))
        | exception End_of_file -> acc in
    s 0

