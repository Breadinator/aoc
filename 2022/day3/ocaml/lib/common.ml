module CS = Set.Make(Char)

let get_priority ch =
    let ch_val = Char.code ch in
    ch_val - (if ch_val < 97 then 38 else 96)

let str_to_set_of_chars s =
    String.to_seq s
    |> CS.of_seq

