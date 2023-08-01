let (>>=) = Result.bind
let (>>=|>) x y = x >>= fun z -> z |> y

type crane_operation = {
    n: int;
    from_idx: int;
    to_idx: int;
}

type parse_crates_error =
    | InvalidCrateConfiguration
    | InvalidValue of char

let get_opt arr idx =
    if idx < Array.length arr then Some (Array.get arr idx)
    else None

let print_stacks stacks =
    let rec print_stack stack =
        match stack with
        | hd :: tl ->
            print_char hd;
            print_stack tl
        | [] -> print_newline ()
    in
    Array.iter print_stack stacks

let parse_crates lines =
    let rec parse_line stacks line n num_stacks =
        if String.length line == 0 then
            Ok stacks
        else if (String.length line + 1) == (num_stacks - n) * 4 then
            (* if stmt bc it throws if oob *)
            let tl = if String.length line > 4 then Str.string_after line 4 else "" in
            match String.get line 1 with
            | ('A'..'Z' as ch) ->
                stacks.(n) <- ch :: stacks.(n);
                parse_line stacks tl (n + 1) num_stacks
            | ' ' -> parse_line stacks tl (n + 1) num_stacks
            | ch -> Error (InvalidValue ch)
        else
            Error InvalidCrateConfiguration
    in

    let rec handle_line rem_lines num_stacks stacks =
        match rem_lines with
        | hd :: tl -> parse_line stacks hd 0 num_stacks >>=|> handle_line tl num_stacks
        | [] -> Ok stacks
    in

    match List.rev lines with
    | labels :: crates ->
            let num_stacks = (String.length labels + 1) / 4 in
            handle_line crates num_stacks (Array.make num_stacks [])
    | [] -> Error InvalidCrateConfiguration

let solve _crane ic =
    (* this just takes the first tines of the input *)
    let rec input_starting_positions ic acc =
        let temp = input_line ic in
        if String.length temp == 0 then acc
        else input_starting_positions ic (acc @ [temp])
    in

    let stacks = input_starting_positions ic []
    |> parse_crates in

    (* debug print *)
    (* probably a better way to do this *)
    let _ = match stacks with
    | Ok stacks -> print_stacks stacks
    | Error InvalidCrateConfiguration -> print_endline "InvalidCrateConfiguration"
    | Error (InvalidValue ch) ->
        print_string "InvalidValue: ";
        print_char ch;
        print_newline ()
    in

    ""

