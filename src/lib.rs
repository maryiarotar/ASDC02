
/* for creating linux library to install dependencies for linux and then
rustc ----target=x86_64-unknown-linux-gnu .... ** */

extern crate serde_json;
extern crate serde;

use std::fs;
use std::time::Instant;
use serde_json::Value;
use serde::{Serialize, Deserialize};
use std::fs::{OpenOptions, File};
use std::error::Error;

use std::os::raw::{c_char, c_void};
use jni::JNIEnv; // interface to the JVM for methods.

// These objects are what you should use as arguments to your native
// function. They carry extra lifetime information to prevent them escaping
// this context and getting used after being GC'd.
use jni::objects::{JClass, JString, JObject};

// This is just a pointer. We'll be returning it from our function. We
// can't return one of the objects with lifetime information because the
// lifetime checker won't let us.
use jni::sys::jstring;

use jni::sys::jobject;

use std::io::Write;



#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Product {
    id: i64,
    name: String,
    description: String,
    price: f64,
    amount: i64,
    measure: String,
}

 //impl Clone for Product {}

// impl <'a > From<&'a Product> for Product {

//     fn from(other: &'a Product) {
//  Product {
//             id: other.id,
//             name: other.name.clone(),
//             description: other.description.clone(),
//             price: other.price,
//             amount: other.amount,
//             measure: other.measure.clone(),
//         }
//     }

// }



/* all methods return path to new sorted files */


#[no_mangle]
pub extern "system" fn Java_org_example_lab2_Lab2_bubbleSort<'local>(mut env: JNIEnv<'local>,
                                                     class: JClass<'local>,
                                                     input: JString<'local>)
                                                     -> jstring {

    // Getting the string out of Java.
    let filepath: String =
        env.get_string(&input).expect("Couldn't get java string!").into();


    let mut array :Vec<Box<Product>> = Vec::new();
    get_array_from_file(filepath, &mut array);
    let mut number_of_comparisons :i32 = 0;
    let mut number_of_permutations :i32  = 0;


    let start = Instant::now();
    //------------------------ BUBBLE SORT -----------------------------\\
    loop {
        let mut check = true;
        for i in 1..array.len() {
            number_of_comparisons += 1;
            if array[i - 1].id > array[i].id {
                number_of_permutations += 1;
                array.swap(i, i - 1);
                check = false;
            }
        }
        if check == true { break; }
    }
    //-------------------------------------------------------------------\\
    let end = start.elapsed();


    let new_file :String = "./BUBBLE_SORTED.json".to_string();
    write_to_file(&new_file, &array);


    // creating a new Java string to return.
    let output = env.new_string(format!("<<<Bubble sorting... \
    {} comparisons & {} permutations has done>>>.\n Sorted file is: {} \n
    performance = {} seconds ",
        number_of_comparisons, number_of_permutations, new_file, end.as_secs_f64()))
        .expect("Couldn't create java string!");



    // Extracting the raw pointer to return.
    output.into_raw()

}




#[no_mangle]
pub extern "system" fn Java_org_example_lab2_Lab2_insertionSort<'local>(mut env: JNIEnv<'local>,
                                                     class: JClass<'local>,
                                                     input: JString<'local>)
                                                     -> jstring {

    let filepath: String =
        env.get_string(&input).expect("Couldn't get java string!").into();


    let mut array :Vec<Box<Product>> = Vec::new();
    get_array_from_file(filepath, &mut array);
    let mut number_of_comparisons :i32 = 0;
    let mut number_of_permutations :i32  = 0;


    let start = Instant::now();
    //------------------------ INSERTION SORT -----------------------------\\
    for i in 1..array.len(){
        let cur = array[i].clone();
        let mut j = i;
    
        number_of_comparisons += 1; 
    
       while j>0 && (array[j-1].id > cur.id) {
            number_of_permutations += 1;
            number_of_comparisons += 1;
    
           let ind = array[j-1].clone();
            array[j]= ind;
            j -= 1;
        }
        
        array[j]= cur;
    
        number_of_permutations += 1;
    }
    
    //-------------------------------------------------------------------\\
    let end = start.elapsed();


    let new_file :String = "./INSERTION_SORTED.json".to_string();
    write_to_file(&new_file, &array);


    // creating a new Java string to return.
    let output = env.new_string(format!("<<<Insertion sorting... \
    {} comparisons & {} permutations has done>>>.\n Sorted file is: {} \n
    performance = {} seconds ",
        number_of_comparisons, number_of_permutations, new_file, end.as_secs_f64()))
        .expect("Couldn't create java string!");


    // Extracting the raw pointer to return.
    output.into_raw()
                                             


}


