use std::any::TypeId;
use itertools::any;
use any_vec::any_value::AnyValue;
use any_vec::AnyVec;
use any_vec::traits::Cloneable;

#[test]
fn any_vec_get_test(){
    let mut any_vec: AnyVec<dyn Cloneable> = AnyVec::new::<String>();
    assert_eq!(any_vec.element_typeid(), TypeId::of::<String>());
    {
        let mut vec = any_vec.downcast_mut::<String>().unwrap();
        vec.push(String::from("0"));
        vec.push(String::from("1"));
        vec.push(String::from("2"));
    }

    let e1_ref = any_vec.get(1);
    assert_eq!(e1_ref.downcast_ref::<String>().unwrap(), &String::from("1"));
    assert_eq!(e1_ref.value_typeid(), TypeId::of::<String>());

    {
        let e1 = e1_ref.clone();
        let e2 = (*e1_ref).clone();
        let e3 = (*e1_ref).clone();

        assert_eq!(e1.downcast::<String>(), String::from("1"));
        assert_eq!(e2.downcast::<String>(), String::from("1"));
        assert_eq!(e3.downcast::<String>(), String::from("1"));
    }
}

/*#[test]
fn any_vec_push_to_self_test(){
    let mut any_vec: AnyVec<dyn Cloneable> = AnyVec::new::<String>();
    {
        let mut vec = any_vec.downcast_mut::<String>().unwrap();
        vec.push(String::from("0"));
        vec.push(String::from("1"));
        vec.push(String::from("2"));
    }

    let e = any_vec.get(1);
    any_vec.push((*e).clone());
}*/