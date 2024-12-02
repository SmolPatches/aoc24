# Day1
## Part 1
- Compare list items iteratively in ascending order
Use a [min heap](https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html#min-heap)
Get the list from 2 files
make each list into a binary heap
- Get the distance for [0..n) items of both lists
- Get total distance for 2 lists
Add the individiual distances

so i need 
2 filenames
2 heaps of length n
1 collection for the distances ( also of length n )

## Part 2
Q: How many times does each # in left list appear in right
HashMap with keys the unique ids in the left
Value is the frequency on the right
