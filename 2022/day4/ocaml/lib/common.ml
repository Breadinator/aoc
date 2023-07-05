type pairing = {
    a_min: int;
    a_max: int;
    b_min: int;
    b_max: int
}

(*
 * I doubt this is idiomatic lmao 🤔
 * Even if it is idiomatic, it is grotesque
 * Could've used List.nth / List.nth_opt ig??????
 *)
let parse line =
    let read_list_into_tuple acc item =
        match acc with
        | Some a, Some b, Some c, None -> Some a, Some b, Some c, int_of_string_opt item
        | Some a, Some b, None, None -> Some a, Some b, int_of_string_opt item, None
        | Some a, None, None, None -> Some a, int_of_string_opt item, None, None
        | _ -> int_of_string_opt item, None, None, None in
    let convert_tuple_into_pairing tup =
        match tup with
        | Some a, Some b, Some c, Some d -> Some { a_min = a; a_max = b; b_min = c; b_max = d }
        | _ -> None in

    String.split_on_char ',' line
    |> (List.map (String.split_on_char '-'))
    |> List.flatten
    |> (List.fold_left read_list_into_tuple (None, None, None, None))
    |> convert_tuple_into_pairing

let has_subset p =
    (p.a_min >= p.b_min && p.a_max <= p.b_max)
        || (p.a_min <= p.b_min && p.a_max >= p.b_max)

let has_any_overlap p =
    (p.a_min <= p.b_min && p.a_max >= p.b_min)
        || (p.b_min <= p.a_min && p.b_max >= p.a_min)
        || (p.a_max >= p.b_max && p.a_min <= p.b_max)
        || (p.b_max >= p.a_max && p.b_min <= p.a_max)

let get_opt arr idx =
    if idx < Array.length arr then Some (Array.get arr idx)
    else None

