use tree::Node;

pub fn is_identical<T: std::cmp::PartialOrd + Copy>(a: &Node<T>, b: &Node<T>) -> bool {
    if a != b {
        return false;
    }

    if a.left != b.left {
        return false;
    }

    if let (Some(ref a), Some(ref b)) = (&a.left, &b.left) {
        return is_identical(a, b);
    }

    if a.right != b.right {
        return false;
    }

    if let (Some(ref a), Some(ref b)) = (&a.right, &b.right) {
        return is_identical(a, b);
    }

    return true;
}
