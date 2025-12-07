let string_to_char_list string = string |> String.to_seq |> List.of_seq
let get_initial_beams string = List.map (fun c -> c = 'S') string
let get_splits string char = List.map (fun c -> c = char) string

let count_true list =
  List.fold_left (fun acc x -> if x then acc + 1 else acc) 0 list

let count_splits splits beams = count_true (List.map2 ( && ) splits beams)

let split_beams splits beams =
  let length = List.length splits in
  List.mapi
    (fun index _ ->
      if List.nth splits index then false
      else if
        index < length - 1
        && List.nth splits (index + 1)
        && List.nth beams (index + 1)
      then true
      else if
        index > 0 && List.nth splits (index - 1) && List.nth beams (index - 1)
      then true
      else List.nth beams index)
    splits

let process ic =
  let beams = get_initial_beams (string_to_char_list (input_line ic)) in
  let rec process' ic beams total =
    match input_line ic with
    | exception End_of_file -> total
    | line ->
        let line = string_to_char_list line in
        let splits = get_splits line '^' in
        let split_count = count_splits splits beams in
        let new_beams = split_beams splits beams in
        process' ic new_beams (total + split_count)
  in
  process' ic beams 0

let () =
  let ic = In_channel.with_open_text "input.txt" in
  let total = ic process in
  Printf.printf "Total: %d\n" total
