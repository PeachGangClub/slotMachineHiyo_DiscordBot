#[test]
fn isnotzero_test(){
    assert!(!isnotzero(vec![1,1,1,0,0,1,0]));
    assert!(!isnotzero(vec![1,1,0,1,1,1,1]));
    assert!(!isnotzero(vec![1,0,1,0,1,0,1]));
    assert!(!isnotzero(vec![0,1,1,1,0,0,0]));
    assert!(!isnotzero(vec![0,1,0,0,1,1,0]));
    assert!(isnotzero(vec![1,1,1,1,1,1,1]));
}
