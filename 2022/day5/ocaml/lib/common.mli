type crane_operation = {
    n: int;
    from_idx: int;
    to_idx: int;
}
val sexp_of_crane_operation : crane_operation -> Core.Sexp.t
val crane_operation_of_sexp : Core.Sexp.t ->crane_operation
val compare_crane_operation : crane_operation -> crane_operation -> int

type parse_error =
    | InvalidCrateConfiguration
    | InvalidValue of char
    | DidntMatchTemplate of string
    | CouldntParseInt of string
val sexp_of_parse_error : parse_error -> Core.Sexp.t
val parse_error_of_sexp : Core.Sexp.t -> parse_error
val compare_parse_error : parse_error -> parse_error -> int

type error =
    | ParseError of parse_error
    | IndexOutOfBounds of crane_operation
val sexp_of_error : error -> Core.Sexp.t
val error_of_sexp : Core.Sexp.t -> error
val compare_error : error -> error -> int

(* main functionality *)
type crane_func = crane_operation -> char list array -> (char list array, error) result
val solve : crane_func -> in_channel -> (string, error) result

(* parsing *)
val parse_crates : string list -> (char list array, parse_error) result
val parse_operation : string -> (crane_operation, parse_error) result

(* printing etc *)
val print_stacks : char list array -> unit
val fold_stacks : char list array -> string

(* utilities *)
val get_opt : 'a array -> int -> 'a option

