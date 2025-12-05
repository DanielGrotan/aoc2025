let get_ranges ic =
  let rec loop list =
    match input_line ic with
    | "" -> list
    | line ->
        let range = String.split_on_char '-' line in
        let range_start = int_of_string (List.nth range 0) in
        let range_end = int_of_string (List.nth range 1) in
        loop ((range_start, range_end) :: list)
  in
  loop []

let count_fresh ranges =
  let total, _ =
    List.fold_left
      (fun (acc, max_seen) (start, stop) ->
        let range_length =
          if start > max_seen then stop - start + 1
          else if stop <= max_seen then 0
          else stop - max_seen
        in
        (acc + range_length, max max_seen stop))
      (0, -1) ranges
  in
  total

let solve ic =
  let ranges = get_ranges ic in
  let sorted = List.sort compare ranges in
  let fresh = count_fresh sorted in
  Printf.printf "Count is: %d\n" fresh

let () = In_channel.with_open_text "input.txt" solve
