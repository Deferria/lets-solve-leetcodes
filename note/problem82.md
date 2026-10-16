# Leetcode Problem 82. Remove Duplicates from Sorted List II [goto source](https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/)

## Problem Description

You are given the head of a sorted linked list.

Delete all nodes that have duplicate numbers, leaving only distinct numbers from the original list.

Return the linked list sorted as well.

### Constraints

- The number of nodes in the list is in the range [0, 300].
- -100 <= `Node.val` <= 100
- The list is guaranteed to be sorted in ascending order.

### Examples

- [1,2,3,3,4,4,5] -> [1,2,5]
- [1,1,1,2,3] -> [2,3]

## Solutions

### Navigator

- [Solution 1: 直接再新建一个链表](#solution-1-直接再新建一个链表)
- [Solution 2: 遇到一样的就丢弃](#solution-2-遇到一样的就丢弃)

### Solution 1: 直接再新建一个链表

考虑到 Rust 语言繁琐的引用检查机制，如果我们直接在原链表上操作，必然会遇到引用和所有权的问题，所以为了简化步骤，我们干脆把原链表当成不可变的，直接从它里面读值然后新造一个链表出来，由于数据类型是 `i32`, 空间复杂度最坏也是 `O(n)`, 不会带来很严重的内存开销，所以不失为一条计策。

**1. 新建链表**

```rust
    let mut dummy = Box::new(ListNode::new(i32::MAX));
    let mut tl = &mut dummy;
```

新建一个 `dummy` 节点，作为新链表的头节点，`tl` 指针指向新链表的尾部。

**2. 准备遍历变量**

```rust
    let (mut cval, mut cnt) = (i32::MAX, 0);
    let mut curr = head.as_ref();
```

`cval` 记录当前值，`cnt` 记录当前值的出现次数，`curr` 指针指向原链表的当前节点。由于 `head` 不可变，所以我们直接使用 `as_ref()` 方法获取一个不可变引用。

> 其实这里也可以使用 `&head` 获得不可变引用, 这样获得的引用类型是 `&Option<Box<ListNode>>`, 也可以使用 `as_ref()` 方法获得不可变引用，获得的引用类型是 `Option<&Box<ListNode>>`. 二者在 `unwrap()` 之后都会得到 `&Box<ListNode>`.

**3. 遍历链表**

```rust
    while let Some(node) = curr {
        let temp = node.val;
        ...
```

解包 `curr`，如果当前节点存在，则获取当前节点的值存入 `temp`。

```rust
        if temp == cval {
            cnt += 1;
        } else {
            if cnt == 1 {
                tl.next = Some(Box::new(ListNode::new(cval)));
                tl = tl.next.as_mut().unwrap();
            }
            cval = temp;
            cnt = 1;
        }
```

判断：如果当前节点值就是原先存下的 `cval`, 则 `cnt` 加一，否则，说明当前节点值和原先存下的 `cval` 不同了，说明我们已经遍历完了所有的 `cval` 节点，这时候再执行判断：如果 `cnt == 1`, 说明 `cval` 是唯一的，我们就把它加入新链表中，然后更新 `cval` 和 `cnt`.

给链表尾插新节点只需要先新建一个 `next` 为空的新节点，然后将 `tl` 指针指向新节点，最后将 `tl` 指针移动到新节点即可。注意 `tl` 的类型是 `&mut Box<ListNode>`, 所以我们需要使用 `as_mut()` 方法获取一个可变引用，然后再使用 `unwrap()` 方法解包，这样可以保证全局只有一个可变引用。


```rust
        curr = node.next.as_ref();
```

这里是遍历链表的将指针移到下一个节点的操作，由于 `curr` 是不可变引用，所以我们使用 `as_ref()` 方法获取下一个节点的不可变引用。

**4. 处理最后一个节点**

```rust
    if cnt == 1 {
        tl.next = Some(Box::new(ListNode::new(cval)));
    }
```

原因很简单, 如果最后一个节点是唯一的，那么在循环结束后，`cnt` 的值就是 1, 但我们的操作是在 `cval` 更新时进行的，所以我们需要在循环结束后再判断一次 `cnt`, 如果 `cnt == 1`, 就把最后一个节点加入新链表中。

```
    dummy.next
```

节点被接在 `dummy` 的后面, 所以返回 `dummy.next` 即可。

> 这也是我想到的第一种方法，也是最简单、最直观的方法.

### Solution 2: 遇到一样的就丢弃

好，现在我们要尝试原地修改链表了。其实思路也很清晰，用一个指针去扫一遍链表，在此过程中，每一次都比较和下一个节点的值，如果一样就取下来，如果不一样就往下继续。

**1. 定义哑节点**

```rust
    let mut dummy = Some(Box::new(ListNode {
        val: i32::MAX,
        next: head,
    }));
    let mut tl = &mut dummy;
```

直接把 `head` 接在 `dummy` 的后面，从此往后我们只考虑 `dummy` 后面的节点。

这里使用 `Some` 是为了保证 `tl` 的类型一致性，现在 `tl` 的类型是 `&mut Option<Box<ListNode>>`, 粗略地理解就是指向指针的指针。

> 与上面一样, 也可以定义 `tl = dummy.as_mut()`, 这样 `tl` 的类型就是 `Option<&mut Box<ListNode>>`, 使用 `unwrap()` 方法解包后就是 `&mut Box<ListNode>`, 这样就可以直接使用 `tl.next` 来访问下一个节点了。

**2. 开始遍历**

```rust
    while let Some(node) = tl.as_mut() {
```

解包 `tl`.

```rust
        let dup_flag = node
            .next
            .as_ref()
            .and_then(|n1| n1.next.as_ref().map(|n2| n1.val == n2.val))
            .unwrap_or(false);
```

这里是判断是否需要删除当前节点的标志，需要详细解释: 

1. `node` 具有类型 `&mut Box<ListNode>`, 所以 `node.next` 的类型是 `Option<Box<ListNode>>`, 也就是指向下一个节点的指针.
2. `node.next.as_ref()` 的类型是 `Option<&Box<ListNode>>`, 也就是穿透 Option 的指向下一个节点的不可变引用. 由于我们并不想现在就修改，所以使用 `as_ref()` 方法获取不可变引用.
3. `and_then()` 是 Option 泛型中的方法，意思是: 如果 `node.next` 是 `None`, 那么直接返回 `None`, 否则就把 `node.next` 解包成 `n1`, 然后执行闭包函数，再把结果包装成 `Option`. 这是 Rust 独有的链式处理 Option 的方法。
4. 那我们要怎么处理 `n1` 呢? 首先得判断 `n1` 有没有下一个节点, 如果有, 就判断它的值与 `n1` 的值是否相等. 所以第一步自然是得到 `n1.next`, 同样加上 `as_ref()`, 然后将它映到一个 `boolean` 值, 这也就是后面的 `map(|n2| n1.val == n2.val)` 的作用。
5. 如果 `n1.next` 是 `None`, 那么就返回 `None`, 否则就返回 `Some(true)` 或者 `Some(false)`. 这样我们就得到了一个 `Option<bool>`, 最后使用 `unwrap_or(false)` 方法, 如果需要解包的是 `None`, 就返回 `false`, 否则就返回里面的值.

一番操作下来就实现了即使下面节点可能不存在, 也能判断当前节点是否需要删除的功能。

**3. 执行删除（丢弃）**

```rust
        if dup_flag {
            let vl = node.next.as_ref().unwrap().val;
```

先把需要比较的值存起来。

```rust
            let mut nxt = node.next.take();
```

这里使用 `take()` 方法将 `node.next` 的值取出来，这样 `node.next` 就变成了 `None`, 也就是丢弃了当前节点的下一个节点，并且数据所有权现在已经转移到了 `nxt` 变量上。

```rust
            while let Some(m) = nxt.as_ref() {
```

仍然使用 `Some` 解包并存下节点. `as_ref()` 也是必须使用的, 凡是只获取值的操作一律加上 `as_ref()`, 这样就不会触发所有权转移, 也就不会触发借用检查机制.

```rust
                if m.val == vl {
                    nxt = nxt.as_mut().unwrap().next.take();
                }
```

这里是重点: 要更新 `nxt` 指针，因为 `nxt` 的类型是 `Option<Box<ListNode>>`, 也就是一个链表，所以得先用 `unwrap()` 解包，再使用 `next` 指针指向下一个节点，然后使用 `take()` 方法取出下一个节点的所有权，这样就可以丢弃当前节点了。当然这还没完，`take()` 方法需要节点是可变的，怎么办呢？只能给 `nxt` 加上 `as_mut()` 方法获取可变引用，然后再解包。

这里绝不能使用 `m.next.take()`, 因为 `m` 是不可变引用，无法使用 `take()` 方法；也不能标记 `Some(mut m) = nxt`, 因为 `m` 本来就是 `as_ref()` 得到的, 不是 `as_mut()` 得到的.

```rust
                } else {
                    node.next = nxt.take();
                }
```

如果不相同, 那就说明当前节点的下一个节点是唯一的, 我们就把它接到 `node` 的后面, 然后结束循环. 接入的方法是用 `take()` 方法把 `nxt` 的所有权直接转移到 `node.next` 上, 这样就不会触发借用检查机制.

**4. 移动指针

```rust
        } else {
            tl = &mut tl.as_mut().unwrap().next
        }
```

原因在于, `tl` 的类型是 `&mut Option<Box<ListNode>>`, 所以我们需要使用 `as_mut()` 方法获取可变引用，然后再使用 `unwrap()` 方法解包，这样就可以访问 `tl.next` 了。最后再把 `tl` 指针指向下一个节点。

**5. 返回结果**

```rust
    dummy.unwrap().next
```

原因在于现在的 `dummy` 还在 `Some` 里面。