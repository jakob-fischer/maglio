use crate::kd_tree_traits::*;
use crate::ray_box::*;
use std::collections::HashSet;

struct KdNode<BoundingBox: BoundingBoxTrait, Content: KdTreeContent<BoundingBox>> {
    enclosure: BoundingBox,
    content: Vec<Content>,
    children: Vec<Self>,
}

pub struct KdTree<BoundingBox: BoundingBoxTrait, Content: KdTreeContent<BoundingBox>> {
    root: Option<KdNode<BoundingBox, Content>>,
    size: usize,
}

impl<BoundingBox: BoundingBoxTrait, Content: KdTreeContent<BoundingBox>>
    KdTree<BoundingBox, Content>
{
    fn new_node(enclosure: BoundingBox) -> KdNode<BoundingBox, Content> {
        KdNode::<BoundingBox, Content> {
            enclosure,
            content: vec![],
            children: vec![],
        }
    }

    fn add_to_node(
        node: &mut KdNode<BoundingBox, Content>,
        content: &Content,
        content_enclosure: &BoundingBox,
    ) {
        if content_enclosure.is_sub_scale(&node.enclosure)
            && node.children.is_empty()
            && node.content.len() > 3
        {
            if let Some((left, right)) = node.enclosure.partition() {
                let mut left = Self::new_node(left);
                let mut right = Self::new_node(right);

                for content in &node.content {
                    let enclosure = content.get_bounding_box();
                    if enclosure.intersects(&left.enclosure) {
                        left.content.push(content.clone());
                    }
                    if enclosure.intersects(&right.enclosure) {
                        right.content.push(content.clone());
                    }
                }
                node.content.clear();
                node.children.push(left);
                node.children.push(right);
            }
        }

        if node.children.is_empty() {
            node.content.push(content.clone());
        } else {
            for child in &mut node.children {
                if content_enclosure.intersects(&child.enclosure) {
                    Self::add_to_node(child, content, content_enclosure);
                }
            }
        }
    }

    pub fn new() -> Self {
        KdTree {
            root: Some(Self::new_node(BoundingBox::default())),
            size: 0,
        }
    }

    pub fn add(&mut self, content: Content) {
        let content_enclosure = content.get_bounding_box();

        while !content_enclosure.is_contained(&self.root.as_ref().unwrap().enclosure) {
            let (other, parent) = self.root.as_ref().unwrap().enclosure.extend().unwrap();

            let other_node = Self::new_node(other);
            let current_root = self.root.take().unwrap();
            let mut new_root = Self::new_node(parent);
            new_root.children.push(other_node);
            new_root.children.push(current_root);

            self.root = Some(new_root);
        }

        Self::add_to_node(
            &mut self.root.as_mut().unwrap(),
            &content,
            &content_enclosure,
        );
        self.size += 1;
    }

    fn get_intersection_internal(
        node: &KdNode<BoundingBox, Content>,
        filter: &BoundingBox,
        result: &mut HashSet<Content>,
    ) {
        if !node.enclosure.intersects(filter) {
            return;
        }

        for content in &node.content {
            if content.get_bounding_box().intersects(filter) {
                result.insert(content.clone());
            }
        }

        for child in &node.children {
            Self::get_intersection_internal(child, filter, result);
        }
    }

    pub fn get_intersection(&self, filter: &BoundingBox) -> HashSet<Content> {
        let mut result = HashSet::new();

        if let Some(node) = &self.root {
            Self::get_intersection_internal(node, filter, &mut result);
        }

        result
    }
}

impl<BoundingBox: HittableBoundingBoxTrait, Content: KdTreeContent<BoundingBox>>
    KdTree<BoundingBox, Content>
{
    fn get_closest_hit_internal<F>(
        node: &KdNode<BoundingBox, Content>,
        fun: &F,
        cray: &ConstrainedRay3d,
        result: &mut Option<Content>,
        current: &mut f64,
    ) where
        F: Fn(&Content, &ConstrainedRay3d) -> Option<f64>,
    {
        for content in &node.content {
            if let Some(candidate) = fun(content, cray) {
                if candidate < *current {
                    *current = candidate;
                    *result = Some(content.clone());
                }
            }
        }

        for children in &node.children {
            match children.enclosure.hit(cray) {
                HitBoxResult::Miss => (),
                HitBoxResult::Inside(_) => {
                    Self::get_closest_hit_internal::<F>(children, fun, cray, result, current)
                }
                HitBoxResult::Outside(close, _) => {
                    if close <= *current {
                        Self::get_closest_hit_internal::<F>(children, fun, cray, result, current);
                    }
                }
            }
        }
    }

    pub fn get_closest_hit<F>(&self, fun: &F, cray: &ConstrainedRay3d) -> Option<Content>
    where
        F: Fn(&Content, &ConstrainedRay3d) -> Option<f64>,
    {
        let mut result: Option<Content> = None;
        let mut current = cray.range.1;
        Self::get_closest_hit_internal::<F>(
            &self.root.as_ref().unwrap(),
            fun,
            cray,
            &mut result,
            &mut current,
        );

        result
    }
}

#[test]
fn test_kd_tree_with_points_2d() {
    let mut kd_tree = KdTree::<BoundingBox2d, Point2d>::new();

    kd_tree.add(Point2d::new_raw(0.0, 3.0));
    kd_tree.add(Point2d::new_raw(1.0, 12.0));
    kd_tree.add(Point2d::new_raw(5.0, 2.0));
    kd_tree.add(Point2d::new_raw(-3.0, 2.0));
    kd_tree.add(Point2d::new_raw(3.3, -1.0));
    kd_tree.add(Point2d::new_raw(3.3, 3.0));
    assert_eq!(kd_tree.size, 6);

    {
        let result = kd_tree.get_intersection(&BoundingBox2d {
            u: Point2d::new_raw(0.0, 0.0),
            v: Point2d::new_raw(4.0, 4.0),
        });
        println!("{:?}", result);
        assert_eq!(result.len(), 2);
    }

    {
        let result = kd_tree.get_intersection(&BoundingBox2d {
            u: Point2d::new_raw(2.0, 2.0),
            v: Point2d::new_raw(6.0, 6.0),
        });
        println!("{:?}", result);
        assert_eq!(result.len(), 2);
    }

    {
        let result = kd_tree.get_intersection(&BoundingBox2d {
            u: Point2d::new_raw(-6.0, 1.0),
            v: Point2d::new_raw(6.0, 4.0),
        });
        println!("{:?}", result);
        assert_eq!(result.len(), 4);
    }
}
