// contracts/common-utils/src/oracle.rs

use soroban_sdk::{Env, Address, BytesN, Vec, Map, symbol_short};
use crate::signature::verify_signature;
use crate::types::*;
use crate::storage::OracleKey;