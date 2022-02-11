# Problem

For a bipartite graph of {P, R}, partition P into sets such that every element in
partition is adjacent to exactly the same nodes in R.

# Solutions

- a Datalog one using Crepe
- A hand-coded one with HashMaps

Any other ideas are welcome!

# Current Results

Comparing the Datalog impl to a for loops and hashmaps impl: The Datalog impl using Crepe is incredibly slower, but I think that is due to use of `_` in expressions, in Crepe I believe this unneccessarily matches against every item of the set, so refactoring the logic to not need it is a next step.

```
second-order-graph-trial on  main is 📦 v0.1.0 via 🦀 v1.58.1 on ☁️  (us-east-1)
❯ date ; time cargo criterion
Fri Feb 11 13:41:31 PST 2022
   Compiling utilities v0.1.0 (/Users/rch/repo/second-order-graph-trial/utilities)
   Compiling second-order-graph-trial v0.1.0 (/Users/rch/repo/second-order-graph-trial)
    Finished bench [optimized] target(s) in 3.49s
Gnuplot not found, using plotters backend
datalog: 8 edges        time:   [186.40 ns 187.57 ns 188.79 ns]
                        change: [-2.4398% -1.5748% -0.6312%] (p = 0.00 < 0.05)
                        Change within noise threshold.

Benchmarking datalog: 2408 edges: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 105.8s, or reduce sample count to 10.
datalog: 2408 edges     time:   [1.0076 s 1.0126 s 1.0176 s]
                        change: [-2.1476% -1.4667% -0.6874%] (p = 0.00 < 0.05)
                        Change within noise threshold.

Benchmarking datalog: 4808 edges: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 548.4s, or reduce sample count to 10.
datalog: 4808 edges     time:   [5.1255 s 5.1672 s 5.2159 s]

Gnuplot not found, using plotters backend
for loops and hashmaps: 8 edges
                        time:   [98.874 ns 99.472 ns 100.17 ns]

for loops and hashmaps: 2408 edges
                        time:   [1.9979 ms 2.0197 ms 2.0495 ms]

for loops and hashmaps: 4808 edges
                        time:   [7.1976 ms 7.2238 ms 7.2512 ms]


real	11m10.299s
user	11m57.804s
sys	0m17.483s
```
