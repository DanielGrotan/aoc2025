let solve _ = 
  "Part 2"

let () =
  let input = In_channel.(with_open_bin "input.txt" input_all) in
  Printf.printf "%s\n" (solve input)
