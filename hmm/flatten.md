```yml
parent:
  vals:
    ...
  ptrs:
    - ch1
    - ch2
ch1:
  vals:
    ...
  ptrs:
    null
ch2:
  vals:
    ...
  ptrs:
    - parent
```

# How to traverse through all paths of a tree while also preventing cyclic references
# Ideas:
# Recursion
# While loops

```py (pseudo code)
parents = ["parent"]
messages_list = parents.clone
for parent in parents:
  pass

look_through_all_branches(parent_nodes) (all_nodes):
  all_nodes = []
  for node in parent_nodes:
      yes(all_nodes, parent_node)

yes(&all_nodes, parent_node):
  if all_nodes.has(parent_node):
    # warning
    return
  all_nodes.push(parent_node.data)
  if parent_node has children:
    for child in parent_node.children:
      yes(all_nodes, child)
```

# I wanted to try implement it myself
