// run with: clear && dotnet fsi pythagoras_constant.fsx
// https://pl.10steps.org/Calculate-a-Square-Root-by-Hand-9247

// todo: on arr or string (so it can be used in the first step)
let calcRes (x: uint64) (y: uint64) : uint64 = x * uint64 (10) + y

// test cases for square root
// 1) x=0    limit=2     -> (1, 1)
// 2) x=1    limit=100   -> (4, 4)
// 3) x=14   limit=400   -> (1, 119)
// 4) x=141  limit=11900 -> (4, 604)
// 5) x=1414 limit=60400 -> (2, 383600)
let calcStep (x: uint64) (limit: uint64) =
    printfn "-> %i %i" x limit

    let a = x * uint64 2 * uint64 10
    let mutable max = uint64 1
    let mutable sub = uint64 0

    for i in 0 .. 10 do
        let i = uint64 i

        if (a + i) * i <= limit then
            // todo: break loop if max found
            max <- i
            sub <- limit - (a + i) * i

    if max >= uint64 10 then
        // todo: this computation method to work, it would have to use much bigger datatype than uint64
        raise (System.OverflowException())

    max, (sub * uint64 100)

// ENTRY POINT // INPUT VARIABLE
let squareRootOf = uint64 2

let mutable r, y = calcStep (uint64 0) squareRootOf

for _ in 1 .. 20 do
    let x, yn = calcStep r y
    y <- yn
    r <- calcRes r x
    ()
