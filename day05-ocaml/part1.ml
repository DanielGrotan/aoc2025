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

let process_queries ranges ic =
  let rec loop accum =
    match input_line ic with
    | exception End_of_file -> accum
    | line ->
        let id = int_of_string line in
        let exists =
          List.exists (fun (start, stop) -> start <= id && id <= stop) ranges
        in
        loop (accum + if exists then 1 else 0)
  in
  loop 0

let solve ic =
  let ranges = get_ranges ic in
  let count = process_queries ranges ic in
  Printf.printf "Count is: %d\n" count

let () = In_channel.with_open_text "input.txt" solve
