let sum l = List.fold_left (+) 0 l;;

let rec read ic cur max =
  match input_line ic with
  | "" -> read ic 0 (Int.max max cur)
  | line -> begin
    let v = int_of_string line in
    read ic (v + cur) max
  end
  | exception End_of_file -> Int.max max cur

