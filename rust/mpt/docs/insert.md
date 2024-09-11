# Insert

Depending on the type of the node, insert method works differently.

The (key, value) pair is often represented in the Node::insert function using the Entry structure to facilitate efficient insertion into a trie. The Entry struct provides a convenient way to encapsulate both the key and value, allowing the Node::insert function to handle them as a single unit. The From<(K, V)> for Entry implementation allows for easy creation of an Entry from various types, enabling the Node::insert function to accept a wide range of input formats. Entry also enables creating Node directly from it, transforming self into Branch with value and empty children (in the case of empty keys) or Leaf (if key is not empty).

Insert function works differently depending on the type of node we are inserting to:

## Into Null

This happens only during the first insert into the trie. `Node::Null` is not created later during insertion in any way.

When we insert into `Node::Null`, we replace the null node with a `Branch` with a value if the inserted value's key is empty or a `Leaf` otherwise.

## Into Digest

Insert into Digest node shouldn't even happen so we just panic if we trie to insert into Digest.

## Into Leaf

In order to simplify the number of cases we handle, we convert leaf key and value into the Entry and replace it with the new node created using `from_two_entries` function. This way we treat old entry and the new one symmetricaly and have less cases to consider.

### from_two_entries(lhs: impl Into<Entry>,rhs: impl Into<Entry>)

It does the following:

If the keys are equal we throw `DuplicateKey` error.

```rs
if shorter.key == longer.key {
    return Err(NodeError::DuplicateKey);
}
```

Then it sorts `lhs` and `rhs` entries

```rs
let (shorter, longer) = order_entries(lhs, rhs);
```

Now we handle multiple cases of `shorter.key` and `longer.key` configurations.

#### 1. `shorter.key` is empty

    We know that `longer.key` can't be, since the case of equal keys was already handled above. Therefore, we can split longer, extracting its first key nibble. We then create a `Branch` that has `shorter.value` as a value and `remaining_longer` as a child at `longer_first_nibble index`.

   Note that the branch child can be a `Leaf` or a `Branch` (with value) depending on the `remaining_longer.key` length.

![Schema](./images/into_leaf_0.png)

```rs
if shorter.key.is_empty() {
    let (longer_first_nibble, remaining_longer) = longer.split_first_key_nibble();
    return Ok(Node::branch_with_child_and_value(
        longer_first_nibble,
        remaining_longer,
        shorter.value,
    ));
}
```

#### 2. Both keys are not empty, `shorter_first_nibble != longer_first_nibble`
Here we just create branch with two children, as shown on the picture below.

![Schema](./images/into_leaf_1.png)

```rs
let (shorter_first_nibble, remaining_shorter) = shorter.split_first_key_nibble();
let (longer_first_nibble, remaining_longer) = longer.split_first_key_nibble();

if shorter_first_nibble != longer_first_nibble {
    return Ok(Node::branch_with_two_children(
        shorter_first_nibble,
        remaining_shorter,
        longer_first_nibble,
        remaining_longer,
    ));
}
```

#### 3. Both keys are not empty, `shorter_first_nibble == longer_first_nibble`.
In that case we extract recursively longest common prefix and then return `Extension` node with longest common prefix as a key with a `Branch` as a child. This `Branch` has two children, each correspoding to one of the entries.

![Schema](./images/into_leaf_2.png)

```rs
let node = from_two_entries(remaining_shorter, remaining_longer)?;

let result_node = match node {
    Node::Branch(_, _) => Node::extension([shorter_first_nibble], node),
    Node::Extension(nibbles, child) => {
        Node::Extension(nibbles.push_front(shorter_first_nibble), child)
    }
    _ => unreachable!("from_two_ordered_entries should return only Branch or Extension"),
};

Ok(result_node)
```
In the above code we only handle cases if resulting node is either a `Branch` or an `Extension`, as this are the only possible node types that `from_two_entries` can return.

## Into Branch