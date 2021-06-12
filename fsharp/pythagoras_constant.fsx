// run with: dotnet fsi pythagoras_constant.fsx
// https://pl.10steps.org/Calculate-a-Square-Root-by-Hand-9247


// step 7
// given x=558 limit=17300 returns 3
let calcNextRoot x limit =
    let mutable x = x * 10
    let mutable currentMax = 1

    let mutable i = 1;

    for i in 1..10 do
        x <- x+i

        if x <= limit
        then currentMax <- i

    currentMax

// step 8
// given x=558 nextRoot=3 returns 5583
let calcNextSubtraction x nextRoot =
    x * 10 + nextRoot
