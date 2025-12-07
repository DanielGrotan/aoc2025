let string_to_char_list string = string |> String.to_seq |> Array.of_seq
let read_line_chars ic = input_line ic |> string_to_char_list

let get_initial_beams chars =
  Array.map (fun c -> if c = 'S' then 1 else 0) chars

let get_splits chars = Array.map (( = ) '^') chars

let apply_splits splits beams =
  let n = Array.length splits in
  Array.init n (fun i ->
      if splits.(i) then 0
      else
        let left = if i > 0 then beams.(i - 1) else 0 in
        let right = if i < n - 1 then beams.(i + 1) else 0 in
        let left_split = i > 0 && splits.(i - 1) && left > 0 in
        let right_split = i < n - 1 && splits.(i + 1) && right > 0 in
        let extra =
          match (left_split, right_split) with
          | true, true -> left + right
          | true, false -> left
          | false, true -> right
          | false, false -> 0
        in
        beams.(i) + extra)

let process ic =
  let beams = get_initial_beams (read_line_chars ic) in
  let rec loop beams =
    match read_line_chars ic with
    | exception End_of_file -> beams
    | chars ->
        let splits = get_splits chars in
        apply_splits splits beams |> loop
  in
  loop beams

let () =
  let beams = In_channel.with_open_text "input.txt" process in
  let sum = Array.fold_left ( + ) 0 beams in
  Printf.printf "Total: %d\n" sum
