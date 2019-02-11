# Part 1

* Store the claims in a new `struct` type
* `Claim` will contain:
** `id`
** `x1` - the starting coordinate from the left edge
** `y1` - the starting coordinate from the top edge
** `x2` - the ending coordinate from the left edge
** `y2` - the ending coordinate from the top edge
* `Claim` will have `intersection()` method that takes another claim as an argument and returns an optional `Claim` that contains coordinates of the intersection of the two original claims
* Another method can find the total square inches of all intersections while removing duplicate overlaps
* This step could possibly find a union of the overlaps to remove the duplicates?
