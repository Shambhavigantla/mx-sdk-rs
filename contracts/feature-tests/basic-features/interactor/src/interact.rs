#![allow(non_snake_case)]

pub mod config;
mod basic_features_proxy;

use config::Config;
use multiversx_sc_snippets::imports::*;
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    path::Path,
};

const STATE_FILE: &str = "state.toml";

pub async fn basic_features_cli() {
    env_logger::init();

    let mut args = std::env::args();
    let _ = args.next();
    let cmd = args.next().expect("at least one argument required");
    let config = Config::new();
    let mut interact = ContractInteract::new(config).await;
    match cmd.as_str() {
        "deploy" => interact.deploy().await,
        "panicWithMessage" => interact.panic_with_message().await,
        "count_ones" => interact.count_ones().await,
        "endpoint_with_mutable_arg" => interact.endpoint_with_mutable_arg().await,
        "get_esdt_token_data" => interact.get_esdt_token_data().await,
        "sqrt_big_uint" => interact.sqrt_big_uint().await,
        "sqrt_big_uint_ref" => interact.sqrt_big_uint_ref().await,
        "log2_big_uint" => interact.log2_big_uint().await,
        "log2_big_uint_ref" => interact.log2_big_uint_ref().await,
        "pow_big_int" => interact.pow_big_int().await,
        "pow_big_int_ref" => interact.pow_big_int_ref().await,
        "pow_big_uint" => interact.pow_big_uint().await,
        "pow_big_uint_ref" => interact.pow_big_uint_ref().await,
        "big_uint_to_u64" => interact.big_uint_to_u64().await,
        "biguint_overwrite_u64" => interact.biguint_overwrite_u64().await,
        "big_uint_zero" => interact.big_uint_zero().await,
        "big_uint_from_u64_1" => interact.big_uint_from_u64_1().await,
        "big_uint_from_u64_2" => interact.big_uint_from_u64_2().await,
        "biguint_from_u128" => interact.biguint_from_u128().await,
        "big_uint_from_managed_buffer" => interact.big_uint_from_managed_buffer().await,
        "big_uint_from_managed_buffer_ref" => interact.big_uint_from_managed_buffer_ref().await,
        "big_int_zero" => interact.big_int_zero().await,
        "big_int_from_i64_1" => interact.big_int_from_i64_1().await,
        "big_int_from_i64_2" => interact.big_int_from_i64_2().await,
        "big_uint_eq_u64" => interact.big_uint_eq_u64().await,
        "big_int_to_i64" => interact.big_int_to_i64().await,
        "bigint_overwrite_i64" => interact.bigint_overwrite_i64().await,
        "big_int_to_parts" => interact.big_int_to_parts().await,
        "big_int_from_biguint" => interact.big_int_from_biguint().await,
        "add_big_int_big_uint" => interact.add_big_int_big_uint().await,
        "add_big_uint_big_int" => interact.add_big_uint_big_int().await,
        "add_big_int_big_uint_ref" => interact.add_big_int_big_uint_ref().await,
        "add_big_uint_big_int_ref" => interact.add_big_uint_big_int_ref().await,
        "add_big_int_big_int" => interact.add_big_int_big_int().await,
        "add_big_int_big_int_ref" => interact.add_big_int_big_int_ref().await,
        "add_big_int_ref_big_int" => interact.add_big_int_ref_big_int().await,
        "add_big_int_ref_big_int_ref" => interact.add_big_int_ref_big_int_ref().await,
        "add_big_uint_big_uint" => interact.add_big_uint_big_uint().await,
        "add_big_uint_big_uint_ref" => interact.add_big_uint_big_uint_ref().await,
        "add_big_uint_ref_big_uint" => interact.add_big_uint_ref_big_uint().await,
        "add_big_uint_ref_big_uint_ref" => interact.add_big_uint_ref_big_uint_ref().await,
        "add_big_uint_u32" => interact.add_big_uint_u32().await,
        "add_big_uint_ref_u32" => interact.add_big_uint_ref_u32().await,
        "add_big_uint_u64" => interact.add_big_uint_u64().await,
        "add_big_uint_ref_u64" => interact.add_big_uint_ref_u64().await,
        "add_non_zero_big_uint_non_zero_big_uint" => interact.add_non_zero_big_uint_non_zero_big_uint().await,
        "add_non_zero_big_uint_non_zero_big_uint_ref" => interact.add_non_zero_big_uint_non_zero_big_uint_ref().await,
        "add_non_zero_big_uint_ref_non_zero_big_uint" => interact.add_non_zero_big_uint_ref_non_zero_big_uint().await,
        "add_non_zero_big_uint_ref_non_zero_big_uint_ref" => interact.add_non_zero_big_uint_ref_non_zero_big_uint_ref().await,
        "add_non_zero_big_uint_u32" => interact.add_non_zero_big_uint_u32().await,
        "add_non_zero_big_uint_ref_u32" => interact.add_non_zero_big_uint_ref_u32().await,
        "add_non_zero_big_uint_u64" => interact.add_non_zero_big_uint_u64().await,
        "add_non_zero_big_uint_ref_u64" => interact.add_non_zero_big_uint_ref_u64().await,
        "sub_big_int_big_int" => interact.sub_big_int_big_int().await,
        "sub_big_int_big_int_ref" => interact.sub_big_int_big_int_ref().await,
        "sub_big_int_ref_big_int" => interact.sub_big_int_ref_big_int().await,
        "sub_big_int_ref_big_int_ref" => interact.sub_big_int_ref_big_int_ref().await,
        "sub_big_uint_big_uint" => interact.sub_big_uint_big_uint().await,
        "sub_big_uint_big_uint_ref" => interact.sub_big_uint_big_uint_ref().await,
        "sub_big_uint_ref_big_uint" => interact.sub_big_uint_ref_big_uint().await,
        "sub_big_uint_ref_big_uint_ref" => interact.sub_big_uint_ref_big_uint_ref().await,
        "sub_big_uint_u32" => interact.sub_big_uint_u32().await,
        "sub_big_uint_ref_u32" => interact.sub_big_uint_ref_u32().await,
        "sub_big_uint_u64" => interact.sub_big_uint_u64().await,
        "sub_big_uint_ref_u64" => interact.sub_big_uint_ref_u64().await,
        "sub_non_zero_big_uint_non_zero_big_uint" => interact.sub_non_zero_big_uint_non_zero_big_uint().await,
        "sub_non_zero_big_uint_non_zero_big_uint_ref" => interact.sub_non_zero_big_uint_non_zero_big_uint_ref().await,
        "sub_non_zero_big_uint_ref_non_zero_big_uint" => interact.sub_non_zero_big_uint_ref_non_zero_big_uint().await,
        "sub_non_zero_big_uint_ref_non_zero_big_uint_ref" => interact.sub_non_zero_big_uint_ref_non_zero_big_uint_ref().await,
        "sub_non_zero_big_uint_u32" => interact.sub_non_zero_big_uint_u32().await,
        "sub_non_zero_big_uint_ref_u32" => interact.sub_non_zero_big_uint_ref_u32().await,
        "sub_non_zero_big_uint_u64" => interact.sub_non_zero_big_uint_u64().await,
        "sub_non_zero_big_uint_ref_u64" => interact.sub_non_zero_big_uint_ref_u64().await,
        "mul_big_int_big_int" => interact.mul_big_int_big_int().await,
        "mul_big_int_big_int_ref" => interact.mul_big_int_big_int_ref().await,
        "mul_big_int_ref_big_int" => interact.mul_big_int_ref_big_int().await,
        "mul_big_int_ref_big_int_ref" => interact.mul_big_int_ref_big_int_ref().await,
        "mul_big_uint_big_uint" => interact.mul_big_uint_big_uint().await,
        "mul_big_uint_big_uint_ref" => interact.mul_big_uint_big_uint_ref().await,
        "mul_big_uint_ref_big_uint" => interact.mul_big_uint_ref_big_uint().await,
        "mul_big_uint_ref_big_uint_ref" => interact.mul_big_uint_ref_big_uint_ref().await,
        "mul_big_uint_u32" => interact.mul_big_uint_u32().await,
        "mul_big_uint_ref_u32" => interact.mul_big_uint_ref_u32().await,
        "mul_big_uint_u64" => interact.mul_big_uint_u64().await,
        "mul_big_uint_ref_u64" => interact.mul_big_uint_ref_u64().await,
        "mul_non_zero_big_uint_non_zero_big_uint" => interact.mul_non_zero_big_uint_non_zero_big_uint().await,
        "mul_non_zero_big_uint_non_zero_big_uint_ref" => interact.mul_non_zero_big_uint_non_zero_big_uint_ref().await,
        "mul_non_zero_big_uint_ref_non_zero_big_uint" => interact.mul_non_zero_big_uint_ref_non_zero_big_uint().await,
        "mul_non_zero_big_uint_ref_non_zero_big_uint_ref" => interact.mul_non_zero_big_uint_ref_non_zero_big_uint_ref().await,
        "mul_non_zero_big_uint_u32" => interact.mul_non_zero_big_uint_u32().await,
        "mul_non_zero_big_uint_ref_u32" => interact.mul_non_zero_big_uint_ref_u32().await,
        "mul_non_zero_big_uint_u64" => interact.mul_non_zero_big_uint_u64().await,
        "mul_non_zero_big_uint_ref_u64" => interact.mul_non_zero_big_uint_ref_u64().await,
        "div_big_int_big_int" => interact.div_big_int_big_int().await,
        "div_big_int_big_int_ref" => interact.div_big_int_big_int_ref().await,
        "div_big_int_ref_big_int" => interact.div_big_int_ref_big_int().await,
        "div_big_int_ref_big_int_ref" => interact.div_big_int_ref_big_int_ref().await,
        "div_big_uint_big_uint" => interact.div_big_uint_big_uint().await,
        "div_big_uint_big_uint_ref" => interact.div_big_uint_big_uint_ref().await,
        "div_big_uint_ref_big_uint" => interact.div_big_uint_ref_big_uint().await,
        "div_big_uint_ref_big_uint_ref" => interact.div_big_uint_ref_big_uint_ref().await,
        "div_big_uint_u32" => interact.div_big_uint_u32().await,
        "div_big_uint_ref_u32" => interact.div_big_uint_ref_u32().await,
        "div_big_uint_u64" => interact.div_big_uint_u64().await,
        "div_big_uint_ref_u64" => interact.div_big_uint_ref_u64().await,
        "div_non_zero_big_uint_non_zero_big_uint" => interact.div_non_zero_big_uint_non_zero_big_uint().await,
        "div_non_zero_big_uint_non_zero_big_uint_ref" => interact.div_non_zero_big_uint_non_zero_big_uint_ref().await,
        "div_non_zero_big_uint_ref_non_zero_big_uint" => interact.div_non_zero_big_uint_ref_non_zero_big_uint().await,
        "div_non_zero_big_uint_ref_non_zero_big_uint_ref" => interact.div_non_zero_big_uint_ref_non_zero_big_uint_ref().await,
        "div_non_zero_big_uint_u32" => interact.div_non_zero_big_uint_u32().await,
        "div_non_zero_big_uint_ref_u32" => interact.div_non_zero_big_uint_ref_u32().await,
        "div_non_zero_big_uint_u64" => interact.div_non_zero_big_uint_u64().await,
        "div_non_zero_big_uint_ref_u64" => interact.div_non_zero_big_uint_ref_u64().await,
        "rem_big_int_big_int" => interact.rem_big_int_big_int().await,
        "rem_big_int_big_int_ref" => interact.rem_big_int_big_int_ref().await,
        "rem_big_int_ref_big_int" => interact.rem_big_int_ref_big_int().await,
        "rem_big_int_ref_big_int_ref" => interact.rem_big_int_ref_big_int_ref().await,
        "rem_big_uint_big_uint" => interact.rem_big_uint_big_uint().await,
        "rem_big_uint_big_uint_ref" => interact.rem_big_uint_big_uint_ref().await,
        "rem_big_uint_ref_big_uint" => interact.rem_big_uint_ref_big_uint().await,
        "rem_big_uint_ref_big_uint_ref" => interact.rem_big_uint_ref_big_uint_ref().await,
        "rem_big_uint_u32" => interact.rem_big_uint_u32().await,
        "rem_big_uint_ref_u32" => interact.rem_big_uint_ref_u32().await,
        "rem_big_uint_u64" => interact.rem_big_uint_u64().await,
        "rem_big_uint_ref_u64" => interact.rem_big_uint_ref_u64().await,
        "rem_non_zero_big_uint_non_zero_big_uint" => interact.rem_non_zero_big_uint_non_zero_big_uint().await,
        "rem_non_zero_big_uint_non_zero_big_uint_ref" => interact.rem_non_zero_big_uint_non_zero_big_uint_ref().await,
        "rem_non_zero_big_uint_ref_non_zero_big_uint" => interact.rem_non_zero_big_uint_ref_non_zero_big_uint().await,
        "rem_non_zero_big_uint_ref_non_zero_big_uint_ref" => interact.rem_non_zero_big_uint_ref_non_zero_big_uint_ref().await,
        "rem_non_zero_big_uint_u32" => interact.rem_non_zero_big_uint_u32().await,
        "rem_non_zero_big_uint_ref_u32" => interact.rem_non_zero_big_uint_ref_u32().await,
        "rem_non_zero_big_uint_u64" => interact.rem_non_zero_big_uint_u64().await,
        "rem_non_zero_big_uint_ref_u64" => interact.rem_non_zero_big_uint_ref_u64().await,
        "add_assign_big_int_big_int" => interact.add_assign_big_int_big_int().await,
        "add_assign_big_int_big_int_ref" => interact.add_assign_big_int_big_int_ref().await,
        "add_assign_big_uint_big_uint" => interact.add_assign_big_uint_big_uint().await,
        "add_assign_big_uint_big_uint_ref" => interact.add_assign_big_uint_big_uint_ref().await,
        "add_assign_big_uint_u32" => interact.add_assign_big_uint_u32().await,
        "add_assign_big_uint_u64" => interact.add_assign_big_uint_u64().await,
        "add_assign_non_zero_big_uint_non_zero_big_uint" => interact.add_assign_non_zero_big_uint_non_zero_big_uint().await,
        "add_assign_non_zero_big_uint_non_zero_big_uint_ref" => interact.add_assign_non_zero_big_uint_non_zero_big_uint_ref().await,
        "add_assign_non_zero_big_uint_big_uint" => interact.add_assign_non_zero_big_uint_big_uint().await,
        "add_assign_non_zero_big_uint_big_uint_ref" => interact.add_assign_non_zero_big_uint_big_uint_ref().await,
        "add_assign_non_zero_big_uint_u32" => interact.add_assign_non_zero_big_uint_u32().await,
        "add_assign_non_zero_big_uint_u64" => interact.add_assign_non_zero_big_uint_u64().await,
        "sub_assign_big_int_big_int" => interact.sub_assign_big_int_big_int().await,
        "sub_assign_big_int_big_int_ref" => interact.sub_assign_big_int_big_int_ref().await,
        "sub_assign_big_uint_big_uint" => interact.sub_assign_big_uint_big_uint().await,
        "sub_assign_big_uint_big_uint_ref" => interact.sub_assign_big_uint_big_uint_ref().await,
        "sub_assign_big_uint_u32" => interact.sub_assign_big_uint_u32().await,
        "sub_assign_big_uint_u64" => interact.sub_assign_big_uint_u64().await,
        "sub_assign_non_zero_big_uint_non_zero_big_uint" => interact.sub_assign_non_zero_big_uint_non_zero_big_uint().await,
        "sub_assign_non_zero_big_uint_non_zero_big_uint_ref" => interact.sub_assign_non_zero_big_uint_non_zero_big_uint_ref().await,
        "sub_assign_non_zero_big_uint_big_uint" => interact.sub_assign_non_zero_big_uint_big_uint().await,
        "sub_assign_non_zero_big_uint_big_uint_ref" => interact.sub_assign_non_zero_big_uint_big_uint_ref().await,
        "sub_assign_non_zero_big_uint_u32" => interact.sub_assign_non_zero_big_uint_u32().await,
        "sub_assign_non_zero_big_uint_u64" => interact.sub_assign_non_zero_big_uint_u64().await,
        "mul_assign_big_int_big_int" => interact.mul_assign_big_int_big_int().await,
        "mul_assign_big_int_big_int_ref" => interact.mul_assign_big_int_big_int_ref().await,
        "mul_assign_big_uint_big_uint" => interact.mul_assign_big_uint_big_uint().await,
        "mul_assign_big_uint_big_uint_ref" => interact.mul_assign_big_uint_big_uint_ref().await,
        "mul_assign_big_uint_u32" => interact.mul_assign_big_uint_u32().await,
        "mul_assign_big_uint_u64" => interact.mul_assign_big_uint_u64().await,
        "mul_assign_non_zero_big_uint_non_zero_big_uint" => interact.mul_assign_non_zero_big_uint_non_zero_big_uint().await,
        "mul_assign_non_zero_big_uint_non_zero_big_uint_ref" => interact.mul_assign_non_zero_big_uint_non_zero_big_uint_ref().await,
        "mul_assign_non_zero_big_uint_big_uint" => interact.mul_assign_non_zero_big_uint_big_uint().await,
        "mul_assign_non_zero_big_uint_big_uint_ref" => interact.mul_assign_non_zero_big_uint_big_uint_ref().await,
        "mul_assign_non_zero_big_uint_u32" => interact.mul_assign_non_zero_big_uint_u32().await,
        "mul_assign_non_zero_big_uint_u64" => interact.mul_assign_non_zero_big_uint_u64().await,
        "div_assign_big_int_big_int" => interact.div_assign_big_int_big_int().await,
        "div_assign_big_int_big_int_ref" => interact.div_assign_big_int_big_int_ref().await,
        "div_assign_big_uint_big_uint" => interact.div_assign_big_uint_big_uint().await,
        "div_assign_big_uint_big_uint_ref" => interact.div_assign_big_uint_big_uint_ref().await,
        "div_assign_big_uint_u32" => interact.div_assign_big_uint_u32().await,
        "div_assign_big_uint_u64" => interact.div_assign_big_uint_u64().await,
        "div_assign_non_zero_big_uint_non_zero_big_uint" => interact.div_assign_non_zero_big_uint_non_zero_big_uint().await,
        "div_assign_non_zero_big_uint_non_zero_big_uint_ref" => interact.div_assign_non_zero_big_uint_non_zero_big_uint_ref().await,
        "div_assign_non_zero_big_uint_big_uint" => interact.div_assign_non_zero_big_uint_big_uint().await,
        "div_assign_non_zero_big_uint_big_uint_ref" => interact.div_assign_non_zero_big_uint_big_uint_ref().await,
        "div_assign_non_zero_big_uint_u32" => interact.div_assign_non_zero_big_uint_u32().await,
        "div_assign_non_zero_big_uint_u64" => interact.div_assign_non_zero_big_uint_u64().await,
        "rem_assign_big_int_big_int" => interact.rem_assign_big_int_big_int().await,
        "rem_assign_big_int_big_int_ref" => interact.rem_assign_big_int_big_int_ref().await,
        "rem_assign_big_uint_big_uint" => interact.rem_assign_big_uint_big_uint().await,
        "rem_assign_big_uint_big_uint_ref" => interact.rem_assign_big_uint_big_uint_ref().await,
        "rem_assign_big_uint_u32" => interact.rem_assign_big_uint_u32().await,
        "rem_assign_big_uint_u64" => interact.rem_assign_big_uint_u64().await,
        "rem_assign_non_zero_big_uint_non_zero_big_uint" => interact.rem_assign_non_zero_big_uint_non_zero_big_uint().await,
        "rem_assign_non_zero_big_uint_non_zero_big_uint_ref" => interact.rem_assign_non_zero_big_uint_non_zero_big_uint_ref().await,
        "rem_assign_non_zero_big_uint_big_uint" => interact.rem_assign_non_zero_big_uint_big_uint().await,
        "rem_assign_non_zero_big_uint_big_uint_ref" => interact.rem_assign_non_zero_big_uint_big_uint_ref().await,
        "rem_assign_non_zero_big_uint_u32" => interact.rem_assign_non_zero_big_uint_u32().await,
        "rem_assign_non_zero_big_uint_u64" => interact.rem_assign_non_zero_big_uint_u64().await,
        "bit_and_big_uint_big_uint" => interact.bit_and_big_uint_big_uint().await,
        "bit_and_big_uint_big_uint_ref" => interact.bit_and_big_uint_big_uint_ref().await,
        "bit_and_big_uint_ref_big_uint" => interact.bit_and_big_uint_ref_big_uint().await,
        "bit_and_big_uint_ref_big_uint_ref" => interact.bit_and_big_uint_ref_big_uint_ref().await,
        "bit_and_big_uint_u32" => interact.bit_and_big_uint_u32().await,
        "bit_and_big_uint_ref_u32" => interact.bit_and_big_uint_ref_u32().await,
        "bit_and_big_uint_u64" => interact.bit_and_big_uint_u64().await,
        "bit_and_big_uint_ref_u64" => interact.bit_and_big_uint_ref_u64().await,
        "bit_or_big_uint_big_uint" => interact.bit_or_big_uint_big_uint().await,
        "bit_or_big_uint_big_uint_ref" => interact.bit_or_big_uint_big_uint_ref().await,
        "bit_or_big_uint_ref_big_uint" => interact.bit_or_big_uint_ref_big_uint().await,
        "bit_or_big_uint_ref_big_uint_ref" => interact.bit_or_big_uint_ref_big_uint_ref().await,
        "bit_or_big_uint_u32" => interact.bit_or_big_uint_u32().await,
        "bit_or_big_uint_ref_u32" => interact.bit_or_big_uint_ref_u32().await,
        "bit_or_big_uint_u64" => interact.bit_or_big_uint_u64().await,
        "bit_or_big_uint_ref_u64" => interact.bit_or_big_uint_ref_u64().await,
        "bit_xor_big_uint_big_uint" => interact.bit_xor_big_uint_big_uint().await,
        "bit_xor_big_uint_big_uint_ref" => interact.bit_xor_big_uint_big_uint_ref().await,
        "bit_xor_big_uint_ref_big_uint" => interact.bit_xor_big_uint_ref_big_uint().await,
        "bit_xor_big_uint_ref_big_uint_ref" => interact.bit_xor_big_uint_ref_big_uint_ref().await,
        "bit_xor_big_uint_u32" => interact.bit_xor_big_uint_u32().await,
        "bit_xor_big_uint_ref_u32" => interact.bit_xor_big_uint_ref_u32().await,
        "bit_xor_big_uint_u64" => interact.bit_xor_big_uint_u64().await,
        "bit_xor_big_uint_ref_u64" => interact.bit_xor_big_uint_ref_u64().await,
        "bit_and_assign_big_uint_big_uint" => interact.bit_and_assign_big_uint_big_uint().await,
        "bit_and_assign_big_uint_big_uint_ref" => interact.bit_and_assign_big_uint_big_uint_ref().await,
        "bit_and_assign_big_uint_u32" => interact.bit_and_assign_big_uint_u32().await,
        "bit_and_assign_big_uint_u64" => interact.bit_and_assign_big_uint_u64().await,
        "bit_or_assign_big_uint_big_uint" => interact.bit_or_assign_big_uint_big_uint().await,
        "bit_or_assign_big_uint_big_uint_ref" => interact.bit_or_assign_big_uint_big_uint_ref().await,
        "bit_or_assign_big_uint_u32" => interact.bit_or_assign_big_uint_u32().await,
        "bit_or_assign_big_uint_u64" => interact.bit_or_assign_big_uint_u64().await,
        "bit_xor_assign_big_uint_big_uint" => interact.bit_xor_assign_big_uint_big_uint().await,
        "bit_xor_assign_big_uint_big_uint_ref" => interact.bit_xor_assign_big_uint_big_uint_ref().await,
        "bit_xor_assign_big_uint_u32" => interact.bit_xor_assign_big_uint_u32().await,
        "bit_xor_assign_big_uint_u64" => interact.bit_xor_assign_big_uint_u64().await,
        "shr_big_uint_usize" => interact.shr_big_uint_usize().await,
        "shr_big_uint_ref_usize" => interact.shr_big_uint_ref_usize().await,
        "shl_big_uint_usize" => interact.shl_big_uint_usize().await,
        "shl_big_uint_ref_usize" => interact.shl_big_uint_ref_usize().await,
        "shr_assign_big_uint_usize" => interact.shr_assign_big_uint_usize().await,
        "shl_assign_big_uint_usize" => interact.shl_assign_big_uint_usize().await,
        "eq_big_int_big_int" => interact.eq_big_int_big_int().await,
        "eq_big_int_i32" => interact.eq_big_int_i32().await,
        "eq_big_int_i64" => interact.eq_big_int_i64().await,
        "eq_big_int_u32" => interact.eq_big_int_u32().await,
        "eq_big_int_u64" => interact.eq_big_int_u64().await,
        "eq_big_uint_big_uint" => interact.eq_big_uint_big_uint().await,
        "eq_big_uint_i32" => interact.eq_big_uint_i32().await,
        "eq_big_uint_i64" => interact.eq_big_uint_i64().await,
        "eq_big_uint_u32" => interact.eq_big_uint_u32().await,
        "eq_big_uint_u64" => interact.eq_big_uint_u64().await,
        "eq_non_zero_big_uint_non_zero_big_uint" => interact.eq_non_zero_big_uint_non_zero_big_uint().await,
        "eq_non_zero_big_uint_big_uint" => interact.eq_non_zero_big_uint_big_uint().await,
        "eq_non_zero_big_uint_i32" => interact.eq_non_zero_big_uint_i32().await,
        "eq_non_zero_big_uint_i64" => interact.eq_non_zero_big_uint_i64().await,
        "eq_non_zero_big_uint_u32" => interact.eq_non_zero_big_uint_u32().await,
        "eq_non_zero_big_uint_u64" => interact.eq_non_zero_big_uint_u64().await,
        "gt_big_int_big_int" => interact.gt_big_int_big_int().await,
        "gt_big_int_i32" => interact.gt_big_int_i32().await,
        "gt_big_int_i64" => interact.gt_big_int_i64().await,
        "gt_big_int_u32" => interact.gt_big_int_u32().await,
        "gt_big_int_u64" => interact.gt_big_int_u64().await,
        "gt_big_uint_big_uint" => interact.gt_big_uint_big_uint().await,
        "gt_big_uint_i32" => interact.gt_big_uint_i32().await,
        "gt_big_uint_i64" => interact.gt_big_uint_i64().await,
        "gt_big_uint_u32" => interact.gt_big_uint_u32().await,
        "gt_big_uint_u64" => interact.gt_big_uint_u64().await,
        "gt_non_zero_big_uint_non_zero_big_uint" => interact.gt_non_zero_big_uint_non_zero_big_uint().await,
        "gt_non_zero_big_uint_big_uint" => interact.gt_non_zero_big_uint_big_uint().await,
        "gt_non_zero_big_uint_i32" => interact.gt_non_zero_big_uint_i32().await,
        "gt_non_zero_big_uint_i64" => interact.gt_non_zero_big_uint_i64().await,
        "gt_non_zero_big_uint_u32" => interact.gt_non_zero_big_uint_u32().await,
        "gt_non_zero_big_uint_u64" => interact.gt_non_zero_big_uint_u64().await,
        "ge_big_int_big_int" => interact.ge_big_int_big_int().await,
        "ge_big_int_i32" => interact.ge_big_int_i32().await,
        "ge_big_int_i64" => interact.ge_big_int_i64().await,
        "ge_big_int_u32" => interact.ge_big_int_u32().await,
        "ge_big_int_u64" => interact.ge_big_int_u64().await,
        "ge_big_uint_big_uint" => interact.ge_big_uint_big_uint().await,
        "ge_big_uint_i32" => interact.ge_big_uint_i32().await,
        "ge_big_uint_i64" => interact.ge_big_uint_i64().await,
        "ge_big_uint_u32" => interact.ge_big_uint_u32().await,
        "ge_big_uint_u64" => interact.ge_big_uint_u64().await,
        "ge_non_zero_big_uint_non_zero_big_uint" => interact.ge_non_zero_big_uint_non_zero_big_uint().await,
        "ge_non_zero_big_uint_big_uint" => interact.ge_non_zero_big_uint_big_uint().await,
        "ge_non_zero_big_uint_i32" => interact.ge_non_zero_big_uint_i32().await,
        "ge_non_zero_big_uint_i64" => interact.ge_non_zero_big_uint_i64().await,
        "ge_non_zero_big_uint_u32" => interact.ge_non_zero_big_uint_u32().await,
        "ge_non_zero_big_uint_u64" => interact.ge_non_zero_big_uint_u64().await,
        "lt_big_int_big_int" => interact.lt_big_int_big_int().await,
        "lt_big_int_i32" => interact.lt_big_int_i32().await,
        "lt_big_int_i64" => interact.lt_big_int_i64().await,
        "lt_big_int_u32" => interact.lt_big_int_u32().await,
        "lt_big_int_u64" => interact.lt_big_int_u64().await,
        "lt_big_uint_big_uint" => interact.lt_big_uint_big_uint().await,
        "lt_big_uint_i32" => interact.lt_big_uint_i32().await,
        "lt_big_uint_i64" => interact.lt_big_uint_i64().await,
        "lt_big_uint_u32" => interact.lt_big_uint_u32().await,
        "lt_big_uint_u64" => interact.lt_big_uint_u64().await,
        "lt_non_zero_big_uint_non_zero_big_uint" => interact.lt_non_zero_big_uint_non_zero_big_uint().await,
        "lt_non_zero_big_uint_big_uint" => interact.lt_non_zero_big_uint_big_uint().await,
        "lt_non_zero_big_uint_i32" => interact.lt_non_zero_big_uint_i32().await,
        "lt_non_zero_big_uint_i64" => interact.lt_non_zero_big_uint_i64().await,
        "lt_non_zero_big_uint_u32" => interact.lt_non_zero_big_uint_u32().await,
        "lt_non_zero_big_uint_u64" => interact.lt_non_zero_big_uint_u64().await,
        "le_big_int_big_int" => interact.le_big_int_big_int().await,
        "le_big_int_i32" => interact.le_big_int_i32().await,
        "le_big_int_i64" => interact.le_big_int_i64().await,
        "le_big_int_u32" => interact.le_big_int_u32().await,
        "le_big_int_u64" => interact.le_big_int_u64().await,
        "le_big_uint_big_uint" => interact.le_big_uint_big_uint().await,
        "le_big_uint_i32" => interact.le_big_uint_i32().await,
        "le_big_uint_i64" => interact.le_big_uint_i64().await,
        "le_big_uint_u32" => interact.le_big_uint_u32().await,
        "le_big_uint_u64" => interact.le_big_uint_u64().await,
        "le_non_zero_big_uint_non_zero_big_uint" => interact.le_non_zero_big_uint_non_zero_big_uint().await,
        "le_non_zero_big_uint_big_uint" => interact.le_non_zero_big_uint_big_uint().await,
        "le_non_zero_big_uint_i32" => interact.le_non_zero_big_uint_i32().await,
        "le_non_zero_big_uint_i64" => interact.le_non_zero_big_uint_i64().await,
        "le_non_zero_big_uint_u32" => interact.le_non_zero_big_uint_u32().await,
        "le_non_zero_big_uint_u64" => interact.le_non_zero_big_uint_u64().await,
        "get_block_timestamp" => interact.get_block_timestamp().await,
        "get_block_nonce" => interact.get_block_nonce().await,
        "get_block_round" => interact.get_block_round().await,
        "get_block_epoch" => interact.get_block_epoch().await,
        "get_block_random_seed" => interact.get_block_random_seed().await,
        "get_prev_block_timestamp" => interact.get_prev_block_timestamp().await,
        "get_prev_block_nonce" => interact.get_prev_block_nonce().await,
        "get_prev_block_round" => interact.get_prev_block_round().await,
        "get_prev_block_epoch" => interact.get_prev_block_epoch().await,
        "get_prev_block_random_seed" => interact.get_prev_block_random_seed().await,
        "epoch_info" => interact.epoch_info().await,
        "code_hash" => interact.code_hash().await,
        "get_block_timestamps" => interact.get_block_timestamps().await,
        "get_block_timestamp_ms" => interact.get_block_timestamp_ms().await,
        "get_prev_block_timestamp_ms" => interact.get_prev_block_timestamp_ms().await,
        "get_caller" => interact.get_caller().await,
        "get_owner_address" => interact.get_owner_address().await,
        "get_shard_of_address" => interact.get_shard_of_address().await,
        "is_smart_contract" => interact.is_smart_contract().await,
        "get_state_root_hash" => interact.get_state_root_hash().await,
        "get_tx_hash" => interact.get_tx_hash().await,
        "get_gas_left" => interact.get_gas_left().await,
        "get_cumulated_validator_rewards" => interact.get_cumulated_validator_rewards().await,
        "get_code_metadata" => interact.get_code_metadata().await,
        "is_builtin_function" => interact.is_builtin_function().await,
        "codec_err_finish" => interact.codec_err_finish().await,
        "codec_err_storage_key" => interact.codec_err_storage_key().await,
        "codec_err_storage_get" => interact.codec_err_storage_get().await,
        "codec_err_storage_set" => interact.codec_err_storage_set().await,
        "codec_err_event_topic" => interact.codec_err_event_topic().await,
        "codec_err_event_data" => interact.codec_err_event_data().await,
        "codec_err_contract_init" => interact.codec_err_contract_init().await,
        "codec_err_contract_call" => interact.codec_err_contract_call().await,
        "compute_sha256" => interact.compute_sha256().await,
        "compute_keccak256" => interact.compute_keccak256().await,
        "compute_ripemd160" => interact.compute_ripemd160().await,
        "verify_bls_signature" => interact.verify_bls_signature().await,
        "verify_ed25519_signature" => interact.verify_ed25519_signature().await,
        "verify_secp256k1_signature" => interact.verify_secp256k1_signature().await,
        "verify_custom_secp256k1_signature" => interact.verify_custom_secp256k1_signature().await,
        "compute_secp256k1_der_signature" => interact.compute_secp256k1_der_signature().await,
        "verify_secp256r1_signature" => interact.verify_secp256r1_signature().await,
        "verify_bls_signature_share" => interact.verify_bls_signature_share().await,
        "verify_bls_aggregated_signature" => interact.verify_bls_aggregated_signature().await,
        "echo_u64" => interact.echo_u64().await,
        "echo_i64" => interact.echo_i64().await,
        "echo_i32" => interact.echo_i32().await,
        "echo_u32" => interact.echo_u32().await,
        "echo_isize" => interact.echo_isize().await,
        "echo_usize" => interact.echo_usize().await,
        "echo_i8" => interact.echo_i8().await,
        "echo_u8" => interact.echo_u8().await,
        "echo_bool" => interact.echo_bool().await,
        "echo_opt_bool" => interact.echo_opt_bool().await,
        "echo_nothing" => interact.echo_nothing().await,
        "echo_array_u8" => interact.echo_array_u8().await,
        "echo_multi_value_u32" => interact.echo_multi_value_u32().await,
        "echo_multi_value_tuples" => interact.echo_multi_value_tuples().await,
        "echo_ser_example_2" => interact.echo_ser_example_2().await,
        "echo_simple_enum" => interact.echo_simple_enum().await,
        "finish_simple_enum_variant_1" => interact.finish_simple_enum_variant_1().await,
        "echo_non_zero_usize" => interact.echo_non_zero_usize().await,
        "echo_some_args_ignore_others" => interact.echo_some_args_ignore_others().await,
        "echo_arrayvec" => interact.echo_arrayvec().await,
        "echo_big_uint" => interact.echo_big_uint().await,
        "echo_big_int" => interact.echo_big_int().await,
        "echo_non_zero_big_uint" => interact.echo_non_zero_big_uint().await,
        "echo_managed_buffer" => interact.echo_managed_buffer().await,
        "echo_managed_address" => interact.echo_managed_address().await,
        "echo_managed_option" => interact.echo_managed_option().await,
        "echo_big_int_managed_vec" => interact.echo_big_int_managed_vec().await,
        "echo_big_int_tuple" => interact.echo_big_int_tuple().await,
        "echo_big_int_option" => interact.echo_big_int_option().await,
        "echo_tuple_into_multiresult" => interact.echo_tuple_into_multiresult().await,
        "echo_managed_vec_of_managed_vec" => interact.echo_managed_vec_of_managed_vec().await,
        "echo_managed_vec_of_token_identifier" => interact.echo_managed_vec_of_token_identifier().await,
        "echo_managed_async_result_empty" => interact.echo_managed_async_result_empty().await,
        "echo_varags_managed_eager" => interact.echo_varags_managed_eager().await,
        "echo_varags_managed_sum" => interact.echo_varags_managed_sum().await,
        "echo_varags_vec_with_counted" => interact.echo_varags_vec_with_counted().await,
        "echo_varags_vec_with_counted_pairs" => interact.echo_varags_vec_with_counted_pairs().await,
        "convert_varags_vec_with_counted_pairs_1" => interact.convert_varags_vec_with_counted_pairs_1().await,
        "convert_varags_vec_with_counted_pairs_2" => interact.convert_varags_vec_with_counted_pairs_2().await,
        "compute_get_values" => interact.compute_get_values().await,
        "compute_create_ec" => interact.compute_create_ec().await,
        "compute_get_ec_length" => interact.compute_get_ec_length().await,
        "compute_get_priv_key_byte_length" => interact.compute_get_priv_key_byte_length().await,
        "compute_ec_add" => interact.compute_ec_add().await,
        "compute_ec_double" => interact.compute_ec_double().await,
        "compute_is_on_curve_ec" => interact.compute_is_on_curve_ec().await,
        "compute_scalar_mult" => interact.compute_scalar_mult().await,
        "compute_scalar_base_mult" => interact.compute_scalar_base_mult().await,
        "compute_marshal_ec" => interact.compute_marshal_ec().await,
        "compute_marshal_compressed_ec" => interact.compute_marshal_compressed_ec().await,
        "compute_unmarshal_ec" => interact.compute_unmarshal_ec().await,
        "compute_unmarshal_compressed_ec" => interact.compute_unmarshal_compressed_ec().await,
        "compute_generate_key_ec" => interact.compute_generate_key_ec().await,
        "logEventA" => interact.log_event_a().await,
        "logEventARepeat" => interact.log_event_a_repeat().await,
        "logEventB" => interact.log_event_b().await,
        "only_owner_endpoint" => interact.only_owner_endpoint().await,
        "only_user_account_endpoint" => interact.only_user_account_endpoint().await,
        "require_equals" => interact.require_equals().await,
        "sc_panic" => interact.sc_panic().await,
        "maddress_from_array" => interact.maddress_from_array().await,
        "maddress_from_managed_buffer" => interact.maddress_from_managed_buffer().await,
        "mbuffer_new" => interact.mbuffer_new().await,
        "mbuffer_concat" => interact.mbuffer_concat().await,
        "mbuffer_copy_slice" => interact.mbuffer_copy_slice().await,
        "mbuffer_set_random" => interact.mbuffer_set_random().await,
        "mbuffer_eq" => interact.mbuffer_eq().await,
        "managed_address_zero" => interact.managed_address_zero().await,
        "managed_address_eq" => interact.managed_address_eq().await,
        "managed_vec_new" => interact.managed_vec_new().await,
        "managed_vec_biguint_push" => interact.managed_vec_biguint_push().await,
        "managed_vec_biguint_eq" => interact.managed_vec_biguint_eq().await,
        "managed_vec_address_push" => interact.managed_vec_address_push().await,
        "managed_vec_set" => interact.managed_vec_set().await,
        "managed_vec_remove" => interact.managed_vec_remove().await,
        "managed_vec_find" => interact.managed_vec_find().await,
        "managed_vec_contains" => interact.managed_vec_contains().await,
        "managed_ref_explicit" => interact.managed_ref_explicit().await,
        "storage_read_raw" => interact.storage_read_raw().await,
        "storage_write_raw" => interact.storage_write_raw().await,
        "storage_read_from_address" => interact.storage_read_from_address().await,
        "load_bytes" => interact.load_bytes().await,
        "load_big_uint" => interact.load_big_uint().await,
        "load_big_int" => interact.load_big_int().await,
        "load_u64" => interact.load_u64().await,
        "load_usize" => interact.load_usize().await,
        "load_i64" => interact.load_i64().await,
        "load_bool" => interact.load_bool().await,
        "load_addr" => interact.load_addr().await,
        "load_opt_addr" => interact.load_opt_addr().await,
        "is_empty_opt_addr" => interact.is_empty_opt_addr().await,
        "get_nr_to_clear" => interact.get_nr_to_clear().await,
        "clear_storage_value" => interact.clear_storage_value().await,
        "load_ser_2" => interact.load_ser_2().await,
        "load_map1" => interact.load_map1().await,
        "load_map2" => interact.load_map2().await,
        "load_map3" => interact.load_map3().await,
        "load_from_address_raw" => interact.load_from_address_raw().await,
        "store_bytes" => interact.store_bytes().await,
        "store_big_uint" => interact.store_big_uint().await,
        "store_big_int" => interact.store_big_int().await,
        "store_usize" => interact.store_usize().await,
        "store_i32" => interact.store_i32().await,
        "store_u64" => interact.store_u64().await,
        "store_i64" => interact.store_i64().await,
        "store_bool" => interact.store_bool().await,
        "store_addr" => interact.store_addr().await,
        "store_opt_addr" => interact.store_opt_addr().await,
        "store_ser_2" => interact.store_ser_2().await,
        "store_map1" => interact.store_map1().await,
        "store_map2" => interact.store_map2().await,
        "store_map3" => interact.store_map3().await,
        "store_reserved_i64" => interact.store_reserved_i64().await,
        "store_reserved_big_uint" => interact.store_reserved_big_uint().await,
        "store_reserved_vec_u8" => interact.store_reserved_vec_u8().await,
        "token_has_transfer_role" => interact.token_has_transfer_role().await,
        "timelock_mapper" => interact.timelock_mapper().await,
        "timelock_set_initial_value" => interact.timelock_set_initial_value().await,
        "timelock_set_unlock_timestamp" => interact.timelock_set_unlock_timestamp().await,
        "timelock_commit_action" => interact.timelock_commit_action().await,
        "timelock_get_unlock_timestamp" => interact.timelock_get_unlock_timestamp().await,
        "timelock_get_future_value" => interact.timelock_get_future_value().await,
        "timelock_get_current_value_at_address" => interact.timelock_get_current_value_at_address().await,
        "timelock_get_unlock_timestamp_at_address" => interact.timelock_get_unlock_timestamp_at_address().await,
        "timelock_get_future_value_at_address" => interact.timelock_get_future_value_at_address().await,
        "address_to_id_mapper_get_id" => interact.address_to_id_mapper_get_id().await,
        "address_to_id_mapper_get_id_non_zero" => interact.address_to_id_mapper_get_id_non_zero().await,
        "address_to_id_mapper_get_address" => interact.address_to_id_mapper_get_address().await,
        "address_to_id_mapper_contains" => interact.address_to_id_mapper_contains().await,
        "address_to_id_mapper_set" => interact.address_to_id_mapper_set().await,
        "address_to_id_mapper_get_id_or_insert" => interact.address_to_id_mapper_get_id_or_insert().await,
        "address_to_id_mapper_remove_by_id" => interact.address_to_id_mapper_remove_by_id().await,
        "address_to_id_mapper_remove_by_address" => interact.address_to_id_mapper_remove_by_address().await,
        "getListMapper" => interact.list_mapper().await,
        "listMapperPushBack" => interact.list_mapper_push_back().await,
        "listMapperPushFront" => interact.list_mapper_push_front().await,
        "listMapperPopFront" => interact.list_mapper_pop_front().await,
        "listMapperPopBack" => interact.list_mapper_pop_back().await,
        "listMapperFront" => interact.list_mapper_front().await,
        "listMapperBack" => interact.list_mapper_back().await,
        "listMapperPushAfter" => interact.list_mapper_push_after().await,
        "listMapperPushBefore" => interact.list_mapper_push_before().await,
        "listMapperRemoveNode" => interact.list_mapper_remove_node().await,
        "listMapperRemoveNodeById" => interact.list_mapper_remove_node_by_id().await,
        "listMapperSetValue" => interact.list_mapper_set_value().await,
        "listMapperSetValueById" => interact.list_mapper_set_value_by_id().await,
        "listMapperIterateByHand" => interact.list_mapper_iterate_by_hand().await,
        "listMapperIterateByIter" => interact.list_mapper_iterate_by_iter().await,
        "queue_mapper" => interact.queue_mapper().await,
        "queue_mapper_push_back" => interact.queue_mapper_push_back().await,
        "queue_mapper_pop_front" => interact.queue_mapper_pop_front().await,
        "queue_mapper_front" => interact.queue_mapper_front().await,
        "map_mapper" => interact.map_mapper().await,
        "map_mapper_keys" => interact.map_mapper_keys().await,
        "map_mapper_values" => interact.map_mapper_values().await,
        "map_mapper_insert" => interact.map_mapper_insert().await,
        "map_mapper_contains_key" => interact.map_mapper_contains_key().await,
        "map_mapper_get" => interact.map_mapper_get().await,
        "map_mapper_remove" => interact.map_mapper_remove().await,
        "map_mapper_entry_or_default_update_increment" => interact.map_mapper_entry_or_default_update_increment().await,
        "map_mapper_entry_or_insert_default" => interact.map_mapper_entry_or_insert_default().await,
        "map_mapper_entry_and_modify" => interact.map_mapper_entry_and_modify().await,
        "map_mapper_entry_or_insert_with_key" => interact.map_mapper_entry_or_insert_with_key().await,
        "map_storage_mapper_view" => interact.map_storage_mapper_view().await,
        "map_storage_mapper_insert_default" => interact.map_storage_mapper_insert_default().await,
        "map_storage_mapper_contains_key" => interact.map_storage_mapper_contains_key().await,
        "map_storage_mapper_get" => interact.map_storage_mapper_get().await,
        "map_storage_mapper_insert_value" => interact.map_storage_mapper_insert_value().await,
        "map_storage_mapper_get_value" => interact.map_storage_mapper_get_value().await,
        "map_storage_mapper_remove" => interact.map_storage_mapper_remove().await,
        "map_storage_mapper_clear" => interact.map_storage_mapper_clear().await,
        "map_storage_mapper_entry_or_default_update_increment" => interact.map_storage_mapper_entry_or_default_update_increment().await,
        "map_storage_mapper_entry_and_modify_increment_or_default" => interact.map_storage_mapper_entry_and_modify_increment_or_default().await,
        "map_storage_mapper_entry_or_default_update" => interact.map_storage_mapper_entry_or_default_update().await,
        "set_mapper" => interact.set_mapper().await,
        "set_mapper_insert" => interact.set_mapper_insert().await,
        "set_mapper_contains" => interact.set_mapper_contains().await,
        "set_mapper_remove" => interact.set_mapper_remove().await,
        "set_mapper_front" => interact.set_mapper_front().await,
        "set_mapper_back" => interact.set_mapper_back().await,
        "set_mapper_next" => interact.set_mapper_next().await,
        "set_mapper_previous" => interact.set_mapper_previous().await,
        "set_mapper_iter_from_and_count" => interact.set_mapper_iter_from_and_count().await,
        "map_my_single_value_mapper" => interact.map_my_single_value_mapper().await,
        "my_single_value_mapper_increment_1" => interact.my_single_value_mapper_increment_1().await,
        "my_single_value_mapper_increment_2" => interact.my_single_value_mapper_increment_2().await,
        "my_single_value_mapper_subtract_with_require" => interact.my_single_value_mapper_subtract_with_require().await,
        "my_single_value_mapper_set_if_empty" => interact.my_single_value_mapper_set_if_empty().await,
        "clear_single_value_mapper" => interact.clear_single_value_mapper().await,
        "get_from_address_single_value_mapper" => interact.get_from_address_single_value_mapper().await,
        "is_empty_single_value_mapper" => interact.is_empty_single_value_mapper().await,
        "is_empty_at_address_single_value_mapper" => interact.is_empty_at_address_single_value_mapper().await,
        "raw_byte_length_single_value_mapper" => interact.raw_byte_length_single_value_mapper().await,
        "set_single_value_mapper_with_key" => interact.set_single_value_mapper_with_key().await,
        "vec_mapper" => interact.vec_mapper().await,
        "vec_mapper_push" => interact.vec_mapper_push().await,
        "vec_mapper_get" => interact.vec_mapper_get().await,
        "vec_mapper_get_at_address" => interact.vec_mapper_get_at_address().await,
        "vec_mapper_len" => interact.vec_mapper_len().await,
        "vec_mapper_len_at_address" => interact.vec_mapper_len_at_address().await,
        "token_attributes_set" => interact.token_attributes_set().await,
        "token_attributes_update" => interact.token_attributes_update().await,
        "token_attributes_get_attributes" => interact.token_attributes_get_attributes().await,
        "token_attributes_get_nonce" => interact.token_attributes_get_nonce().await,
        "token_attributes_clear" => interact.token_attributes_clear().await,
        "token_attributes_has_attributes" => interact.token_attributes_has_attributes().await,
        "add_to_whitelist" => interact.add_to_whitelist().await,
        "remove_from_whitelist" => interact.remove_from_whitelist().await,
        "check_contains" => interact.check_contains().await,
        "check_contains_at_address" => interact.check_contains_at_address().await,
        "require_contains" => interact.require_contains().await,
        "require_contains_at_address" => interact.require_contains_at_address().await,
        "issue_fungible_default_callback" => interact.issue_fungible_default_callback().await,
        "issue_fungible_custom_callback" => interact.issue_fungible_custom_callback().await,
        "issue_and_set_all_roles_fungible" => interact.issue_and_set_all_roles_fungible().await,
        "set_local_roles_fungible" => interact.set_local_roles_fungible().await,
        "mint_fungible" => interact.mint_fungible().await,
        "mint_and_send_fungible" => interact.mint_and_send_fungible().await,
        "burn_fungible" => interact.burn_fungible().await,
        "get_balance_fungible" => interact.get_balance_fungible().await,
        "require_same_token_fungible" => interact.require_same_token_fungible().await,
        "require_all_same_token_fungible" => interact.require_all_same_token_fungible().await,
        "getFungibleTokenId" => interact.fungible_token_mapper().await,
        "issue_and_set_all_roles_meta" => interact.issue_and_set_all_roles_meta().await,
        "mapper_nft_set_token_id" => interact.mapper_nft_set_token_id().await,
        "mapper_nft_create" => interact.mapper_nft_create().await,
        "mapper_nft_create_and_send" => interact.mapper_nft_create_and_send().await,
        "mapper_nft_add_quantity" => interact.mapper_nft_add_quantity().await,
        "mapper_nft_add_quantity_and_send" => interact.mapper_nft_add_quantity_and_send().await,
        "mapper_nft_burn" => interact.mapper_nft_burn().await,
        "mapper_nft_get_balance" => interact.mapper_nft_get_balance().await,
        "mapper_get_token_attributes" => interact.mapper_get_token_attributes().await,
        "getNonFungibleTokenId" => interact.non_fungible_token_mapper().await,
        "init_unique_id_mapper" => interact.init_unique_id_mapper().await,
        "unique_id_mapper_get" => interact.unique_id_mapper_get().await,
        "unique_id_mapper_swap_remove" => interact.unique_id_mapper_swap_remove().await,
        "unique_id_mapper_set" => interact.unique_id_mapper_set().await,
        "unique_id_mapper" => interact.unique_id_mapper().await,
        "unordered_set_mapper" => interact.unordered_set_mapper().await,
        "unordered_set_mapper_insert" => interact.unordered_set_mapper_insert().await,
        "unordered_set_mapper_contains" => interact.unordered_set_mapper_contains().await,
        "unordered_set_mapper_remove" => interact.unordered_set_mapper_remove().await,
        "managed_struct_eq" => interact.managed_struct_eq().await,
        "no_overflow_usize" => interact.no_overflow_usize().await,
        "no_overflow_u8" => interact.no_overflow_u8().await,
        "no_overflow_u16" => interact.no_overflow_u16().await,
        "no_overflow_u32" => interact.no_overflow_u32().await,
        "no_overflow_u64" => interact.no_overflow_u64().await,
        "overflow_usize" => interact.overflow_usize().await,
        "overflow_u8" => interact.overflow_u8().await,
        "overflow_u16" => interact.overflow_u16().await,
        "overflow_u32" => interact.overflow_u32().await,
        "overflow_u64" => interact.overflow_u64().await,
        "no_overflow_isize" => interact.no_overflow_isize().await,
        "no_overflow_i8" => interact.no_overflow_i8().await,
        "no_overflow_i16" => interact.no_overflow_i16().await,
        "no_overflow_i32" => interact.no_overflow_i32().await,
        "no_overflow_i64" => interact.no_overflow_i64().await,
        "overflow_isize" => interact.overflow_isize().await,
        "overflow_i8" => interact.overflow_i8().await,
        "overflow_i16" => interact.overflow_i16().await,
        "overflow_i32" => interact.overflow_i32().await,
        "overflow_i64" => interact.overflow_i64().await,
        "token_identifier_egld" => interact.token_identifier_egld().await,
        "token_identifier_is_valid_1" => interact.token_identifier_is_valid_1().await,
        "token_identifier_is_valid_2" => interact.token_identifier_is_valid_2().await,
        "non_zero_usize_iter" => interact.non_zero_usize_iter().await,
        "non_zero_usize_macro" => interact.non_zero_usize_macro().await,
        "returns_egld_decimal" => interact.returns_egld_decimal().await,
        "set_contract_address" => interact.set_contract_address().await,
        "is_empty_at_address" => interact.is_empty_at_address().await,
        "contains_at_address" => interact.contains_at_address().await,
        "len_at_address" => interact.len_at_address().await,
        "next_at_address" => interact.next_at_address().await,
        "previous_at_address" => interact.previous_at_address().await,
        "front_at_address" => interact.front_at_address().await,
        "back_at_address" => interact.back_at_address().await,
        "keys_at_address" => interact.keys_at_address().await,
        "values_at_address" => interact.values_at_address().await,
        "contains_unordered_at_address" => interact.contains_unordered_at_address().await,
        "get_by_index" => interact.get_by_index().await,
        "fill_set_mapper" => interact.fill_set_mapper().await,
        "fill_map_mapper" => interact.fill_map_mapper().await,
        "fill_unordered_set_mapper" => interact.fill_unordered_set_mapper().await,
        "get_value_from_address_with_keys" => interact.get_value_from_address_with_keys().await,
        "address_to_id_mapper_get_id_from_address" => interact.address_to_id_mapper_get_id_from_address().await,
        "managed_decimal_addition" => interact.managed_decimal_addition().await,
        "managed_decimal_subtraction" => interact.managed_decimal_subtraction().await,
        "managed_decimal_eq" => interact.managed_decimal_eq().await,
        "managed_decimal_trunc" => interact.managed_decimal_trunc().await,
        "managed_decimal_into_raw_units" => interact.managed_decimal_into_raw_units().await,
        "managed_decimal_ln" => interact.managed_decimal_ln().await,
        "managed_decimal_log2" => interact.managed_decimal_log2().await,
        "managed_decimal_addition_var" => interact.managed_decimal_addition_var().await,
        "managed_decimal_subtraction_var" => interact.managed_decimal_subtraction_var().await,
        "managed_decimal_eq_var" => interact.managed_decimal_eq_var().await,
        "managed_decimal_ln_var" => interact.managed_decimal_ln_var().await,
        "managed_decimal_log2_var" => interact.managed_decimal_log2_var().await,
        "mm_get" => interact.mm_get().await,
        "mm_contains" => interact.mm_contains().await,
        "mm_remove_get" => interact.mm_remove_get().await,
        "mm_mutable_input_test" => interact.mm_mutable_input_test().await,
        _ => panic!("unknown command: {}", &cmd),
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    contract_address: Option<Bech32Address>
}

impl State {
        // Deserializes state from file
        pub fn load_state() -> Self {
            if Path::new(STATE_FILE).exists() {
                let mut file = std::fs::File::open(STATE_FILE).unwrap();
                let mut content = String::new();
                file.read_to_string(&mut content).unwrap();
                toml::from_str(&content).unwrap()
            } else {
                Self::default()
            }
        }
    
        /// Sets the contract address
        pub fn set_address(&mut self, address: Bech32Address) {
            self.contract_address = Some(address);
        }
    
        /// Returns the contract address
        pub fn current_address(&self) -> &Bech32Address {
            self.contract_address
                .as_ref()
                .expect("no known contract, deploy first")
        }
    }
    
    impl Drop for State {
        // Serializes state to file
        fn drop(&mut self) {
            let mut file = std::fs::File::create(STATE_FILE).unwrap();
            file.write_all(toml::to_string(self).unwrap().as_bytes())
                .unwrap();
        }
    }

pub struct ContractInteract {
    interactor: Interactor,
    wallet_address: Address,
    contract_code: BytesValue,
    state: State
}

impl ContractInteract {
    pub async fn new(config: Config) -> Self {
        let mut interactor = Interactor::new(config.gateway_uri())
            .await
            .use_chain_simulator(config.use_chain_simulator());

        interactor.set_current_dir_from_workspace("basic-features");
        let wallet_address = interactor.register_wallet(test_wallets::alice()).await;

        // Useful in the chain simulator setting
        // generate blocks until ESDTSystemSCAddress is enabled
        interactor.generate_blocks_until_all_activations().await;
        
        let contract_code = BytesValue::interpret_from(
            "mxsc:../output/basic-features.mxsc.json",
            &InterpreterContext::default(),
        );

        ContractInteract {
            interactor,
            wallet_address,
            contract_code,
            state: State::load_state()
        }
    }

    pub async fn deploy(&mut self) {
        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .init()
            .code(&self.contract_code)
            .returns(ReturnsNewAddress)
            .run()
            .await;
        let new_address_bech32 = new_address.to_bech32_default();
        println!("new address: {new_address_bech32}");
        self.state.set_address(new_address_bech32);
    }

    pub async fn panic_with_message(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .panic_with_message()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn count_ones(&mut self) {
        let arg = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .count_ones(arg)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn endpoint_with_mutable_arg(&mut self) {
        let arg1 = BigUint::<StaticApi>::from(0u128);
        let arg2 = 0u64;
        let arg3 = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .endpoint_with_mutable_arg(arg1, arg2, arg3)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn get_esdt_token_data(&mut self) {
        let address = ManagedAddress::<StaticApi>::zero();
        let token_id = TokenIdentifier::<StaticApi>::default();
        let nonce = 0u64;

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .get_esdt_token_data(address, token_id, nonce)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn sqrt_big_uint(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .sqrt_big_uint(a)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn sqrt_big_uint_ref(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .sqrt_big_uint_ref(a)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn log2_big_uint(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .log2_big_uint(a)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn log2_big_uint_ref(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .log2_big_uint_ref(a)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn pow_big_int(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .pow_big_int(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn pow_big_int_ref(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .pow_big_int_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn pow_big_uint(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .pow_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn pow_big_uint_ref(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .pow_big_uint_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn big_uint_to_u64(&mut self) {
        let bu = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .big_uint_to_u64(bu)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn biguint_overwrite_u64(&mut self) {
        let bu = BigUint::<StaticApi>::from(0u128);
        let small = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .biguint_overwrite_u64(bu, small)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn big_uint_zero(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .big_uint_zero()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn big_uint_from_u64_1(&mut self) {
        let small = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .big_uint_from_u64_1(small)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn big_uint_from_u64_2(&mut self) {
        let small = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .big_uint_from_u64_2(small)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn biguint_from_u128(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .biguint_from_u128()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn big_uint_from_managed_buffer(&mut self) {
        let mb = ManagedBuffer::new_from_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .big_uint_from_managed_buffer(mb)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn big_uint_from_managed_buffer_ref(&mut self) {
        let mb = ManagedBuffer::new_from_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .big_uint_from_managed_buffer_ref(mb)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn big_int_zero(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .big_int_zero()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn big_int_from_i64_1(&mut self) {
        let small = i64::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .big_int_from_i64_1(small)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn big_int_from_i64_2(&mut self) {
        let small = i64::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .big_int_from_i64_2(small)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn big_uint_eq_u64(&mut self) {
        let bi = BigUint::<StaticApi>::from(0u128);
        let small = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .big_uint_eq_u64(bi, small)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn big_int_to_i64(&mut self) {
        let bi = BigInt::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .big_int_to_i64(bi)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn bigint_overwrite_i64(&mut self) {
        let bi = BigInt::<StaticApi>::default();
        let small = i64::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .bigint_overwrite_i64(bi, small)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn big_int_to_parts(&mut self) {
        let bi = BigInt::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .big_int_to_parts(bi)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn big_int_from_biguint(&mut self) {
        let sign = Sign::<StaticApi>::default();
        let unsigned = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .big_int_from_biguint(sign, unsigned)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn add_big_int_big_uint(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .add_big_int_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn add_big_uint_big_int(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigInt::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .add_big_uint_big_int(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn add_big_int_big_uint_ref(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .add_big_int_big_uint_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn add_big_uint_big_int_ref(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigInt::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .add_big_uint_big_int_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn add_big_int_big_int(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = BigInt::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .add_big_int_big_int(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn add_big_int_big_int_ref(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = BigInt::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .add_big_int_big_int_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn add_big_int_ref_big_int(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = BigInt::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .add_big_int_ref_big_int(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn add_big_int_ref_big_int_ref(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = BigInt::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .add_big_int_ref_big_int_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn add_big_uint_big_uint(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .add_big_uint_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn add_big_uint_big_uint_ref(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .add_big_uint_big_uint_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn add_big_uint_ref_big_uint(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .add_big_uint_ref_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn add_big_uint_ref_big_uint_ref(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .add_big_uint_ref_big_uint_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn add_big_uint_u32(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .add_big_uint_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn add_big_uint_ref_u32(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .add_big_uint_ref_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn add_big_uint_u64(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .add_big_uint_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn add_big_uint_ref_u64(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .add_big_uint_ref_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn add_non_zero_big_uint_non_zero_big_uint(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = NonZeroBigUint::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .add_non_zero_big_uint_non_zero_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn add_non_zero_big_uint_non_zero_big_uint_ref(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = NonZeroBigUint::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .add_non_zero_big_uint_non_zero_big_uint_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn add_non_zero_big_uint_ref_non_zero_big_uint(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = NonZeroBigUint::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .add_non_zero_big_uint_ref_non_zero_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn add_non_zero_big_uint_ref_non_zero_big_uint_ref(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = NonZeroBigUint::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .add_non_zero_big_uint_ref_non_zero_big_uint_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn add_non_zero_big_uint_u32(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .add_non_zero_big_uint_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn add_non_zero_big_uint_ref_u32(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .add_non_zero_big_uint_ref_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn add_non_zero_big_uint_u64(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .add_non_zero_big_uint_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn add_non_zero_big_uint_ref_u64(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .add_non_zero_big_uint_ref_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn sub_big_int_big_int(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = BigInt::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .sub_big_int_big_int(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn sub_big_int_big_int_ref(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = BigInt::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .sub_big_int_big_int_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn sub_big_int_ref_big_int(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = BigInt::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .sub_big_int_ref_big_int(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn sub_big_int_ref_big_int_ref(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = BigInt::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .sub_big_int_ref_big_int_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn sub_big_uint_big_uint(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .sub_big_uint_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn sub_big_uint_big_uint_ref(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .sub_big_uint_big_uint_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn sub_big_uint_ref_big_uint(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .sub_big_uint_ref_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn sub_big_uint_ref_big_uint_ref(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .sub_big_uint_ref_big_uint_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn sub_big_uint_u32(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .sub_big_uint_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn sub_big_uint_ref_u32(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .sub_big_uint_ref_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn sub_big_uint_u64(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .sub_big_uint_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn sub_big_uint_ref_u64(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .sub_big_uint_ref_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn sub_non_zero_big_uint_non_zero_big_uint(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = NonZeroBigUint::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .sub_non_zero_big_uint_non_zero_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn sub_non_zero_big_uint_non_zero_big_uint_ref(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = NonZeroBigUint::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .sub_non_zero_big_uint_non_zero_big_uint_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn sub_non_zero_big_uint_ref_non_zero_big_uint(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = NonZeroBigUint::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .sub_non_zero_big_uint_ref_non_zero_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn sub_non_zero_big_uint_ref_non_zero_big_uint_ref(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = NonZeroBigUint::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .sub_non_zero_big_uint_ref_non_zero_big_uint_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn sub_non_zero_big_uint_u32(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .sub_non_zero_big_uint_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn sub_non_zero_big_uint_ref_u32(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .sub_non_zero_big_uint_ref_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn sub_non_zero_big_uint_u64(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .sub_non_zero_big_uint_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn sub_non_zero_big_uint_ref_u64(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .sub_non_zero_big_uint_ref_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mul_big_int_big_int(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = BigInt::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mul_big_int_big_int(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mul_big_int_big_int_ref(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = BigInt::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mul_big_int_big_int_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mul_big_int_ref_big_int(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = BigInt::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mul_big_int_ref_big_int(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mul_big_int_ref_big_int_ref(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = BigInt::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mul_big_int_ref_big_int_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mul_big_uint_big_uint(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mul_big_uint_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mul_big_uint_big_uint_ref(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mul_big_uint_big_uint_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mul_big_uint_ref_big_uint(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mul_big_uint_ref_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mul_big_uint_ref_big_uint_ref(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mul_big_uint_ref_big_uint_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mul_big_uint_u32(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mul_big_uint_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mul_big_uint_ref_u32(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mul_big_uint_ref_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mul_big_uint_u64(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mul_big_uint_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mul_big_uint_ref_u64(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mul_big_uint_ref_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mul_non_zero_big_uint_non_zero_big_uint(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = NonZeroBigUint::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mul_non_zero_big_uint_non_zero_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mul_non_zero_big_uint_non_zero_big_uint_ref(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = NonZeroBigUint::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mul_non_zero_big_uint_non_zero_big_uint_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mul_non_zero_big_uint_ref_non_zero_big_uint(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = NonZeroBigUint::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mul_non_zero_big_uint_ref_non_zero_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mul_non_zero_big_uint_ref_non_zero_big_uint_ref(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = NonZeroBigUint::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mul_non_zero_big_uint_ref_non_zero_big_uint_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mul_non_zero_big_uint_u32(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mul_non_zero_big_uint_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mul_non_zero_big_uint_ref_u32(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mul_non_zero_big_uint_ref_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mul_non_zero_big_uint_u64(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mul_non_zero_big_uint_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mul_non_zero_big_uint_ref_u64(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mul_non_zero_big_uint_ref_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn div_big_int_big_int(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = BigInt::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .div_big_int_big_int(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn div_big_int_big_int_ref(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = BigInt::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .div_big_int_big_int_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn div_big_int_ref_big_int(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = BigInt::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .div_big_int_ref_big_int(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn div_big_int_ref_big_int_ref(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = BigInt::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .div_big_int_ref_big_int_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn div_big_uint_big_uint(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .div_big_uint_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn div_big_uint_big_uint_ref(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .div_big_uint_big_uint_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn div_big_uint_ref_big_uint(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .div_big_uint_ref_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn div_big_uint_ref_big_uint_ref(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .div_big_uint_ref_big_uint_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn div_big_uint_u32(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .div_big_uint_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn div_big_uint_ref_u32(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .div_big_uint_ref_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn div_big_uint_u64(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .div_big_uint_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn div_big_uint_ref_u64(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .div_big_uint_ref_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn div_non_zero_big_uint_non_zero_big_uint(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = NonZeroBigUint::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .div_non_zero_big_uint_non_zero_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn div_non_zero_big_uint_non_zero_big_uint_ref(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = NonZeroBigUint::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .div_non_zero_big_uint_non_zero_big_uint_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn div_non_zero_big_uint_ref_non_zero_big_uint(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = NonZeroBigUint::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .div_non_zero_big_uint_ref_non_zero_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn div_non_zero_big_uint_ref_non_zero_big_uint_ref(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = NonZeroBigUint::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .div_non_zero_big_uint_ref_non_zero_big_uint_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn div_non_zero_big_uint_u32(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .div_non_zero_big_uint_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn div_non_zero_big_uint_ref_u32(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .div_non_zero_big_uint_ref_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn div_non_zero_big_uint_u64(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .div_non_zero_big_uint_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn div_non_zero_big_uint_ref_u64(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .div_non_zero_big_uint_ref_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn rem_big_int_big_int(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = BigInt::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .rem_big_int_big_int(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn rem_big_int_big_int_ref(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = BigInt::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .rem_big_int_big_int_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn rem_big_int_ref_big_int(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = BigInt::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .rem_big_int_ref_big_int(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn rem_big_int_ref_big_int_ref(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = BigInt::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .rem_big_int_ref_big_int_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn rem_big_uint_big_uint(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .rem_big_uint_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn rem_big_uint_big_uint_ref(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .rem_big_uint_big_uint_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn rem_big_uint_ref_big_uint(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .rem_big_uint_ref_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn rem_big_uint_ref_big_uint_ref(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .rem_big_uint_ref_big_uint_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn rem_big_uint_u32(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .rem_big_uint_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn rem_big_uint_ref_u32(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .rem_big_uint_ref_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn rem_big_uint_u64(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .rem_big_uint_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn rem_big_uint_ref_u64(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .rem_big_uint_ref_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn rem_non_zero_big_uint_non_zero_big_uint(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = NonZeroBigUint::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .rem_non_zero_big_uint_non_zero_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn rem_non_zero_big_uint_non_zero_big_uint_ref(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = NonZeroBigUint::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .rem_non_zero_big_uint_non_zero_big_uint_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn rem_non_zero_big_uint_ref_non_zero_big_uint(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = NonZeroBigUint::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .rem_non_zero_big_uint_ref_non_zero_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn rem_non_zero_big_uint_ref_non_zero_big_uint_ref(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = NonZeroBigUint::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .rem_non_zero_big_uint_ref_non_zero_big_uint_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn rem_non_zero_big_uint_u32(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .rem_non_zero_big_uint_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn rem_non_zero_big_uint_ref_u32(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .rem_non_zero_big_uint_ref_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn rem_non_zero_big_uint_u64(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .rem_non_zero_big_uint_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn rem_non_zero_big_uint_ref_u64(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .rem_non_zero_big_uint_ref_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn add_assign_big_int_big_int(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = BigInt::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .add_assign_big_int_big_int(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn add_assign_big_int_big_int_ref(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = BigInt::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .add_assign_big_int_big_int_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn add_assign_big_uint_big_uint(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .add_assign_big_uint_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn add_assign_big_uint_big_uint_ref(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .add_assign_big_uint_big_uint_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn add_assign_big_uint_u32(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .add_assign_big_uint_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn add_assign_big_uint_u64(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .add_assign_big_uint_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn add_assign_non_zero_big_uint_non_zero_big_uint(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = NonZeroBigUint::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .add_assign_non_zero_big_uint_non_zero_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn add_assign_non_zero_big_uint_non_zero_big_uint_ref(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = NonZeroBigUint::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .add_assign_non_zero_big_uint_non_zero_big_uint_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn add_assign_non_zero_big_uint_big_uint(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .add_assign_non_zero_big_uint_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn add_assign_non_zero_big_uint_big_uint_ref(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .add_assign_non_zero_big_uint_big_uint_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn add_assign_non_zero_big_uint_u32(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .add_assign_non_zero_big_uint_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn add_assign_non_zero_big_uint_u64(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .add_assign_non_zero_big_uint_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn sub_assign_big_int_big_int(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = BigInt::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .sub_assign_big_int_big_int(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn sub_assign_big_int_big_int_ref(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = BigInt::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .sub_assign_big_int_big_int_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn sub_assign_big_uint_big_uint(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .sub_assign_big_uint_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn sub_assign_big_uint_big_uint_ref(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .sub_assign_big_uint_big_uint_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn sub_assign_big_uint_u32(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .sub_assign_big_uint_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn sub_assign_big_uint_u64(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .sub_assign_big_uint_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn sub_assign_non_zero_big_uint_non_zero_big_uint(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = NonZeroBigUint::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .sub_assign_non_zero_big_uint_non_zero_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn sub_assign_non_zero_big_uint_non_zero_big_uint_ref(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = NonZeroBigUint::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .sub_assign_non_zero_big_uint_non_zero_big_uint_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn sub_assign_non_zero_big_uint_big_uint(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .sub_assign_non_zero_big_uint_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn sub_assign_non_zero_big_uint_big_uint_ref(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .sub_assign_non_zero_big_uint_big_uint_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn sub_assign_non_zero_big_uint_u32(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .sub_assign_non_zero_big_uint_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn sub_assign_non_zero_big_uint_u64(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .sub_assign_non_zero_big_uint_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mul_assign_big_int_big_int(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = BigInt::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mul_assign_big_int_big_int(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mul_assign_big_int_big_int_ref(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = BigInt::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mul_assign_big_int_big_int_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mul_assign_big_uint_big_uint(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mul_assign_big_uint_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mul_assign_big_uint_big_uint_ref(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mul_assign_big_uint_big_uint_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mul_assign_big_uint_u32(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mul_assign_big_uint_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mul_assign_big_uint_u64(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mul_assign_big_uint_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mul_assign_non_zero_big_uint_non_zero_big_uint(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = NonZeroBigUint::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mul_assign_non_zero_big_uint_non_zero_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mul_assign_non_zero_big_uint_non_zero_big_uint_ref(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = NonZeroBigUint::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mul_assign_non_zero_big_uint_non_zero_big_uint_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mul_assign_non_zero_big_uint_big_uint(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mul_assign_non_zero_big_uint_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mul_assign_non_zero_big_uint_big_uint_ref(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mul_assign_non_zero_big_uint_big_uint_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mul_assign_non_zero_big_uint_u32(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mul_assign_non_zero_big_uint_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mul_assign_non_zero_big_uint_u64(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mul_assign_non_zero_big_uint_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn div_assign_big_int_big_int(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = BigInt::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .div_assign_big_int_big_int(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn div_assign_big_int_big_int_ref(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = BigInt::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .div_assign_big_int_big_int_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn div_assign_big_uint_big_uint(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .div_assign_big_uint_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn div_assign_big_uint_big_uint_ref(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .div_assign_big_uint_big_uint_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn div_assign_big_uint_u32(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .div_assign_big_uint_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn div_assign_big_uint_u64(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .div_assign_big_uint_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn div_assign_non_zero_big_uint_non_zero_big_uint(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = NonZeroBigUint::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .div_assign_non_zero_big_uint_non_zero_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn div_assign_non_zero_big_uint_non_zero_big_uint_ref(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = NonZeroBigUint::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .div_assign_non_zero_big_uint_non_zero_big_uint_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn div_assign_non_zero_big_uint_big_uint(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .div_assign_non_zero_big_uint_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn div_assign_non_zero_big_uint_big_uint_ref(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .div_assign_non_zero_big_uint_big_uint_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn div_assign_non_zero_big_uint_u32(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .div_assign_non_zero_big_uint_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn div_assign_non_zero_big_uint_u64(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .div_assign_non_zero_big_uint_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn rem_assign_big_int_big_int(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = BigInt::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .rem_assign_big_int_big_int(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn rem_assign_big_int_big_int_ref(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = BigInt::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .rem_assign_big_int_big_int_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn rem_assign_big_uint_big_uint(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .rem_assign_big_uint_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn rem_assign_big_uint_big_uint_ref(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .rem_assign_big_uint_big_uint_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn rem_assign_big_uint_u32(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .rem_assign_big_uint_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn rem_assign_big_uint_u64(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .rem_assign_big_uint_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn rem_assign_non_zero_big_uint_non_zero_big_uint(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = NonZeroBigUint::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .rem_assign_non_zero_big_uint_non_zero_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn rem_assign_non_zero_big_uint_non_zero_big_uint_ref(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = NonZeroBigUint::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .rem_assign_non_zero_big_uint_non_zero_big_uint_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn rem_assign_non_zero_big_uint_big_uint(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .rem_assign_non_zero_big_uint_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn rem_assign_non_zero_big_uint_big_uint_ref(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .rem_assign_non_zero_big_uint_big_uint_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn rem_assign_non_zero_big_uint_u32(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .rem_assign_non_zero_big_uint_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn rem_assign_non_zero_big_uint_u64(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .rem_assign_non_zero_big_uint_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn bit_and_big_uint_big_uint(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .bit_and_big_uint_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn bit_and_big_uint_big_uint_ref(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .bit_and_big_uint_big_uint_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn bit_and_big_uint_ref_big_uint(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .bit_and_big_uint_ref_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn bit_and_big_uint_ref_big_uint_ref(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .bit_and_big_uint_ref_big_uint_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn bit_and_big_uint_u32(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .bit_and_big_uint_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn bit_and_big_uint_ref_u32(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .bit_and_big_uint_ref_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn bit_and_big_uint_u64(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .bit_and_big_uint_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn bit_and_big_uint_ref_u64(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .bit_and_big_uint_ref_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn bit_or_big_uint_big_uint(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .bit_or_big_uint_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn bit_or_big_uint_big_uint_ref(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .bit_or_big_uint_big_uint_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn bit_or_big_uint_ref_big_uint(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .bit_or_big_uint_ref_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn bit_or_big_uint_ref_big_uint_ref(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .bit_or_big_uint_ref_big_uint_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn bit_or_big_uint_u32(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .bit_or_big_uint_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn bit_or_big_uint_ref_u32(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .bit_or_big_uint_ref_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn bit_or_big_uint_u64(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .bit_or_big_uint_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn bit_or_big_uint_ref_u64(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .bit_or_big_uint_ref_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn bit_xor_big_uint_big_uint(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .bit_xor_big_uint_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn bit_xor_big_uint_big_uint_ref(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .bit_xor_big_uint_big_uint_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn bit_xor_big_uint_ref_big_uint(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .bit_xor_big_uint_ref_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn bit_xor_big_uint_ref_big_uint_ref(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .bit_xor_big_uint_ref_big_uint_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn bit_xor_big_uint_u32(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .bit_xor_big_uint_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn bit_xor_big_uint_ref_u32(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .bit_xor_big_uint_ref_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn bit_xor_big_uint_u64(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .bit_xor_big_uint_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn bit_xor_big_uint_ref_u64(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .bit_xor_big_uint_ref_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn bit_and_assign_big_uint_big_uint(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .bit_and_assign_big_uint_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn bit_and_assign_big_uint_big_uint_ref(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .bit_and_assign_big_uint_big_uint_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn bit_and_assign_big_uint_u32(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .bit_and_assign_big_uint_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn bit_and_assign_big_uint_u64(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .bit_and_assign_big_uint_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn bit_or_assign_big_uint_big_uint(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .bit_or_assign_big_uint_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn bit_or_assign_big_uint_big_uint_ref(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .bit_or_assign_big_uint_big_uint_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn bit_or_assign_big_uint_u32(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .bit_or_assign_big_uint_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn bit_or_assign_big_uint_u64(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .bit_or_assign_big_uint_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn bit_xor_assign_big_uint_big_uint(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .bit_xor_assign_big_uint_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn bit_xor_assign_big_uint_big_uint_ref(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .bit_xor_assign_big_uint_big_uint_ref(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn bit_xor_assign_big_uint_u32(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .bit_xor_assign_big_uint_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn bit_xor_assign_big_uint_u64(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .bit_xor_assign_big_uint_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn shr_big_uint_usize(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .shr_big_uint_usize(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn shr_big_uint_ref_usize(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .shr_big_uint_ref_usize(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn shl_big_uint_usize(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .shl_big_uint_usize(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn shl_big_uint_ref_usize(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .shl_big_uint_ref_usize(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn shr_assign_big_uint_usize(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .shr_assign_big_uint_usize(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn shl_assign_big_uint_usize(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .shl_assign_big_uint_usize(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn eq_big_int_big_int(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = BigInt::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .eq_big_int_big_int(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn eq_big_int_i32(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = i32::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .eq_big_int_i32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn eq_big_int_i64(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = i64::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .eq_big_int_i64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn eq_big_int_u32(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .eq_big_int_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn eq_big_int_u64(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .eq_big_int_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn eq_big_uint_big_uint(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .eq_big_uint_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn eq_big_uint_i32(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = i32::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .eq_big_uint_i32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn eq_big_uint_i64(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = i64::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .eq_big_uint_i64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn eq_big_uint_u32(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .eq_big_uint_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn eq_big_uint_u64(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .eq_big_uint_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn eq_non_zero_big_uint_non_zero_big_uint(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = NonZeroBigUint::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .eq_non_zero_big_uint_non_zero_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn eq_non_zero_big_uint_big_uint(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .eq_non_zero_big_uint_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn eq_non_zero_big_uint_i32(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = i32::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .eq_non_zero_big_uint_i32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn eq_non_zero_big_uint_i64(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = i64::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .eq_non_zero_big_uint_i64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn eq_non_zero_big_uint_u32(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .eq_non_zero_big_uint_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn eq_non_zero_big_uint_u64(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .eq_non_zero_big_uint_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn gt_big_int_big_int(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = BigInt::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .gt_big_int_big_int(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn gt_big_int_i32(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = i32::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .gt_big_int_i32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn gt_big_int_i64(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = i64::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .gt_big_int_i64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn gt_big_int_u32(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .gt_big_int_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn gt_big_int_u64(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .gt_big_int_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn gt_big_uint_big_uint(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .gt_big_uint_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn gt_big_uint_i32(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = i32::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .gt_big_uint_i32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn gt_big_uint_i64(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = i64::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .gt_big_uint_i64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn gt_big_uint_u32(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .gt_big_uint_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn gt_big_uint_u64(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .gt_big_uint_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn gt_non_zero_big_uint_non_zero_big_uint(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = NonZeroBigUint::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .gt_non_zero_big_uint_non_zero_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn gt_non_zero_big_uint_big_uint(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .gt_non_zero_big_uint_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn gt_non_zero_big_uint_i32(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = i32::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .gt_non_zero_big_uint_i32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn gt_non_zero_big_uint_i64(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = i64::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .gt_non_zero_big_uint_i64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn gt_non_zero_big_uint_u32(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .gt_non_zero_big_uint_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn gt_non_zero_big_uint_u64(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .gt_non_zero_big_uint_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn ge_big_int_big_int(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = BigInt::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .ge_big_int_big_int(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn ge_big_int_i32(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = i32::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .ge_big_int_i32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn ge_big_int_i64(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = i64::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .ge_big_int_i64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn ge_big_int_u32(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .ge_big_int_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn ge_big_int_u64(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .ge_big_int_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn ge_big_uint_big_uint(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .ge_big_uint_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn ge_big_uint_i32(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = i32::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .ge_big_uint_i32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn ge_big_uint_i64(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = i64::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .ge_big_uint_i64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn ge_big_uint_u32(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .ge_big_uint_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn ge_big_uint_u64(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .ge_big_uint_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn ge_non_zero_big_uint_non_zero_big_uint(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = NonZeroBigUint::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .ge_non_zero_big_uint_non_zero_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn ge_non_zero_big_uint_big_uint(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .ge_non_zero_big_uint_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn ge_non_zero_big_uint_i32(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = i32::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .ge_non_zero_big_uint_i32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn ge_non_zero_big_uint_i64(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = i64::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .ge_non_zero_big_uint_i64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn ge_non_zero_big_uint_u32(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .ge_non_zero_big_uint_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn ge_non_zero_big_uint_u64(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .ge_non_zero_big_uint_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn lt_big_int_big_int(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = BigInt::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .lt_big_int_big_int(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn lt_big_int_i32(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = i32::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .lt_big_int_i32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn lt_big_int_i64(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = i64::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .lt_big_int_i64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn lt_big_int_u32(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .lt_big_int_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn lt_big_int_u64(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .lt_big_int_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn lt_big_uint_big_uint(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .lt_big_uint_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn lt_big_uint_i32(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = i32::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .lt_big_uint_i32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn lt_big_uint_i64(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = i64::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .lt_big_uint_i64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn lt_big_uint_u32(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .lt_big_uint_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn lt_big_uint_u64(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .lt_big_uint_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn lt_non_zero_big_uint_non_zero_big_uint(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = NonZeroBigUint::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .lt_non_zero_big_uint_non_zero_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn lt_non_zero_big_uint_big_uint(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .lt_non_zero_big_uint_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn lt_non_zero_big_uint_i32(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = i32::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .lt_non_zero_big_uint_i32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn lt_non_zero_big_uint_i64(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = i64::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .lt_non_zero_big_uint_i64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn lt_non_zero_big_uint_u32(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .lt_non_zero_big_uint_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn lt_non_zero_big_uint_u64(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .lt_non_zero_big_uint_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn le_big_int_big_int(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = BigInt::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .le_big_int_big_int(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn le_big_int_i32(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = i32::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .le_big_int_i32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn le_big_int_i64(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = i64::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .le_big_int_i64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn le_big_int_u32(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .le_big_int_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn le_big_int_u64(&mut self) {
        let a = BigInt::<StaticApi>::default();
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .le_big_int_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn le_big_uint_big_uint(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .le_big_uint_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn le_big_uint_i32(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = i32::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .le_big_uint_i32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn le_big_uint_i64(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = i64::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .le_big_uint_i64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn le_big_uint_u32(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .le_big_uint_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn le_big_uint_u64(&mut self) {
        let a = BigUint::<StaticApi>::from(0u128);
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .le_big_uint_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn le_non_zero_big_uint_non_zero_big_uint(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = NonZeroBigUint::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .le_non_zero_big_uint_non_zero_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn le_non_zero_big_uint_big_uint(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .le_non_zero_big_uint_big_uint(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn le_non_zero_big_uint_i32(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = i32::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .le_non_zero_big_uint_i32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn le_non_zero_big_uint_i64(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = i64::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .le_non_zero_big_uint_i64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn le_non_zero_big_uint_u32(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .le_non_zero_big_uint_u32(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn le_non_zero_big_uint_u64(&mut self) {
        let a = NonZeroBigUint::<StaticApi>::default();
        let b = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .le_non_zero_big_uint_u64(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn get_block_timestamp(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .get_block_timestamp()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn get_block_nonce(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .get_block_nonce()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn get_block_round(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .get_block_round()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn get_block_epoch(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .get_block_epoch()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn get_block_random_seed(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .get_block_random_seed()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn get_prev_block_timestamp(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .get_prev_block_timestamp()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn get_prev_block_nonce(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .get_prev_block_nonce()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn get_prev_block_round(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .get_prev_block_round()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn get_prev_block_epoch(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .get_prev_block_epoch()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn get_prev_block_random_seed(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .get_prev_block_random_seed()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn epoch_info(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .epoch_info()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn code_hash(&mut self) {
        let address = ManagedAddress::<StaticApi>::zero();

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .code_hash(address)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn get_block_timestamps(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .get_block_timestamps()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn get_block_timestamp_ms(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .get_block_timestamp_ms()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn get_prev_block_timestamp_ms(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .get_prev_block_timestamp_ms()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn get_caller(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .get_caller()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn get_owner_address(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .get_owner_address()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn get_shard_of_address(&mut self) {
        let address = ManagedAddress::<StaticApi>::zero();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .get_shard_of_address(address)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn is_smart_contract(&mut self) {
        let address = ManagedAddress::<StaticApi>::zero();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .is_smart_contract(address)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn get_state_root_hash(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .get_state_root_hash()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn get_tx_hash(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .get_tx_hash()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn get_gas_left(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .get_gas_left()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn get_cumulated_validator_rewards(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .get_cumulated_validator_rewards()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn get_code_metadata(&mut self) {
        let address = ManagedAddress::<StaticApi>::zero();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .get_code_metadata(address)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn is_builtin_function(&mut self) {
        let function_name = ManagedBuffer::new_from_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .is_builtin_function(function_name)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn codec_err_finish(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .codec_err_finish()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn codec_err_storage_key(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .codec_err_storage_key()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn codec_err_storage_get(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .codec_err_storage_get()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn codec_err_storage_set(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .codec_err_storage_set()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn codec_err_event_topic(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .codec_err_event_topic()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn codec_err_event_data(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .codec_err_event_data()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn codec_err_contract_init(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .codec_err_contract_init()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn codec_err_contract_call(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .codec_err_contract_call()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn compute_sha256(&mut self) {
        let input = ManagedBuffer::new_from_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .compute_sha256(input)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn compute_keccak256(&mut self) {
        let input = ManagedBuffer::new_from_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .compute_keccak256(input)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn compute_ripemd160(&mut self) {
        let input = ManagedBuffer::new_from_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .compute_ripemd160(input)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn verify_bls_signature(&mut self) {
        let key = ManagedBuffer::new_from_bytes(&b""[..]);
        let message = ManagedBuffer::new_from_bytes(&b""[..]);
        let signature = ManagedBuffer::new_from_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .verify_bls_signature(key, message, signature)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn verify_ed25519_signature(&mut self) {
        let key = ManagedBuffer::new_from_bytes(&b""[..]);
        let message = ManagedBuffer::new_from_bytes(&b""[..]);
        let signature = ManagedBuffer::new_from_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .verify_ed25519_signature(key, message, signature)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn verify_secp256k1_signature(&mut self) {
        let key = ManagedBuffer::new_from_bytes(&b""[..]);
        let message = ManagedBuffer::new_from_bytes(&b""[..]);
        let signature = ManagedBuffer::new_from_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .verify_secp256k1_signature(key, message, signature)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn verify_custom_secp256k1_signature(&mut self) {
        let key = ManagedBuffer::new_from_bytes(&b""[..]);
        let message = ManagedBuffer::new_from_bytes(&b""[..]);
        let signature = ManagedBuffer::new_from_bytes(&b""[..]);
        let hash_type = MessageHashType::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .verify_custom_secp256k1_signature(key, message, signature, hash_type)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn compute_secp256k1_der_signature(&mut self) {
        let r = ManagedBuffer::new_from_bytes(&b""[..]);
        let s = ManagedBuffer::new_from_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .compute_secp256k1_der_signature(r, s)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn verify_secp256r1_signature(&mut self) {
        let key = ManagedBuffer::new_from_bytes(&b""[..]);
        let message = ManagedBuffer::new_from_bytes(&b""[..]);
        let signature = ManagedBuffer::new_from_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .verify_secp256r1_signature(key, message, signature)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn verify_bls_signature_share(&mut self) {
        let key = ManagedBuffer::new_from_bytes(&b""[..]);
        let message = ManagedBuffer::new_from_bytes(&b""[..]);
        let signature = ManagedBuffer::new_from_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .verify_bls_signature_share(key, message, signature)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn verify_bls_aggregated_signature(&mut self) {
        let key = ManagedVec::from_single_item(ManagedBuffer::new_from_bytes(&b""[..]));
        let message = ManagedBuffer::new_from_bytes(&b""[..]);
        let signature = ManagedBuffer::new_from_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .verify_bls_aggregated_signature(key, message, signature)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn echo_u64(&mut self) {
        let i = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .echo_u64(i)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn echo_i64(&mut self) {
        let i = i64::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .echo_i64(i)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn echo_i32(&mut self) {
        let i = i32::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .echo_i32(i)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn echo_u32(&mut self) {
        let i = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .echo_u32(i)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn echo_isize(&mut self) {
        let i = i32::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .echo_isize(i)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn echo_usize(&mut self) {
        let i = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .echo_usize(i)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn echo_i8(&mut self) {
        let i = i8::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .echo_i8(i)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn echo_u8(&mut self) {
        let i = 0u8;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .echo_u8(i)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn echo_bool(&mut self) {
        let i = bool::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .echo_bool(i)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn echo_opt_bool(&mut self) {
        let i = Option::Some(bool::<StaticApi>::default());

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .echo_opt_bool(i)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn echo_nothing(&mut self) {
        let nothing = ()::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .echo_nothing(nothing)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn echo_array_u8(&mut self) {
        let s = [0u8;5];

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .echo_array_u8(s)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn echo_multi_value_u32(&mut self) {
        let m = MultiValueVec::from(vec![0u32]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .echo_multi_value_u32(m)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn echo_multi_value_tuples(&mut self) {
        let m = MultiValueVec::<MultiValue2::<i32<StaticApi>, ManagedBuffer<StaticApi>>>::new();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .echo_multi_value_tuples(m)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn echo_ser_example_2(&mut self) {
        let se = ExampleEnumWithFields::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .echo_ser_example_2(se)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn echo_simple_enum(&mut self) {
        let se = ExampleEnumSimple::<StaticApi>::default();

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .echo_simple_enum(se)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn finish_simple_enum_variant_1(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .finish_simple_enum_variant_1()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn echo_non_zero_usize(&mut self) {
        let nz = NonZeroUsize::<StaticApi>::default();

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .echo_non_zero_usize(nz)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn echo_some_args_ignore_others(&mut self) {
        let i = i32::<StaticApi>::default();
        let opt = OptionalValue::Some(i32::<StaticApi>::default());
        let _ignore = ignore::<StaticApi>::default();

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .echo_some_args_ignore_others(i, opt, _ignore)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn echo_arrayvec(&mut self) {
        let av = ManagedVec::<StaticApi, i32<StaticApi>>::new();

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .echo_arrayvec(av)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn echo_big_uint(&mut self) {
        let bu = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .echo_big_uint(bu)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn echo_big_int(&mut self) {
        let bi = BigInt::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .echo_big_int(bi)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn echo_non_zero_big_uint(&mut self) {
        let nzbu = NonZeroBigUint::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .echo_non_zero_big_uint(nzbu)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn echo_managed_buffer(&mut self) {
        let mb = ManagedBuffer::new_from_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .echo_managed_buffer(mb)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn echo_managed_address(&mut self) {
        let ma = ManagedAddress::<StaticApi>::zero();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .echo_managed_address(ma)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn echo_managed_option(&mut self) {
        let mo = Option::Some(BigUint::<StaticApi>::from(0u128));

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .echo_managed_option(mo)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn echo_big_int_managed_vec(&mut self) {
        let x = ManagedVec::<StaticApi, BigInt<StaticApi>>::new();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .echo_big_int_managed_vec(x)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn echo_big_int_tuple(&mut self) {
        let x = BigInt,bytes>::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .echo_big_int_tuple(x)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn echo_big_int_option(&mut self) {
        let x = Option::Some(BigInt::<StaticApi>::default());

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .echo_big_int_option(x)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn echo_tuple_into_multiresult(&mut self) {
        let addr = ManagedAddress::<StaticApi>::zero();
        let vec = ManagedVec::from_single_item(ManagedBuffer::new_from_bytes(&b""[..]));

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .echo_tuple_into_multiresult(addr, vec)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn echo_managed_vec_of_managed_vec(&mut self) {
        let mv = ManagedVec::from_single_item(ManagedVec::from_single_item(0u32));

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .echo_managed_vec_of_managed_vec(mv)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn echo_managed_vec_of_token_identifier(&mut self) {
        let mv = ManagedVec::<StaticApi, TokenIdentifier<StaticApi>>::new();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .echo_managed_vec_of_token_identifier(mv)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn echo_managed_async_result_empty(&mut self) {
        let a = ()>::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .echo_managed_async_result_empty(a)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn echo_varags_managed_eager(&mut self) {
        let m = MultiValueVec::from(vec![0u32]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .echo_varags_managed_eager(m)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn echo_varags_managed_sum(&mut self) {
        let m = MultiValueVec::from(vec![MultiValue2::<u32, u32>::from((0u32, 0u32))]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .echo_varags_managed_sum(m)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn echo_varags_vec_with_counted(&mut self) {
        let m = MultiValueVec::<MultiValue2::<ManagedBuffer<StaticApi>, u32<StaticApi>>>::new();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .echo_varags_vec_with_counted(m)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn echo_varags_vec_with_counted_pairs(&mut self) {
        let m = MultiValueVec::<MultiValue3::<ManagedBuffer<StaticApi>, multi<u32<StaticApi>, ManagedAddress<StaticApi>>>::new();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .echo_varags_vec_with_counted_pairs(m)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn convert_varags_vec_with_counted_pairs_1(&mut self) {
        let address_number_pairs = MultiValueVec::<MultiValue4::<ManagedAddress<StaticApi>, u32, multi<u32<StaticApi>, u32>>::new();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .convert_varags_vec_with_counted_pairs_1(address_number_pairs)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn convert_varags_vec_with_counted_pairs_2(&mut self) {
        let address_number_pairs = MultiValueVec::<MultiValue4::<ManagedAddress<StaticApi>, u32, multi<u32<StaticApi>, u32>>::new();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .convert_varags_vec_with_counted_pairs_2(address_number_pairs)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn compute_get_values(&mut self) {
        let curve_bitsize = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .compute_get_values(curve_bitsize)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn compute_create_ec(&mut self) {
        let curve = ManagedBuffer::new_from_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .compute_create_ec(curve)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn compute_get_ec_length(&mut self) {
        let curve_bitsize = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .compute_get_ec_length(curve_bitsize)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn compute_get_priv_key_byte_length(&mut self) {
        let curve_bitsize = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .compute_get_priv_key_byte_length(curve_bitsize)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn compute_ec_add(&mut self) {
        let curve_bitsize = 0u32;
        let x_first_point = BigUint::<StaticApi>::from(0u128);
        let y_first_point = BigUint::<StaticApi>::from(0u128);
        let x_second_point = BigUint::<StaticApi>::from(0u128);
        let y_second_point = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .compute_ec_add(curve_bitsize, x_first_point, y_first_point, x_second_point, y_second_point)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn compute_ec_double(&mut self) {
        let curve_bitsize = 0u32;
        let x_point = BigUint::<StaticApi>::from(0u128);
        let y_point = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .compute_ec_double(curve_bitsize, x_point, y_point)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn compute_is_on_curve_ec(&mut self) {
        let curve_bitsize = 0u32;
        let x_point = BigUint::<StaticApi>::from(0u128);
        let y_point = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .compute_is_on_curve_ec(curve_bitsize, x_point, y_point)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn compute_scalar_mult(&mut self) {
        let curve_bitsize = 0u32;
        let x_point = BigUint::<StaticApi>::from(0u128);
        let y_point = BigUint::<StaticApi>::from(0u128);
        let data = ManagedBuffer::new_from_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .compute_scalar_mult(curve_bitsize, x_point, y_point, data)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn compute_scalar_base_mult(&mut self) {
        let curve_bitsize = 0u32;
        let data = ManagedBuffer::new_from_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .compute_scalar_base_mult(curve_bitsize, data)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn compute_marshal_ec(&mut self) {
        let curve_bitsize = 0u32;
        let x_pair = BigUint::<StaticApi>::from(0u128);
        let y_pair = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .compute_marshal_ec(curve_bitsize, x_pair, y_pair)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn compute_marshal_compressed_ec(&mut self) {
        let curve_bitsize = 0u32;
        let x_pair = BigUint::<StaticApi>::from(0u128);
        let y_pair = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .compute_marshal_compressed_ec(curve_bitsize, x_pair, y_pair)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn compute_unmarshal_ec(&mut self) {
        let curve_bitsize = 0u32;
        let data = ManagedBuffer::new_from_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .compute_unmarshal_ec(curve_bitsize, data)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn compute_unmarshal_compressed_ec(&mut self) {
        let curve_bitsize = 0u32;
        let data = ManagedBuffer::new_from_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .compute_unmarshal_compressed_ec(curve_bitsize, data)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn compute_generate_key_ec(&mut self) {
        let curve_bitsize = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .compute_generate_key_ec(curve_bitsize)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn log_event_a(&mut self) {
        let data = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .log_event_a(data)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn log_event_a_repeat(&mut self) {
        let num_logs = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .log_event_a_repeat(num_logs)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn log_event_b(&mut self) {
        let arg1 = BigUint::<StaticApi>::from(0u128);
        let arg2 = ManagedAddress::<StaticApi>::zero();
        let data = MultiValueVec::from(vec![ManagedBuffer::new_from_bytes(&b""[..])]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .log_event_b(arg1, arg2, data)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn only_owner_endpoint(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .only_owner_endpoint()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn only_user_account_endpoint(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .only_user_account_endpoint()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn require_equals(&mut self) {
        let a = 0u32;
        let b = 0u32;

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .require_equals(a, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn sc_panic(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .sc_panic()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn maddress_from_array(&mut self) {
        let array = [0u8;32];

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .maddress_from_array(array)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn maddress_from_managed_buffer(&mut self) {
        let managed_buffer = ManagedBuffer::new_from_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .maddress_from_managed_buffer(managed_buffer)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mbuffer_new(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mbuffer_new()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mbuffer_concat(&mut self) {
        let mb1 = ManagedBuffer::new_from_bytes(&b""[..]);
        let mb2 = ManagedBuffer::new_from_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mbuffer_concat(mb1, mb2)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mbuffer_copy_slice(&mut self) {
        let mb = ManagedBuffer::new_from_bytes(&b""[..]);
        let starting_position = 0u32;
        let slice_len = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mbuffer_copy_slice(mb, starting_position, slice_len)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mbuffer_set_random(&mut self) {
        let nr_bytes = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mbuffer_set_random(nr_bytes)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mbuffer_eq(&mut self) {
        let mb1 = ManagedBuffer::new_from_bytes(&b""[..]);
        let mb2 = ManagedBuffer::new_from_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mbuffer_eq(mb1, mb2)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn managed_address_zero(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .managed_address_zero()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn managed_address_eq(&mut self) {
        let mb1 = ManagedAddress::<StaticApi>::zero();
        let mb2 = ManagedAddress::<StaticApi>::zero();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .managed_address_eq(mb1, mb2)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn managed_vec_new(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .managed_vec_new()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn managed_vec_biguint_push(&mut self) {
        let mv = ManagedVec::from_single_item(BigUint::<StaticApi>::from(0u128));
        let item = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .managed_vec_biguint_push(mv, item)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn managed_vec_biguint_eq(&mut self) {
        let mv1 = ManagedVec::from_single_item(BigUint::<StaticApi>::from(0u128));
        let mv2 = ManagedVec::from_single_item(BigUint::<StaticApi>::from(0u128));

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .managed_vec_biguint_eq(mv1, mv2)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn managed_vec_address_push(&mut self) {
        let mv = ManagedVec::from_single_item(ManagedAddress::<StaticApi>::zero());
        let item = ManagedAddress::<StaticApi>::zero();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .managed_vec_address_push(mv, item)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn managed_vec_set(&mut self) {
        let mv = ManagedVec::from_single_item(BigUint::<StaticApi>::from(0u128));
        let index = 0u32;
        let item = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .managed_vec_set(mv, index, item)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn managed_vec_remove(&mut self) {
        let mv = ManagedVec::from_single_item(BigUint::<StaticApi>::from(0u128));
        let index = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .managed_vec_remove(mv, index)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn managed_vec_find(&mut self) {
        let mv = ManagedVec::from_single_item(BigUint::<StaticApi>::from(0u128));
        let item = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .managed_vec_find(mv, item)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn managed_vec_contains(&mut self) {
        let mv = ManagedVec::from_single_item(BigUint::<StaticApi>::from(0u128));
        let item = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .managed_vec_contains(mv, item)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn managed_ref_explicit(&mut self) {
        let mv = ManagedVec::from_single_item(BigUint::<StaticApi>::from(0u128));
        let index = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .managed_ref_explicit(mv, index)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn storage_read_raw(&mut self) {
        let storage_key = ManagedBuffer::new_from_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .storage_read_raw(storage_key)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn storage_write_raw(&mut self) {
        let storage_key = ManagedBuffer::new_from_bytes(&b""[..]);
        let value = ManagedBuffer::new_from_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .storage_write_raw(storage_key, value)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn storage_read_from_address(&mut self) {
        let address = ManagedAddress::<StaticApi>::zero();
        let storage_key = ManagedBuffer::new_from_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .storage_read_from_address(address, storage_key)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn load_bytes(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .load_bytes()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn load_big_uint(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .load_big_uint()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn load_big_int(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .load_big_int()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn load_u64(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .load_u64()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn load_usize(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .load_usize()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn load_i64(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .load_i64()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn load_bool(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .load_bool()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn load_addr(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .load_addr()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn load_opt_addr(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .load_opt_addr()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn is_empty_opt_addr(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .is_empty_opt_addr()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn get_nr_to_clear(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .get_nr_to_clear()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn clear_storage_value(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .clear_storage_value()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn load_ser_2(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .load_ser_2()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn load_map1(&mut self) {
        let addr = ManagedAddress::<StaticApi>::zero();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .load_map1(addr)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn load_map2(&mut self) {
        let addr1 = ManagedAddress::<StaticApi>::zero();
        let addr2 = ManagedAddress::<StaticApi>::zero();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .load_map2(addr1, addr2)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn load_map3(&mut self) {
        let x = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .load_map3(x)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn load_from_address_raw(&mut self) {
        let address = ManagedAddress::<StaticApi>::zero();
        let key = ManagedBuffer::new_from_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .load_from_address_raw(address, key)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn store_bytes(&mut self) {
        let bi = ManagedBuffer::new_from_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .store_bytes(bi)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn store_big_uint(&mut self) {
        let bi = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .store_big_uint(bi)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn store_big_int(&mut self) {
        let bi = BigInt::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .store_big_int(bi)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn store_usize(&mut self) {
        let i = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .store_usize(i)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn store_i32(&mut self) {
        let i = i32::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .store_i32(i)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn store_u64(&mut self) {
        let i = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .store_u64(i)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn store_i64(&mut self) {
        let i = i64::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .store_i64(i)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn store_bool(&mut self) {
        let i = bool::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .store_bool(i)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn store_addr(&mut self) {
        let arg = ManagedAddress::<StaticApi>::zero();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .store_addr(arg)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn store_opt_addr(&mut self) {
        let opt_addr = OptionalValue::Some(ManagedAddress::<StaticApi>::zero());

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .store_opt_addr(opt_addr)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn store_ser_2(&mut self) {
        let arg = ExampleEnumWithFields::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .store_ser_2(arg)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn store_map1(&mut self) {
        let addr = ManagedAddress::<StaticApi>::zero();
        let bi = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .store_map1(addr, bi)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn store_map2(&mut self) {
        let addr1 = ManagedAddress::<StaticApi>::zero();
        let addr2 = ManagedAddress::<StaticApi>::zero();
        let bi = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .store_map2(addr1, addr2, bi)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn store_map3(&mut self) {
        let x = 0u32;
        let b = bool::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .store_map3(x, b)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn store_reserved_i64(&mut self) {
        let i = i64::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .store_reserved_i64(i)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn store_reserved_big_uint(&mut self) {
        let i = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .store_reserved_big_uint(i)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn store_reserved_vec_u8(&mut self) {
        let i = ManagedBuffer::new_from_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .store_reserved_vec_u8(i)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn token_has_transfer_role(&mut self) {
        let token_identifier = TokenIdentifier::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .token_has_transfer_role(token_identifier)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn timelock_mapper(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .timelock_mapper()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn timelock_set_initial_value(&mut self) {
        let initial_value = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .timelock_set_initial_value(initial_value)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn timelock_set_unlock_timestamp(&mut self) {
        let unlock_timestamp = 0u64;
        let future_value = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .timelock_set_unlock_timestamp(unlock_timestamp, future_value)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn timelock_commit_action(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .timelock_commit_action()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn timelock_get_unlock_timestamp(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .timelock_get_unlock_timestamp()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn timelock_get_future_value(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .timelock_get_future_value()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn timelock_get_current_value_at_address(&mut self) {
        let address = ManagedAddress::<StaticApi>::zero();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .timelock_get_current_value_at_address(address)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn timelock_get_unlock_timestamp_at_address(&mut self) {
        let address = ManagedAddress::<StaticApi>::zero();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .timelock_get_unlock_timestamp_at_address(address)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn timelock_get_future_value_at_address(&mut self) {
        let address = ManagedAddress::<StaticApi>::zero();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .timelock_get_future_value_at_address(address)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn address_to_id_mapper_get_id(&mut self) {
        let address = ManagedAddress::<StaticApi>::zero();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .address_to_id_mapper_get_id(address)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn address_to_id_mapper_get_id_non_zero(&mut self) {
        let address = ManagedAddress::<StaticApi>::zero();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .address_to_id_mapper_get_id_non_zero(address)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn address_to_id_mapper_get_address(&mut self) {
        let address_id = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .address_to_id_mapper_get_address(address_id)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn address_to_id_mapper_contains(&mut self) {
        let address_id = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .address_to_id_mapper_contains(address_id)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn address_to_id_mapper_set(&mut self) {
        let address = ManagedAddress::<StaticApi>::zero();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .address_to_id_mapper_set(address)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn address_to_id_mapper_get_id_or_insert(&mut self) {
        let address = ManagedAddress::<StaticApi>::zero();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .address_to_id_mapper_get_id_or_insert(address)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn address_to_id_mapper_remove_by_id(&mut self) {
        let address_id = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .address_to_id_mapper_remove_by_id(address_id)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn address_to_id_mapper_remove_by_address(&mut self) {
        let address = ManagedAddress::<StaticApi>::zero();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .address_to_id_mapper_remove_by_address(address)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn list_mapper(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .list_mapper()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn list_mapper_push_back(&mut self) {
        let item = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .list_mapper_push_back(item)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn list_mapper_push_front(&mut self) {
        let item = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .list_mapper_push_front(item)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn list_mapper_pop_front(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .list_mapper_pop_front()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn list_mapper_pop_back(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .list_mapper_pop_back()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn list_mapper_front(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .list_mapper_front()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn list_mapper_back(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .list_mapper_back()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn list_mapper_push_after(&mut self) {
        let node_id = 0u32;
        let element = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .list_mapper_push_after(node_id, element)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn list_mapper_push_before(&mut self) {
        let node_id = 0u32;
        let element = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .list_mapper_push_before(node_id, element)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn list_mapper_remove_node(&mut self) {
        let node_id = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .list_mapper_remove_node(node_id)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn list_mapper_remove_node_by_id(&mut self) {
        let node_id = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .list_mapper_remove_node_by_id(node_id)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn list_mapper_set_value(&mut self) {
        let node_id = 0u32;
        let new_value = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .list_mapper_set_value(node_id, new_value)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn list_mapper_set_value_by_id(&mut self) {
        let node_id = 0u32;
        let new_value = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .list_mapper_set_value_by_id(node_id, new_value)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn list_mapper_iterate_by_hand(&mut self) {
        let node_id = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .list_mapper_iterate_by_hand(node_id)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn list_mapper_iterate_by_iter(&mut self) {
        let node_id = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .list_mapper_iterate_by_iter(node_id)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn queue_mapper(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .queue_mapper()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn queue_mapper_push_back(&mut self) {
        let item = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .queue_mapper_push_back(item)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn queue_mapper_pop_front(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .queue_mapper_pop_front()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn queue_mapper_front(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .queue_mapper_front()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn map_mapper(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .map_mapper()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn map_mapper_keys(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .map_mapper_keys()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn map_mapper_values(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .map_mapper_values()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn map_mapper_insert(&mut self) {
        let item = 0u32;
        let value = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .map_mapper_insert(item, value)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn map_mapper_contains_key(&mut self) {
        let item = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .map_mapper_contains_key(item)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn map_mapper_get(&mut self) {
        let item = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .map_mapper_get(item)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn map_mapper_remove(&mut self) {
        let item = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .map_mapper_remove(item)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn map_mapper_entry_or_default_update_increment(&mut self) {
        let item = 0u32;
        let increment = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .map_mapper_entry_or_default_update_increment(item, increment)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn map_mapper_entry_or_insert_default(&mut self) {
        let item = 0u32;
        let default = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .map_mapper_entry_or_insert_default(item, default)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn map_mapper_entry_and_modify(&mut self) {
        let item = 0u32;
        let increment = 0u32;
        let otherwise = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .map_mapper_entry_and_modify(item, increment, otherwise)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn map_mapper_entry_or_insert_with_key(&mut self) {
        let item = 0u32;
        let key_increment = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .map_mapper_entry_or_insert_with_key(item, key_increment)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn map_storage_mapper_view(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .map_storage_mapper_view()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn map_storage_mapper_insert_default(&mut self) {
        let item = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .map_storage_mapper_insert_default(item)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn map_storage_mapper_contains_key(&mut self) {
        let item = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .map_storage_mapper_contains_key(item)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn map_storage_mapper_get(&mut self) {
        let item = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .map_storage_mapper_get(item)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn map_storage_mapper_insert_value(&mut self) {
        let item = 0u32;
        let key = 0u32;
        let value = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .map_storage_mapper_insert_value(item, key, value)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn map_storage_mapper_get_value(&mut self) {
        let item = 0u32;
        let key = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .map_storage_mapper_get_value(item, key)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn map_storage_mapper_remove(&mut self) {
        let item = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .map_storage_mapper_remove(item)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn map_storage_mapper_clear(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .map_storage_mapper_clear()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn map_storage_mapper_entry_or_default_update_increment(&mut self) {
        let item = 0u32;
        let key = 0u32;
        let increment = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .map_storage_mapper_entry_or_default_update_increment(item, key, increment)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn map_storage_mapper_entry_and_modify_increment_or_default(&mut self) {
        let item = 0u32;
        let key = 0u32;
        let value = 0u32;
        let other = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .map_storage_mapper_entry_and_modify_increment_or_default(item, key, value, other)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn map_storage_mapper_entry_or_default_update(&mut self) {
        let item = 0u32;
        let key = 0u32;
        let value = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .map_storage_mapper_entry_or_default_update(item, key, value)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn set_mapper(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .set_mapper()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn set_mapper_insert(&mut self) {
        let item = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .set_mapper_insert(item)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn set_mapper_contains(&mut self) {
        let item = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .set_mapper_contains(item)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn set_mapper_remove(&mut self) {
        let item = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .set_mapper_remove(item)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn set_mapper_front(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .set_mapper_front()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn set_mapper_back(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .set_mapper_back()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn set_mapper_next(&mut self) {
        let item = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .set_mapper_next(item)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn set_mapper_previous(&mut self) {
        let item = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .set_mapper_previous(item)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn set_mapper_iter_from_and_count(&mut self) {
        let item = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .set_mapper_iter_from_and_count(item)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn map_my_single_value_mapper(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .map_my_single_value_mapper()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn my_single_value_mapper_increment_1(&mut self) {
        let amount = BigInt::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .my_single_value_mapper_increment_1(amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn my_single_value_mapper_increment_2(&mut self) {
        let amount = BigInt::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .my_single_value_mapper_increment_2(amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn my_single_value_mapper_subtract_with_require(&mut self) {
        let amount = BigInt::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .my_single_value_mapper_subtract_with_require(amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn my_single_value_mapper_set_if_empty(&mut self) {
        let value = BigInt::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .my_single_value_mapper_set_if_empty(value)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn clear_single_value_mapper(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .clear_single_value_mapper()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn get_from_address_single_value_mapper(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .get_from_address_single_value_mapper()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn is_empty_single_value_mapper(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .is_empty_single_value_mapper()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn is_empty_at_address_single_value_mapper(&mut self) {
        let address = ManagedAddress::<StaticApi>::zero();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .is_empty_at_address_single_value_mapper(address)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn raw_byte_length_single_value_mapper(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .raw_byte_length_single_value_mapper()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn set_single_value_mapper_with_key(&mut self) {
        let key = 0u32;
        let value = ManagedBuffer::new_from_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .set_single_value_mapper_with_key(key, value)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn vec_mapper(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .vec_mapper()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn vec_mapper_push(&mut self) {
        let item = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .vec_mapper_push(item)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn vec_mapper_get(&mut self) {
        let index = 0u32;

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .vec_mapper_get(index)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn vec_mapper_get_at_address(&mut self) {
        let address = ManagedAddress::<StaticApi>::zero();
        let index = 0u32;

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .vec_mapper_get_at_address(address, index)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn vec_mapper_len(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .vec_mapper_len()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn vec_mapper_len_at_address(&mut self) {
        let address = ManagedAddress::<StaticApi>::zero();

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .vec_mapper_len_at_address(address)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn token_attributes_set(&mut self) {
        let token_id = TokenIdentifier::<StaticApi>::default();
        let token_nonce = 0u64;
        let attributes = TokenAttributesStruct::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .token_attributes_set(token_id, token_nonce, attributes)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn token_attributes_update(&mut self) {
        let token_id = TokenIdentifier::<StaticApi>::default();
        let token_nonce = 0u64;
        let attributes = TokenAttributesStruct::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .token_attributes_update(token_id, token_nonce, attributes)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn token_attributes_get_attributes(&mut self) {
        let token_id = TokenIdentifier::<StaticApi>::default();
        let token_nonce = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .token_attributes_get_attributes(token_id, token_nonce)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn token_attributes_get_nonce(&mut self) {
        let token_id = TokenIdentifier::<StaticApi>::default();
        let attributes = TokenAttributesStruct::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .token_attributes_get_nonce(token_id, attributes)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn token_attributes_clear(&mut self) {
        let token_id = TokenIdentifier::<StaticApi>::default();
        let token_nonce = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .token_attributes_clear(token_id, token_nonce)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn token_attributes_has_attributes(&mut self) {
        let token_id = TokenIdentifier::<StaticApi>::default();
        let token_nonce = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .token_attributes_has_attributes(token_id, token_nonce)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn add_to_whitelist(&mut self) {
        let item = ManagedBuffer::new_from_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .add_to_whitelist(item)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn remove_from_whitelist(&mut self) {
        let item = ManagedBuffer::new_from_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .remove_from_whitelist(item)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn check_contains(&mut self) {
        let item = ManagedBuffer::new_from_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .check_contains(item)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn check_contains_at_address(&mut self) {
        let address = ManagedAddress::<StaticApi>::zero();
        let item = ManagedBuffer::new_from_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .check_contains_at_address(address, item)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn require_contains(&mut self) {
        let item = ManagedBuffer::new_from_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .require_contains(item)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn require_contains_at_address(&mut self) {
        let address = ManagedAddress::<StaticApi>::zero();
        let item = ManagedBuffer::new_from_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .require_contains_at_address(address, item)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn issue_fungible_default_callback(&mut self) {
        let egld_amount = BigUint::<StaticApi>::from(0u128);

        let token_ticker = ManagedBuffer::new_from_bytes(&b""[..]);
        let initial_supply = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .issue_fungible_default_callback(token_ticker, initial_supply)
            .egld(egld_amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn issue_fungible_custom_callback(&mut self) {
        let egld_amount = BigUint::<StaticApi>::from(0u128);

        let token_ticker = ManagedBuffer::new_from_bytes(&b""[..]);
        let initial_supply = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .issue_fungible_custom_callback(token_ticker, initial_supply)
            .egld(egld_amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn issue_and_set_all_roles_fungible(&mut self) {
        let egld_amount = BigUint::<StaticApi>::from(0u128);

        let token_ticker = ManagedBuffer::new_from_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .issue_and_set_all_roles_fungible(token_ticker)
            .egld(egld_amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn set_local_roles_fungible(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .set_local_roles_fungible()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mint_fungible(&mut self) {
        let amount = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mint_fungible(amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mint_and_send_fungible(&mut self) {
        let to = ManagedAddress::<StaticApi>::zero();
        let amount = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mint_and_send_fungible(to, amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn burn_fungible(&mut self) {
        let amount = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .burn_fungible(amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn get_balance_fungible(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .get_balance_fungible()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn require_same_token_fungible(&mut self) {
        let token_id = String::new();
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .require_same_token_fungible()
            .payment((EsdtTokenIdentifier::from(token_id.as_str()), token_nonce, token_amount))
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn require_all_same_token_fungible(&mut self) {
        let token_id = String::new();
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .require_all_same_token_fungible()
            .payment((EsdtTokenIdentifier::from(token_id.as_str()), token_nonce, token_amount))
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn fungible_token_mapper(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .fungible_token_mapper()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn issue_and_set_all_roles_meta(&mut self) {
        let egld_amount = BigUint::<StaticApi>::from(0u128);

        let token_ticker = ManagedBuffer::new_from_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .issue_and_set_all_roles_meta(token_ticker)
            .egld(egld_amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mapper_nft_set_token_id(&mut self) {
        let token_id = TokenIdentifier::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mapper_nft_set_token_id(token_id)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mapper_nft_create(&mut self) {
        let amount = BigUint::<StaticApi>::from(0u128);
        let attributes = RgbColor::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mapper_nft_create(amount, attributes)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mapper_nft_create_and_send(&mut self) {
        let to = ManagedAddress::<StaticApi>::zero();
        let amount = BigUint::<StaticApi>::from(0u128);
        let attributes = RgbColor::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mapper_nft_create_and_send(to, amount, attributes)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mapper_nft_add_quantity(&mut self) {
        let token_nonce = 0u64;
        let amount = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mapper_nft_add_quantity(token_nonce, amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mapper_nft_add_quantity_and_send(&mut self) {
        let to = ManagedAddress::<StaticApi>::zero();
        let token_nonce = 0u64;
        let amount = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mapper_nft_add_quantity_and_send(to, token_nonce, amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mapper_nft_burn(&mut self) {
        let token_nonce = 0u64;
        let amount = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mapper_nft_burn(token_nonce, amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mapper_nft_get_balance(&mut self) {
        let token_nonce = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mapper_nft_get_balance(token_nonce)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mapper_get_token_attributes(&mut self) {
        let token_nonce = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mapper_get_token_attributes(token_nonce)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn non_fungible_token_mapper(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .non_fungible_token_mapper()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn init_unique_id_mapper(&mut self) {
        let len = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .init_unique_id_mapper(len)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn unique_id_mapper_get(&mut self) {
        let index = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .unique_id_mapper_get(index)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn unique_id_mapper_swap_remove(&mut self) {
        let index = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .unique_id_mapper_swap_remove(index)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn unique_id_mapper_set(&mut self) {
        let index = 0u32;
        let id = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .unique_id_mapper_set(index, id)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn unique_id_mapper(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .unique_id_mapper()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn unordered_set_mapper(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .unordered_set_mapper()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn unordered_set_mapper_insert(&mut self) {
        let item = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .unordered_set_mapper_insert(item)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn unordered_set_mapper_contains(&mut self) {
        let item = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .unordered_set_mapper_contains(item)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn unordered_set_mapper_remove(&mut self) {
        let item = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .unordered_set_mapper_remove(item)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn managed_struct_eq(&mut self) {
        let s1 = ExampleStructManaged::<StaticApi>::default();
        let s2 = ExampleStructManaged::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .managed_struct_eq(s1, s2)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn no_overflow_usize(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .no_overflow_usize()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn no_overflow_u8(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .no_overflow_u8()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn no_overflow_u16(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .no_overflow_u16()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn no_overflow_u32(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .no_overflow_u32()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn no_overflow_u64(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .no_overflow_u64()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn overflow_usize(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .overflow_usize()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn overflow_u8(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .overflow_u8()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn overflow_u16(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .overflow_u16()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn overflow_u32(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .overflow_u32()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn overflow_u64(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .overflow_u64()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn no_overflow_isize(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .no_overflow_isize()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn no_overflow_i8(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .no_overflow_i8()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn no_overflow_i16(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .no_overflow_i16()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn no_overflow_i32(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .no_overflow_i32()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn no_overflow_i64(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .no_overflow_i64()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn overflow_isize(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .overflow_isize()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn overflow_i8(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .overflow_i8()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn overflow_i16(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .overflow_i16()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn overflow_i32(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .overflow_i32()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn overflow_i64(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .overflow_i64()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn token_identifier_egld(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .token_identifier_egld()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn token_identifier_is_valid_1(&mut self) {
        let token_id = EgldOrEsdtTokenIdentifier::esdt(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .token_identifier_is_valid_1(token_id)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn token_identifier_is_valid_2(&mut self) {
        let bytes = ManagedBuffer::new_from_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .token_identifier_is_valid_2(bytes)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn non_zero_usize_iter(&mut self) {
        let how_many = 0u32;

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .non_zero_usize_iter(how_many)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn non_zero_usize_macro(&mut self) {
        let number = 0u32;

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .non_zero_usize_macro(number)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn returns_egld_decimal(&mut self) {
        let egld_amount = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .returns_egld_decimal()
            .egld(egld_amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn set_contract_address(&mut self) {
        let address = ManagedAddress::<StaticApi>::zero();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .set_contract_address(address)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn is_empty_at_address(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .is_empty_at_address()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn contains_at_address(&mut self) {
        let item = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .contains_at_address(item)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn len_at_address(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .len_at_address()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn next_at_address(&mut self) {
        let item = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .next_at_address(item)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn previous_at_address(&mut self) {
        let item = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .previous_at_address(item)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn front_at_address(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .front_at_address()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn back_at_address(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .back_at_address()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn keys_at_address(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .keys_at_address()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn values_at_address(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .values_at_address()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn contains_unordered_at_address(&mut self) {
        let item = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .contains_unordered_at_address(item)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn get_by_index(&mut self) {
        let index = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .get_by_index(index)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn fill_set_mapper(&mut self) {
        let value = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .fill_set_mapper(value)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn fill_map_mapper(&mut self) {
        let value = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .fill_map_mapper(value)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn fill_unordered_set_mapper(&mut self) {
        let value = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .fill_unordered_set_mapper(value)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn get_value_from_address_with_keys(&mut self) {
        let address = ManagedAddress::<StaticApi>::zero();
        let extra_key = 0u32;

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .get_value_from_address_with_keys(address, extra_key)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn address_to_id_mapper_get_id_from_address(&mut self) {
        let address_arg = ManagedAddress::<StaticApi>::zero();

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .address_to_id_mapper_get_id_from_address(address_arg)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn managed_decimal_addition(&mut self) {
        let first = 2>::<StaticApi>::default();
        let second = 2>::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .managed_decimal_addition(first, second)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn managed_decimal_subtraction(&mut self) {
        let first = 2>::<StaticApi>::default();
        let second = 2>::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .managed_decimal_subtraction(first, second)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn managed_decimal_eq(&mut self) {
        let first = 2>::<StaticApi>::default();
        let second = 2>::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .managed_decimal_eq(first, second)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn managed_decimal_trunc(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .managed_decimal_trunc()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn managed_decimal_into_raw_units(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .managed_decimal_into_raw_units()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn managed_decimal_ln(&mut self) {
        let x = 9>::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .managed_decimal_ln(x)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn managed_decimal_log2(&mut self) {
        let x = 9>::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .managed_decimal_log2(x)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn managed_decimal_addition_var(&mut self) {
        let first = usize>::<StaticApi>::default();
        let second = usize>::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .managed_decimal_addition_var(first, second)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn managed_decimal_subtraction_var(&mut self) {
        let first = usize>::<StaticApi>::default();
        let second = usize>::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .managed_decimal_subtraction_var(first, second)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn managed_decimal_eq_var(&mut self) {
        let first = usize>::<StaticApi>::default();
        let second = usize>::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .managed_decimal_eq_var(first, second)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn managed_decimal_ln_var(&mut self) {
        let x = usize>::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .managed_decimal_ln_var(x)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn managed_decimal_log2_var(&mut self) {
        let x = usize>::<StaticApi>::default();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .managed_decimal_log2_var(x)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn mm_get(&mut self) {
        let key = ManagedBuffer::new_from_bytes(&b""[..]);

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mm_get(key)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn mm_contains(&mut self) {
        let key = ManagedBuffer::new_from_bytes(&b""[..]);

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mm_contains(key)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn mm_remove_get(&mut self) {
        let remove_key = ManagedBuffer::new_from_bytes(&b""[..]);
        let get_key = ManagedBuffer::new_from_bytes(&b""[..]);

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mm_remove_get(remove_key, get_key)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn mm_mutable_input_test(&mut self) {
        let key = ManagedBuffer::new_from_bytes(&b""[..]);
        let value = ManagedBuffer::new_from_bytes(&b""[..]);

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(basic_features_proxy::BasicFeaturesProxy)
            .mm_mutable_input_test(key, value)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

}
