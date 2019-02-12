# Part 1

* Store the claims in a new `struct` type
* `Claim` will contain:
 - `id`
 - `p1` - x,y pair of the top-left point
 - `p2` - x,y pair of the bottom-right point
* `Claim` will have `intersection()` method that takes another claim as an argument and returns an optional `Claim` that contains coordinates of the intersection of the two original claims
* Another method on `Claim` will return a set of all the points in the `Claim`
