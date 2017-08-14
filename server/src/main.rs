
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

mod lang;

use lang::Program;
use lang::action::ops::varops::VarOp;
use lang::action::types::{AfterAction, Action};
use lang::var::types::{Var, DeclareVar};

fn main() {
    let mut p = Program::new();

    p.load_actions(vec![
        Action::Var {
            op: VarOp::DeclareVar {
                var_name: String::from("test_string"),
                var: Var::String(None),
            },
            success: AfterAction::Continue,
            failure: AfterAction::Panic,
        },
        Action::Var {
            op: VarOp::DeclareVar {
                var_name: String::from("test_float"),
                var: Var::Float(Some(1.2)),
            },
            success: AfterAction::Continue,
            failure: AfterAction::Panic,
        },
        Action::Var {
            op: VarOp::DeclareVar {
                var_name: String::from("delete_me"),
                var: Var::Int(None),
            },
            success: AfterAction::Continue,
            failure: AfterAction::Panic,
        },
        Action::Var {
            op: VarOp::DeleteVar(String::from("delete_me")),
            success: AfterAction::Warn,
            failure: AfterAction::Panic,
        },
        Action::Var {
            op: VarOp::SetString {
                var_name: String::from("test"),
                var_value: Some(String::from("woot")),
            },
            success: AfterAction::Continue,
            failure: AfterAction::Panic,
        },
    ]);

    p.run();

    println!("Actions {}", p.get_actions_str_pretty().unwrap());
    println!("State {}", p.get_state_str_pretty().unwrap());
    println!("Output {}", p.get_output_str_pretty().unwrap());
}
