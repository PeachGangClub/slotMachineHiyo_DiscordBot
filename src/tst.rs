#[cfg(test)]

mod tests{
    use crate::*;
    #[test]
    fn isnotzero_test(){
        assert!(!hiyokoslot::common::isnotzero(vec![1,1,1,0,0,1,0]));
        assert!(!hiyokoslot::common::isnotzero(vec![1,1,0,1,1,1,1]));
        assert!(!hiyokoslot::common::isnotzero(vec![1,0,1,0,1,0,1]));
        assert!(!hiyokoslot::common::isnotzero(vec![0,1,1,1,0,0,0]));
        assert!(!hiyokoslot::common::isnotzero(vec![0,1,0,0,1,1,0]));
        assert!(hiyokoslot::common::isnotzero(vec![1,1,1,1,1,1,1]));
    }
}

