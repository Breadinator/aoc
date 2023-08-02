open Common

let crane operation crates =
    Printf.printf "move %d from %d to %d\n" operation.n operation.from_idx operation.to_idx;
    Ok crates
