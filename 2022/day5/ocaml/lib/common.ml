let (>>=) = Result.bind
let (>>=|>) x y = x >>= fun z -> z |> y

(* So I don't have to `open Core` and break everything ^^ *)
let compare_char = Core.compare_char
let char_of_sexp = Core.char_of_sexp
let sexp_of_char = Core.sexp_of_char

type crane_operation = {
    n: int;
    from_idx: int;
    to_idx: int;
}

type parse_crates_error =
    | InvalidCrateConfiguration
    | InvalidValue of char
[@@deriving compare, sexp]

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

(** Parses the initial configuration of crates.

Expects an input such as: `\["[A] [B]"; "[C] [D]"; " 1  2 "]`*)
let parse_crates lines =
    let rec parse_line stacks line n =
        if String.length line == 0 then Ok stacks
        else begin
            (* if stmt bc it throws if oob *)
            let tl = if String.length line > 4 then Str.string_after line 4 else "" in
            match String.get line 1 with
            | ('A'..'Z' as ch) ->
                stacks.(n) <- ch :: stacks.(n);
                parse_line stacks tl (n + 1)
            | ' ' -> parse_line stacks tl (n + 1)
            | ch -> Error (InvalidValue ch)
        end
    in

    let rec handle_line rem_lines stacks =
        match rem_lines with
        | hd :: tl ->
            if ((String.length hd + 1) mod 4) == 0 then
                parse_line stacks hd 0 >>=|> handle_line tl
            else Error InvalidCrateConfiguration
        | [] -> Ok stacks
    in

    match List.rev lines with
    | labels :: crates ->
            let num_stacks = ((String.length labels + 1) / 4) in
            handle_line crates (Array.make num_stacks [])
    | [] -> Error InvalidCrateConfiguration

let solve _crane ic =
    (** This just takes the first lines of the input, up until the first empty line. *)
    let rec input_starting_positions ic acc =
        let temp = input_line ic in
        if String.length temp == 0 then acc
        else input_starting_positions ic (acc @ [temp])
    in

    let stacks = input_starting_positions ic []
    |> parse_crates in

    let _ = match stacks with
    | Ok stacks -> print_stacks stacks
    | Error err -> sexp_of_parse_crates_error err |> Core.Sexp.to_string |> print_endline
    in

    ""

