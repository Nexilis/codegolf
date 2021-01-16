let f x = match x with | x when x = 0 -> "No more bottles" | x when x = 1 -> "1 bottle" | x -> sprintf "%i bottles" x
let b = "of beer on the wall"
printfn "99 bottles %s, 99 bottles of beer." b
[0..98]
|> List.rev
|> List.iter(fun i ->
        let r = f i
        let t = r.ToLower()
        printfn "Take one down and pass it around, %s %s.\n\n%s %s, %s of beer." t b r b t)
printfn "Go to the store and buy some more, 99 bottles %s." b
