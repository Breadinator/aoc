open Aoc2022day1

let () =
  if Array.length Sys.argv == 1 then begin
    print_endline "Expected input file as command-line argument";
    flush stdout;
    exit 1
  end;

  let path = Array.get Sys.argv 1 in
  let ic = open_in path in
    try
      print_int (Reader.read ic [] 0);
      print_newline ();
      flush stdout;
      close_in ic;
      exit 0
    with e ->
      close_in_noerr ic;
      raise e

