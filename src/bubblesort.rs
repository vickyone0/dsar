
// pub fn bubble_sort(arr: &mut [i32]) -> &[i32] {
     
//       let len = arr.len();

//       for i in 0..len {
//         for j in 0..len-1-i {
//             if arr[j]> arr[j+1] {
//                 arr.swap(j, j+1);
//             }
//         }
//       }

//       arr
// }

// //time complexity is o(n^2)



// pub fn bubble_sort(arr: &mut [i32]) -> &[i32]{

//   let len = arr.len();

//   for i in 0..len{
//     for j in 0..len-1 -i {
//       if arr[j]> arr[j+1] {
//         arr.swap(j, j+1);
//       }
//     }
//   }
//   return arr;
// }


// pub fn bubble_sort(arr: &mut [i32]) -> &[i32]{

//   for i in 0..arr.len()-1{
//     for j in 0..arr.len()-1-i{
//       if arr[j] > arr[j+1] {
//         arr.swap(j, j+1);
//       }
//     }
//   }
//   arr
// }

pub fn bubble_sort(arr: &mut [i32]) -> & [i32] {
  for i in 0..arr.len()-1 {
    for j in 0..arr.len()-1 -i {
           if arr[j] > arr[j+1] {
            arr.swap(j, j+1);
           }
    }
  }
  arr
}