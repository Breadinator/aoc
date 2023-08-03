open Common

let crane operation crates =
    if operation.from_idx > 0 && operation.to_idx <= Array.length crates then
        Ok crates
    else
        Error (IndexOutOfBounds operation)

