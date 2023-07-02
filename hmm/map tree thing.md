# This is for the markup languages like json

Instead of using a memory address like 0x1a2b3c4d5e6f7890 and 0x7ffebf19c548 
I will be using a hashmap "address" like "hello" and "pineapple"

```rs (pseudo code)
struct HashMapTreeNode<K, T>:
    
    map: &HashMap<K, HashMapTreeNode<K, T>>
    data: T
    children: Vec<K> 
    

impl HashMapTreeNode:

    fn new() {...}

    fn get_children() {
        // convert hashmap "addresses" to references
        found_children = new vec
        for child in self.children:
            found_children.push(map.get(child)?)

        // do some functional bs on the stuff above
    }

```
