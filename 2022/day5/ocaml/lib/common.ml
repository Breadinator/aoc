let (>>=) = Result.bind
let (>>=|>) x y = x >>= fun z -> z |> y

(* So I don't have to `open Core` and break everything ^^ *)
let compare_char = Core.compare_char
let char_of_sexp = Core.char_of_sexp
let sexp_of_char = Core.sexp_of_char
let compare_int = Core.compare_int
let int_of_sexp = Core.int_of_sexp
let sexp_of_int = Core.sexp_of_int
let compare_string = Core.compare_string
let string_of_sexp = Core.string_of_sexp
let sexp_of_string = Core.sexp_of_string

type crane_operation = {
    n: int;
    from_idx: int;
    to_idx: int;
}
[@@deriving compare, sexp]

type parse_error =
    | InvalidCrateConfiguration
    | InvalidValue of char
    | DidntMatchTemplate of string
    | CouldntParseInt of string
[@@deriving compare, sexp]

type error =
    | ParseError of parse_error
[@@deriving compare, sexp]

let result_wrap res =
    Result.map_error (fun e -> ParseError e) res

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

let fold_stacks (stacks : char list array) : string =
    let fold_stack (stack : char list) : char =
        List.hd stack
    in
    Array.map fold_stack stacks
    |> Array.to_seq
    |> String.of_seq

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

let parse_operation line =
    match String.split_on_char ' ' line with
    | ["move"; n; "from"; from_idx; "to"; to_idx ] -> begin
        match int_of_string_opt n, int_of_string_opt from_idx, int_of_string_opt to_idx with
        | Some n, Some from_idx, Some to_idx -> Ok { n; from_idx; to_idx }
        | None, _, _ -> Error (CouldntParseInt n)
        | _, None, _ -> Error (CouldntParseInt from_idx)
        | _, _, None -> Error (CouldntParseInt to_idx)
    end
    | _ -> Error (DidntMatchTemplate line)

let solve crane ic =
    (** This just takes the first lines of the input, up until the first empty line. *)
    let rec input_starting_positions ic acc =
        let temp = input_line ic in
        if String.length temp == 0 then acc
        else input_starting_positions ic (acc @ [temp])
    in

    (** Iterates through the lines and applies the operations. *)
    let rec apply_operations stacks =
        match input_line ic with
        | line -> begin
            parse_operation line
            |> result_wrap
            >>= (fun x -> crane x stacks)
            >>=|> apply_operations
        end
        | exception End_of_file -> Ok stacks
    in

    input_starting_positions ic []
    |> parse_crates
    |> (Result.map_error (fun e -> ParseError e))
    >>=|> apply_operations
    |> Result.map fold_stacks

