// pub fn selection_sort(arr: &mut [i32]) -> &[i32]{

//     let len = arr.len();

//     for i in 0..len-1 {
//         let mut min_index = i;

//         for j in i..len {
//             if arr[j]< arr[min_index]{
//                 min_index = j;
//             }
            
//         }
//          if min_index != i {
//             arr.swap(i, min_index);
//         }
//     }

//     arr

// }

//selection sort is o(n^2)


// pub fn selection_sort(arr: &mut[i32]) -> &[i32]{

//     let len = arr.len();

//     for i in 0..len -1 {
//         let mut min_index = i;

//         for j in i..len-1{
//             if arr[j]< arr[min_index] {
//                 min_index = j;
//             }
//         }
//         if min_index != i {
//             arr.swap(i,min_index);
//         }
//     }

//     arr
// }

// pub fn selection_sort(arr:&mut [i32])-> &[i32]{
//     for i in 0..arr.len()-1{
//         let mut min_index = i;

//         for j in i+1..arr.len(){
//             if arr[j] < arr[min_index] {

//                 min_index = j;

//             }
//     }

//     if min_index != i {
//         arr.swap(i, min_index);
//     }

//     }
//     arr
// }

pub fn selection_sort(arr:&mut [i32]) -> &[i32]{

    for i in 0..arr.len() -1 {
        let mut min_index = i;

        for j in i+1..arr.len(){
            if arr[j] < arr[min_index]{
                min_index = j;
            }
        }

        if min_index != i {
            arr.swap(i, min_index);
        }
    }
    arr

}