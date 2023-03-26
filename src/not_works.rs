use std::os::raw::{c_char, c_void};
use jni::objects::JList;
use core::ptr::null;
use jni::objects::JListIter;
use jni::objects::AutoLocal;

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

// This keeps Rust from "mangling" the name and making it unique for this
// crate.
#[no_mangle]
pub extern "system" fn Java_org_example_lab2_HelloWorld_hello<'local>(mut env: JNIEnv<'local>,
// This is the class that owns our static method. It's not going to be used,
// but still must be present to match the expected signature of a static
// native method.
                                                     class: JClass<'local>,
                                                     input: JString<'local>)
                                                     -> jstring {
    // First, we have to get the string out of Java. Check out the `strings`
    // module for more info on how this works.
    let input: String =
        env.get_string(&input).expect("Couldn't get java string!").into();

    // Then we have to create a new Java string to return. Again, more info
    // in the `strings` module.
    let output = env.new_string(format!("Hello, {}!", input))
        .expect("Couldn't create java string!");

    // Finally, extract the raw pointer to return.
    output.into_raw()

}

#[repr(C)]
pub struct Product {
    id: i32,
    name: String,
    description: String,
    price: f32,
    amount: i32,
    measure: Measure,
}
impl Default for Product {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::from(""),
            description: String::from(""),
            price: 0.0,
            amount: 0,
            measure: Measure::XS,
        }
    }
}
impl Default for JListIter {
    fn default() -> Self {
        JListIter {
            obj: std::ptr::null_mut(),
            pos: 0,
            size: 0,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub enum Measure {
    XS,
    S,
    M,
    L,
    XL,
}

//method expected to get unsorted List<Product> and return sorted List<Products>
#[no_mangle]
pub extern "system" fn Java_org_example_lab2_Lab2_bubbleSort<'local>(mut env: JNIEnv<'local>,
                                                                     class: JClass<'local>,
                                                                     input: JObject<'local>)
                                                     -> jobject {


// Convert the Java list of Products to a Rust Vec of Products
    let products: Vec<Product> = match JList::from_env(&mut env, &mut input) {
        Err(e) => vec!(Product::default()),
        Ok(r) => {
            let mut iterator = match r.iter(&mut env){
                Ok(r) => r,
                Err(e) => return env.throw_new("java/lang/RuntimeException", &format!("Error iterating over list: {}", e)).unwrap(),
            };

            while let Some(obj) = iterator.next(&mut env)? {
                let obj: AutoLocal<JObject> = env.auto_local(obj);

                obj
                    .map(|obj| {
                        let product_cls = env.get_object_class(obj).unwrap();
                        let id_field_id = env.get_field_id(product_cls, "id", "I").unwrap();
                        let name_field_id = env.get_field_id(product_cls, "name", "Ljava/lang/String;").unwrap();
                        let description_field_id = env.get_field_id(product_cls, "description", "Ljava/lang/String;").unwrap();
                        let price_field_id = env.get_field_id(product_cls, "price", "F").unwrap();
                        let amount_field_id = env.get_field_id(product_cls, "amount", "I").unwrap();
                        let measure_field_id = env.get_field_id(product_cls, "measure", "LMeasure;").unwrap();
                        let id = env.get_field(obj, id_field_id).unwrap().i().unwrap();
                        let name = env.get_string(env.get_field(obj, name_field_id).unwrap().l().unwrap()).unwrap().to_string();
                        let description = env.get_string(env.get_field(obj, description_field_id).unwrap().l().unwrap()).unwrap().to_string();
                        let price = env.get_field(obj, price_field_id).unwrap().f().unwrap();
                        let amount = env.get_field(obj, amount_field_id).unwrap().i().unwrap();
                        let measure_obj = env.get_field(obj, measure_field_id).unwrap().l().unwrap();
                        let measure = match measure_obj.is_null() {
                            true => None,
                            false => Some(match env.get_enum_field(measure_obj, "value", "LMeasure;").unwrap().i().unwrap() {
                                0 => Measure::XS,
                                1 => Measure::S,
                                2 => Measure::M,
                                3 => Measure::L,
                                4 => Measure::XL,
                                _ => panic!("Invalid enum value"),
                            }),
                        };
                        Product {
                            id,
                            name,
                            description,
                            price,
                            amount,
                            measure,
                        }
                    })
                    .collect()
            }
        }
    };

    // Perform bubble sort on the products vector
    let len = products.len();
    for i in 0..len {
        for j in (i + 1)..len {
            if products[j].id < products[i].id {
                products.swap(i, j);
            }
        }
    }

    // Convert the sorted Rust Vec of Products back to a Java ArrayList of Products
    let arraylist_cls = env.find_class("java/util/ArrayList").unwrap();
    let arraylist_constructor = env.get_method_id(arraylist_cls, "<init>", "()V").unwrap();
    let arraylist_obj = env.new_object(arraylist_cls, arraylist_constructor, &[]).unwrap();
    let arraylist_add_method = env.get_method_id(arraylist_cls, "add", "(Ljava/lang/Object;)Z").unwrap();
for product in products {
    let product_cls = env.find_class("org/example/lab2/Product").unwrap();
    let product_constructor = env.get_method_id(product_cls, "<init>", "(ILjava/lang/String;Ljava/lang/String;FIILorg/example/lab2/Measure;)V").unwrap();
    let product_name = env.new_string(product.name).unwrap();
    let product_description = env.new_string(product.description).unwrap();
    let measure_cls = env.find_class("org/example/lab2/Measure").unwrap();
    let measure_constructor = env.get_method_id(measure_cls, "<init>", "(Ljava/lang/String;)V").unwrap();
    let measure_obj = match product.measure {
        None => ptr::null_mut(),
        Some(measure) => {
            let measure_name = match measure {
                Measure::XS => "XS",
                Measure::S => "S",
                Measure::M => "M",
                Measure::L => "L",
                Measure::XL => "XL",
            };
            let measure_name_obj = env.new_string(measure_name).unwrap();
            env.new_object(measure_cls, measure_constructor, &[measure_name_obj.into()]).unwrap()
        },
    };
    let product_obj = env.new_object(product_cls, product_constructor, &[product.id.into(), product_name.into(), product_description.into(), product.price.into(), product.amount.into(), measure_obj.into()]).unwrap();
    env.call_method(arraylist_obj, arraylist_add_method, &[product_obj.into()]).unwrap();
}

arraylist_obj.into_raw()


}

pub extern "system" fn Java_org_example_lab2_Lab2_insertionSort<'local>(mut env: JNIEnv<'local>,
                                                                     class: JClass<'local>,
                                                                     input: JObject<'local>)
                                                     -> () {




}

pub extern "system" fn Java_org_example_lab2_Lab2_selectionSort<'local>(mut env: JNIEnv<'local>,
                                                                     class: JClass<'local>,
                                                                     input: JObject<'local>)
                                                     -> () {




}

pub extern "system" fn Java_org_example_lab2_Lab2_shellSort<'local>(mut env: JNIEnv<'local>,
                                                                     class: JClass<'local>,
                                                                     input: JObject<'local>)
                                                     -> () {




}