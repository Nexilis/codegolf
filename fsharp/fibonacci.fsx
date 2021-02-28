// run with: dotnet fsi fibonacci.fsx
let rec f i j k =
    printfn "%i" i
    if k < 30 then f j (i+j) (k+1)
f 0 1 0
