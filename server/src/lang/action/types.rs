
use std::process;
use super::ops::varops::VarOp;
use super::super::var::Varables;

#[derive(Serialize, Deserialize, Debug)]
pub enum Action {
    Var {
        op: VarOp,
        success: AfterAction,
        failure: AfterAction,
    },
}

impl Action {
    fn run_after_op(
        state: &mut Varables,
        result: Result<(), String>,
        success: &AfterAction,
        failure: &AfterAction,
    ) {
        match result {
            Ok(_) => success.run("No Error Occured"),
            Err(message) => failure.run(&message),
        }
    }

    fn do_action(state: &mut Varables, op: &VarOp, success: &AfterAction, failure: &AfterAction) {
        let rst = op.run(state);
        Action::run_after_op(state, rst, success, failure)
    }

    pub fn run(&self, state: &mut Varables) {
        match self {
            &Action::Var {
                ref op,
                ref success,
                ref failure,
            } => Action::do_action(state, op, success, failure),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AfterAction {
    Continue,
    Exit(i32),
    Warn,
    WarnMessage(String),
    Panic,
    PanicMessage(String),
    // macro name
    Next(String),
}

impl AfterAction {
    pub fn run(&self, error_message: &str) {
        match self {
            &AfterAction::Continue => {}
            &AfterAction::Exit(ref code) => process::exit(*code),
            &AfterAction::Warn => println!("{}", error_message),
            &AfterAction::WarnMessage(ref message) => println!("{}", message),
            &AfterAction::Panic => panic!("{}", error_message),
            &AfterAction::PanicMessage(ref message) => panic!("{}", message),
            &AfterAction::Next(ref macro_name) => {}
        }
    }
}
