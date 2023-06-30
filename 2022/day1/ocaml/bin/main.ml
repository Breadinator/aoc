open Aoc2022day1

let () =
  if Array.length Sys.argv == 1 then begin
    print_endline "Expected input file as command-line argument";
    flush stdout;
    exit 1;
  end;

  let path = Array.get Sys.argv 1 in
  let ic = open_in path in
    try
      let top3 = Reader.read ic 0 { a = 0; b = 0; c = 0 } in
      print_endline "Top 3:";
      print_int top3.a;
      print_newline ();
      print_int top3.b;
      print_newline ();
      print_int top3.c;
      print_newline ();
      print_endline "\nSum of top 3:";
      print_int (top3.a + top3.b + top3.c);
      print_newline ();
      exit 0
    with e ->
      close_in_noerr ic;
      raise e

