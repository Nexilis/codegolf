// run with: clear && dotnet fsi pythagoras_constant.fsx
// https://pl.10steps.org/Calculate-a-Square-Root-by-Hand-9247


let calcMaxSubPair x limit =
    printfn "input -> %i %i" x limit
    // w 1 petli x=0 limit=2 -> (1, 1)
    // w 2 petli x=1 limit=100 -> (4, 4)
    // w 3 petli x=14 limit=400 -> (1, 119)
    // w 4 petli x=141 limit=11900 -> (4, 604)
    // w 5 petli x=1414 limit=60400 -> 2

    let a = x * 2 * 10
    let mutable max = 1
    let mutable sub = 0
    for i in 1..10 do
        if (a+i) * i <= limit
        then
            // todo: break loop if max found
            max <- i
            sub <- (limit - (a+i) *i) *100

    max, sub


let x2,y2 = calcMaxSubPair 1 100
printf "result -> %i %i\n\n" x2 y2

let x3,y3 = calcMaxSubPair 14 400
printf "result -> %i %i\n\n" x3 y3

let x4,y4 = calcMaxSubPair 141 11900
printf "result -> %i %i\n\n" x4 y4

let x5,y5 = calcMaxSubPair 1414 60400
printf "result -> %i %i\n\n" x5 y5

let x1,y1 = calcMaxSubPair 0 2
printf "result -> %i %i\n\n" x1 y1

for i in 1..5 do
    ()
