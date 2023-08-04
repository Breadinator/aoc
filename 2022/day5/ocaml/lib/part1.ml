open Common

let crane operation crates =
    if operation.from_idx > 0 && operation.to_idx <= Array.length crates then
        take_n crates.(operation.from_idx - 1) operation.n
        |> fun (modified, taken) ->
            crates.(operation.from_idx - 1) <- modified;
            crates.(operation.to_idx - 1) <- push_to crates.(operation.to_idx - 1) taken;
            Ok crates
    else
        Error (IndexOutOfBounds operation)