#[no_mangle]
pub extern "system" fn Java_org_example_lab2_Lab2_selectionSort<'local>(mut env: JNIEnv<'local>,
                                                     class: JClass<'local>,
                                                     input: JString<'local>)
                                                     -> jstring {

    let filepath: String =
        env.get_string(&input).expect("Couldn't get java string!").into();


    let mut array :Vec<Box<Product>> = Vec::new();
    get_array_from_file(filepath, &mut array);
    let mut number_of_comparisons :i32 = 0;
    let mut number_of_permutations :i32  = 0;


    let start = Instant::now();
    //------------------------ SELECTION SORT -----------------------------\\
    for i in 0..array.len()-1{

        let mut min = i;
    
       for j in i+1..array.len() {
    
            number_of_comparisons += 1;
    
            if array[j].id < array[min].id {
                min = j;
            }
        }
        array.swap(min, i);
        number_of_permutations += 1;
    
    }
    
    //-------------------------------------------------------------------\\
    let end = start.elapsed();

    
    let new_file :String = "./SELECTION_SORTED.json".to_string();
    write_to_file(&new_file, &array);
    


    // creating a new Java string to return.
    let output = env.new_string(format!("<<<Selection sorting... \
    {} comparisons & {} permutations has done>>>.\n Sorted file is: {} \n
    performance = {} seconds ",
        number_of_comparisons, number_of_permutations, new_file, end.as_secs_f64()))
        .expect("Couldn't create java string!");



    output.into_raw()
                                            
}



#[no_mangle]
pub extern "system" fn Java_org_example_lab2_Lab2_shellSort<'local>(mut env: JNIEnv<'local>,
                                                                     class: JClass<'local>,
                                                                     input: JString<'local>)
                                                     -> jstring {


    let filepath: String =
        env.get_string(&input).expect("Couldn't get java string!").into();


    let mut array :Vec<Box<Product>> = Vec::new();
    get_array_from_file(filepath, &mut array);
    let mut number_of_comparisons :i32 = 0;
    let mut number_of_permutations :i32  = 0;


    let start = Instant::now();
    //------------------------ SHELL SORT -----------------------------\\

    let mut s = array.len()/2;
    while s > 0 {
       for i in s..array.len() {
    
        number_of_comparisons += 1; 
    
        let mut j = i;
    
       while j >= s && (array[j-s].id > array[j].id) {
            number_of_permutations += 1;
    
            array.swap(j, j-s);
            j -= s;
            number_of_comparisons += 1;
    
        }
       }
        s /= 2;
    
    }
    //-------------------------------------------------------------------\\
    let end = start.elapsed();
    

    let new_file :String = "./SHELL_SORTED.json".to_string();
    write_to_file(&new_file, &array);
    


    // creating a new Java string to return.
    let output = env.new_string(format!("<<<Shell sorting... \
    {} comparisons & {} permutations has done>>>.\n Sorted file is: {} \n
    performance = {} seconds ",
        number_of_comparisons, number_of_permutations, new_file, end.as_secs_f64()))
        .expect("Couldn't create java string!");



    output.into_raw()

}






