extern crate weld;

use weld::*;
use weld::common::WeldRuntimeErrno;

mod common;
use common::*;

#[test]
fn simple_log() {
    let code = "|x:f64| log(x)";
    let conf = default_conf();
    let input = 2.718281828459045;
    let ret_value = compile_and_run(code, conf, &input);
    let data = unsafe { weld_value_data(ret_value) as *const f64 };
    let result = unsafe { (*data).clone() };
    let output = 1.0f64;
    assert!(approx_equal(output, result, 5));
    unsafe { free_value_and_module(ret_value) };
}

#[test]
fn log_error() {
    let code = "|x:i64| log(x)";
    let conf = default_conf();
    let input = 1;
    let err_value = compile_and_run_error(code, conf, &input);
    assert_eq!(unsafe { weld_error_code(err_value) },
               WeldRuntimeErrno::CompileError);
    unsafe { weld_error_free(err_value) };
}

#[test]
fn simple_exp() {
    let code = "|x:f64| exp(x)";
    let conf = default_conf();
    let input = 1.0f64;
    let ret_value = compile_and_run(code, conf, &input);
    let data = unsafe { weld_value_data(ret_value) as *const f64 };
    let result = unsafe { (*data).clone() };
    let output = 2.718281828459045;
    assert!(approx_equal(output, result, 5));
    unsafe { free_value_and_module(ret_value) };
}

#[test]
fn exp_error() {
    let code = "|x:i64| exp(x)";
    let conf = default_conf();
    let input = 1;
    let err_value = compile_and_run_error(code, conf, &input);
    assert_eq!(unsafe { weld_error_code(err_value) },
               WeldRuntimeErrno::CompileError);
    unsafe { weld_error_free(err_value) };
}

#[test]
fn simple_erf() {
    let code = "|x:f64| erf(x)";
    let conf = default_conf();
    let input = 1.00;
    let ret_value = compile_and_run(code, conf, &input);
    let data = unsafe { weld_value_data(ret_value) as *const f64 };
    let result = unsafe { (*data).clone() };
    let output = 0.84270079294971478;
    assert!(approx_equal(output, result, 5));
    unsafe { free_value_and_module(ret_value) };
}

#[test]
fn simple_sqrt() {
    let code = "|x:f64| sqrt(x)";
    let conf = default_conf();
    let input = 4.0;
    let ret_value = compile_and_run(code, conf, &input);
    let data = unsafe { weld_value_data(ret_value) as *const f64 };

    let result = unsafe { (*data).clone() };
    let output = 2.0f64;
    assert!(approx_equal(output, result, 5));
    unsafe { free_value_and_module(ret_value) };
}

#[test]
fn simple_pow() {
    use std::f64;
    let code = "|x:f64| pow(x, 2.0)";
    let conf = default_conf();
    let input = 4.0;
    let ret_value = compile_and_run(code, conf, &input);
    let data = unsafe { weld_value_data(ret_value) as *const f64 };

    let result = unsafe { (*data).clone() };
    assert!(approx_equal(16.0, result, 5));
    unsafe { free_value_and_module(ret_value) };
}

#[test]
fn simple_trig() {
    fn check_trig_f32(op: &str, input: f32, expect: f32) {
        let code = format!("|x:f32| {}(x)", op);
        let conf = default_conf();
        let ret_value = compile_and_run(&code, conf, &input);
        let data = unsafe { weld_value_data(ret_value) as *const f32 };
        let result = unsafe { (*data).clone() };
        assert!(approx_equal_f32(expect, result, 5));
        unsafe { free_value_and_module(ret_value) };
    }

    fn check_trig_f64(op: &str, input: f64, expect: f64) {
        let code = format!("|x:f64| {}(x)", op);
        let conf = default_conf();
        let ret_value = compile_and_run(&code, conf, &input);
        let data = unsafe { weld_value_data(ret_value) as *const f64 };
        let result = unsafe { (*data).clone() };
        assert!(approx_equal(expect, result, 5));
        unsafe { free_value_and_module(ret_value) };
    }

    let inp: f32 = 1.0;
    check_trig_f32("sin", inp, inp.sin());
    check_trig_f32("cos", inp, inp.cos());
    check_trig_f32("tan", inp, inp.tan());
    check_trig_f32("asin", inp, inp.asin());
    check_trig_f32("acos", inp, inp.acos());
    check_trig_f32("atan", inp, inp.atan());
    check_trig_f32("sinh", inp, inp.sinh());
    check_trig_f32("cosh", inp, inp.cosh());
    check_trig_f32("tanh", inp, inp.tanh());

    let inp: f64 = 1.0;
    check_trig_f64("sin", inp, inp.sin());
    check_trig_f64("cos", inp, inp.cos());
    check_trig_f64("tan", inp, inp.tan());
    check_trig_f64("asin", inp, inp.asin());
    check_trig_f64("acos", inp, inp.acos());
    check_trig_f64("atan", inp, inp.atan());
    check_trig_f64("sinh", inp, inp.sinh());
    check_trig_f64("cosh", inp, inp.cosh());
    check_trig_f64("tanh", inp, inp.tanh());
}

#[test]
fn map_exp() {
    let code = "|x:vec[f32]| map(x, |a| exp(a))";
    let conf = default_conf();

    let input_vec = [0.0f32, 1.0f32, 2.0f32, 3.0f32];
    let ref input_data = WeldVec {
        data: &input_vec as *const f32,
        len: input_vec.len() as i64,
    };

    let ret_value = compile_and_run(code, conf, input_data);
    let data = unsafe { weld_value_data(ret_value) as *const WeldVec<f32> };
    let result = unsafe { (*data).clone() };

    let output = [1.0, 2.7182817, 7.389056, 20.085537];
    for i in 0..(result.len as isize) {
        assert_eq!(unsafe { *result.data.offset(i) }, output[i as usize])
    }

    unsafe { free_value_and_module(ret_value) };
}

#[test]
fn simple_int_mod() {
    let code = "|x:i64| x % 3L";
    let conf = default_conf();
    let ref input_data: i64 = -10;
    let ret_value = compile_and_run(code, conf, input_data);
    let data = unsafe { weld_value_data(ret_value) as *const i64 };
    let result = unsafe { *data };
    assert_eq!(result, -1);
    unsafe { free_value_and_module(ret_value) };
}

#[test]
fn simple_float_mod() {
    let code = "|x:f64| x % 0.04";
    let conf = default_conf();
    let ref input_data: f64 = 0.5;
    let ret_value = compile_and_run(code, conf, input_data);
    let data = unsafe { weld_value_data(ret_value) as *const f64 };
    let result = unsafe { *data };
    assert!(approx_equal(result, 0.02, 5));
    unsafe { free_value_and_module(ret_value) };
}
