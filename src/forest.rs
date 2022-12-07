#[derive(Debug)]
struct ForestNode<T> {
    value: T,
    parent: Option<usize>,
    next_sibling: Option<usize>,
    first_child: Option<usize>,
}

impl<T> ForestNode<T> {
    pub fn new(v: T, parent: Option<usize>, next_sibling: Option<usize>) -> Self {
        ForestNode {
            value: v,
            parent,
            next_sibling,
            first_child: None,
        }
    }
}

pub struct Forest<T> {
    nodes: Vec<ForestNode<T>>,
    pub root: Option<usize>,
}

pub struct SiblingIter<'a, T> {
    forest: &'a Forest<T>,
    cursor: Option<usize>,
}

pub struct AncestorIter<'a, T> {
    forest: &'a Forest<T>,
    cursor: Option<usize>,
}

pub struct PostOrderIter<'a, T> {
    forest: &'a Forest<T>,
    cursor: Option<usize>,
}

impl<T> Forest<T> {
    pub fn new() -> Self {
        Forest {
            nodes: Vec::new(),
            root: None,
        }
    }

    fn drill_down(&self, mut id: usize) -> usize {
        while self.first_child(id).is_some() {
            id = self.first_child(id).unwrap();
        }
        id
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn parent(&self, id: usize) -> Option<usize> {
        self.nodes[id].parent
    }

    pub fn is_leaf(&self, id: usize) -> bool {
        self.first_child(id).is_none()
    }

    pub fn next_sibling(&self, id: usize) -> Option<usize> {
        self.nodes[id].next_sibling
    }

    pub fn first_child(&self, id: usize) -> Option<usize> {
        self.nodes[id].first_child
    }

    pub fn roots(&self) -> SiblingIter<T> {
        SiblingIter {
            forest: self,
            cursor: self.root,
        }
    }

    pub fn children(&self, id: usize) -> SiblingIter<T> {
        SiblingIter {
            forest: self,
            cursor: self.first_child(id),
        }
    }

    pub fn ancestors(&self, id: usize) -> AncestorIter<T> {
        AncestorIter {
            forest: self,
            cursor: self.parent(id),
        }
    }

    pub fn ancestors_and_self(&self, id: usize) -> AncestorIter<T> {
        AncestorIter {
            forest: self,
            cursor: Some(id),
        }
    }

    pub fn post_order(&self, id: usize) -> PostOrderIter<T> {
        PostOrderIter {
            forest: self,
            cursor: Some(self.drill_down(id)),
        }
    }

    pub fn post_order_root(&self) -> PostOrderIter<T> {
        PostOrderIter {
            forest: self,
            cursor: self.root.map(|r| self.drill_down(r)),
        }
    }

    pub fn insert_root(&mut self, v: T) -> usize {
        let id = self.nodes.len();
        self.nodes.push(ForestNode::new(v, None, self.root));
        self.root = Some(id);
        id
    }

    pub fn prepend_child(&mut self, parent_id: usize, v: T) -> usize {
        let id = self.nodes.len();
        let first_child = self.first_child(parent_id);
        self.nodes
            .push(ForestNode::new(v, Some(parent_id), first_child));
        self.nodes[parent_id].first_child = Some(id);
        id
    }

    pub fn append_child(&mut self, parent_id: usize, v: T) -> usize {
        let id = self.nodes.len();
        let first_child = self.first_child(parent_id);
        self.nodes.push(ForestNode::new(v, Some(parent_id), None));

        if let Some(mut cid) = first_child {
            while self.next_sibling(cid).is_some() {
                cid = self.next_sibling(cid).unwrap()
            }
            self.nodes[cid].next_sibling = Some(id);
        } else {
            self.nodes[parent_id].first_child = Some(id);
        };

        id
    }
}

impl<T> std::ops::Index<usize> for Forest<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.nodes.index(index).value
    }
}

impl<T> std::ops::IndexMut<usize> for Forest<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.nodes.index_mut(index).value
    }
}

impl<T> Default for Forest<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: std::fmt::Debug> Forest<T> {
    pub fn dump(&self) {
        self.nodes
            .iter()
            .enumerate()
            .for_each(|n| println!("{:?}", n));
    }
}

impl<'a, T> Iterator for SiblingIter<'a, T> {
    type Item = (&'a Forest<T>, usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(id) = self.cursor {
            self.cursor = self.forest.next_sibling(id);
            Some((self.forest, id, &self.forest[id]))
        } else {
            None
        }
    }
}

impl<'a, T> Iterator for AncestorIter<'a, T> {
    type Item = (&'a Forest<T>, usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(id) = self.cursor {
            self.cursor = self.forest.parent(id);
            Some((self.forest, id, &self.forest[id]))
        } else {
            None
        }
    }
}

impl<'a, T> Iterator for PostOrderIter<'a, T> {
    type Item = (&'a Forest<T>, usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(id) = self.cursor {
            if let Some(s) = self.forest.next_sibling(id) {
                self.cursor = Some(self.forest.drill_down(s))
            } else if let Some(p) = self.forest.parent(id) {
                self.cursor = Some(p)
            } else {
                self.cursor = None
            }
            Some((self.forest, id, &self.forest[id]))
        } else {
            None
        }
    }
}