#[no_mangle]
pub extern "system" fn Java_org_example_lab2_Lab2_mergeSort<'local>(mut env: JNIEnv<'local>,
                                                     class: JClass<'local>,
                                                     input: JString<'local>)
                                                     -> jstring {

    let filepath: String =
        env.get_string(&input).expect("Couldn't get java string!").into();


    let mut array :Vec<Box<Product>> = Vec::new();
    get_array_from_file(filepath, &mut array);
    let mut number_of_comparisons :i32 = 0;
    let mut number_of_permutations :i32  = 0;


    //------------------------ MERGE SORT -----------------------------\\
 
fn merge_sort(ar: &mut Vec<Box<Product>>, number_of_comparisons: &mut i32,
    number_of_permutations: &mut i32) -> Vec<Box<Product>> {

    let mut sorted_arr: Vec<Box<Product>> = Vec::with_capacity(ar.len());
    *number_of_comparisons += 1;

    if ar.len() < 2 {
        return ar.to_vec();
    }

    else {

        let mut pivot = ar.len()/2;

        let mut left = merge_sort(&mut ar[0..pivot].to_vec(),
        number_of_comparisons, number_of_permutations);
        let mut right = merge_sort(&mut ar[pivot..].to_vec(),
        number_of_comparisons, number_of_permutations);

        let mut i = 0;
        let mut j = 0;
        let mut merged: Vec<Box<Product>> = Vec::new();
    
        while i < left.len() && j < right.len() {
            *number_of_comparisons += 1;

            if left[i].id < right[j].id {
                merged.push(left[i].clone());
                i = i + 1;

                *number_of_permutations += 1;

            } else {
                merged.push(right[j].clone());
                j = j + 1;
                
                *number_of_permutations += 1;
            }
        }
        *number_of_comparisons += 1;

        if i < left.len() {         
            merged.extend_from_slice(&left[i..]);
        }
        *number_of_comparisons += 1;

        if j < right.len() {
            merged.extend_from_slice(&right[j..]);
        }
    
        merged
    }
    
}

let start = Instant::now();
    
let mut a = merge_sort(&mut array, &mut number_of_comparisons,
&mut number_of_permutations);
//-------------------------------------------------------------------\\
let end = start.elapsed();



let new_file :String = "./MERGE.json".to_string();
write_to_file(&new_file, &a);

    // creating a new Java string to return.
    let output = env.new_string(format!("<<<Merge sorting... \
    {} comparisons & {} permutations has done>>>.\n Sorted file is: {} \n
    performance = {} seconds ",
        number_of_comparisons, number_of_permutations, new_file, end.as_secs_f64()))
        .expect("Couldn't create java string!");


    // Extracting the raw pointer to return.
    output.into_raw()
                                             


}









/* function to reading json from file and parsing it to vector */

pub fn get_array_from_file(path :String, array: &mut Vec<Box<Product>>) {
    /* reading from file */
    let data :String = fs::read_to_string(path)
        .expect("Unable to read file");

    /* parse to Value, that presents .json */
    let json: serde_json::Value = serde_json::from_str(&data)
        .expect("Json isn't in correct format!");

    //dbg!(json);

    //let mut array :Vec<&Product> = Vec::new();

    /* casting elements of Value to struct 'Product' and adding it to vector */
    if let Value::Array(arr) = json {
        for elem in arr.iter() {
            if let Value::Object(obj) = elem {

                let mut prod = Product::default();

                prod.id = obj.get("id").unwrap().as_i64().unwrap();
                prod.name = obj.get("name").unwrap().as_str().unwrap().parse().unwrap();
                prod.description = obj.get("description").unwrap().as_str().unwrap().parse().unwrap();
                prod.price = obj.get("price").unwrap().as_f64().unwrap();
                prod.amount = obj.get("amount").unwrap().as_i64().unwrap();
                prod.measure = obj.get("measure").unwrap().as_str().unwrap().parse().unwrap();

                array.push(Box::new(prod));
            }
        }
    }

}


/* function to write json to file */


pub fn write_to_file(new_file :&String, array :&Vec<Box<Product>>) {

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(new_file)
        .unwrap();


    for elem in array{
        serde_json::to_writer(&file, elem)
         .map_err(|err| {
            eprintln!("Error writing to file: {}", err);
        })
        .is_ok();
        
        write!(&file, "\n").unwrap();
    }

}