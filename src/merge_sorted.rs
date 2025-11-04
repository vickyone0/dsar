pub fn merge_sorted_arrays(nums1: Vec<i32>,nums2:Vec<i32>) -> Vec<i32> {
    let (mut i, mut j) = (0,0);
    let(n,m) = (nums1.len(), nums2.len());
    let mut merged =Vec::with_capacity(n+m);

    while i < n && j < m {
        if nums1[i] < nums2[j] {
            merged.push(nums1[i]);
            i+=1;

        } else {
            merged.push(nums2[j]);
            J +=1;
        }
    }

    if i < n {
        merged.extend_from_slice(&nums1[i..]);
    }
    if j < m {
        merged.extend_from_slice(&nums2[j..]);
    }

    merged
}