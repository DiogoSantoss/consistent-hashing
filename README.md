# Consistent Hashing

URL -> x 
h(x) -> x'

Server -> y
h(y) -> y'

Layour:


-----------------------------
| y'|   | x' |   | y2' | x2'|                        
-----------------------------

API:
Lookup(x) -> y
Insert(x)

Insert:
    Receive URL
    Hash URL -> x'
    Store in x' mod size
O(1)


Lookup:
    Receive URL
    Hash URL -> x'
    Find y' that is smaller than x'


Possible data structures:
    Hashmap ?
    Doesn't work because we can't find h(s) <= h(x)
    Hashmap doesn't maintain order

    Heap ?
    Only maintains min/max relationship

    Trees ?
    More specifically a tree such that elements
    have ordering properties
    Binary Search Trees


Binary Search Trees:
    Black-Red trees
    AVL trees

AVL trees are "more balanced" than Black-Red trees.
This provides better lookup and worst insert/delete.
This is good trade for this algorithm..
