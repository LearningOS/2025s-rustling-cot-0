/*
	heap
	This question requires you to implement a binary heap function
*/


use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        // 增加元素计数
        self.count += 1;
        // 将新值添加到元素列表中
        self.items.push(value);
        // 从新添加的元素位置开始向上堆化
        self.heapify_up(self.count);
    }

    // 向上调整堆，以维护堆的性质
    fn heapify_up(&mut self, mut idx: usize) {
        while idx > 1 {
            let parent = self.parent_idx(idx);
            if !(self.comparator)(&self.items[idx], &self.items[parent]) {
                break;
            }
            self.items.swap(idx, parent);
            idx = parent;
        }
    }

    fn heapify_down(&mut self, mut idx: usize) {
        // 从给定的索引开始，向下调整堆以保持堆属性
        loop {
            let left = self.left_child_idx(idx);
            let right = self.right_child_idx(idx);
            // 如果左子节点的索引大于当前堆的元素数量，说明已经是叶子节点，无需继续调整
            if left > self.count {
                break;
            }
            let mut swap = left;
            // 如果右子节点存在且右子节点满足比较条件，则更新swap为右子节点的索引
            if right <= self.count && !(self.comparator)(&self.items[left], &self.items[right]) {
                swap = right;
            }
            // 如果当前节点满足比较条件，则无需交换，退出循环
            if !(self.comparator)(&self.items[swap], &self.items[idx]) {
                break;
            }
            // 交换当前节点与需要调整的子节点
            self.items.swap(idx, swap);
            // 更新当前节点为需要调整的子节点，继续向下调整
            idx = swap;
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        if self.right_child_idx(idx) > self.count {
            self.left_child_idx(idx)
        } else {
            if (self.comparator)(&self.items[self.left_child_idx(idx)], &self.items[self.right_child_idx(idx)]) {
                self.left_child_idx(idx)
            } else {
                self.right_child_idx(idx)
            }
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        // 如果队列为空，返回 None
        if self.is_empty() {
            None
        } else {
            // 弹出并获取队列中的最后一个元素
            let last_item = self.items.pop().unwrap();
            self.count -= 1;
            
            // 如果队列在弹出元素后仍然为空，返回弹出的元素
            if self.is_empty() {
                Some(last_item)
            } else {
                // 将最后一个元素与堆顶元素交换，并重新堆化调整
                let next_item = std::mem::replace(&mut self.items[1], last_item);
                self.heapify_down(1);
                Some(next_item)
            }
        }
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}