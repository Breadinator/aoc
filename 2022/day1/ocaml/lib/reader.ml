(* Could be generic? *)
type top3 = {
  a : int;
  b : int;
  c : int
}

let push_top3 item cur =
  if cur.a < item then      { a = item;  b = cur.a; c = cur.b }
  else if cur.b < item then { a = cur.a; b = item;  c = cur.b }
  else if cur.c < item then { a = cur.a; b = cur.b; c = item }
  else cur

let rec read ic cur cur_top3 =
  match input_line ic with
  | "" -> read ic 0 (push_top3 cur cur_top3)
  | line -> begin
    let v = int_of_string line in
    read ic (v + cur) cur_top3
  end
  | exception End_of_file -> push_top3 cur cur_top3 

