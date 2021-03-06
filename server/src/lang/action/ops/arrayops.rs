
use super::super::super::var::types::Var;
use super::super::super::var::Varables;
use super::super::super::output::Output;
use super::Op;
use std::clone::Clone;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ArrayOp {
    Push { array_name: String, var: Var },
    Pop {
        array_name: String,
        var_name: Option<String>,
    },
    Length {
        array_name: String,
        var_name: String,
    },
//    UnShift { array_name: String, var: Var },
//    Shift {
//        array_name: String,
//        var_name: String,
//    },
}

impl Op for ArrayOp {
    fn run(&self, state: &mut Varables, _output: &mut Output) -> Result<String, String> {
        match self {
            &ArrayOp::Push {
                ref array_name,
                ref var,
            } => {
                let rst = state.get_array_mut(array_name);
                match rst {
                    Ok(option_array) => {
                        if let None = *option_array {
                            *option_array = Some(Box::new(Vec::new()));
                        }
                        if let Some(ref mut arr) = *option_array {
                            arr.push(var.clone());
                            return Ok(format!("Pushed To Array: {}", array_name));
                        }
                        Err(format!(
                            "Unable To Dereference Array {} For Push",
                            array_name
                        ))
                    }
                    Err(msg) => Err(msg),
                }
            }
            &ArrayOp::Pop {
                ref array_name,
                ref var_name,
            } => {
                let popped: Result<Var, String>;
                {
                    let rst = state.get_array_mut(array_name);
                    popped = match rst {
                        Ok(option_array) => {
                            match *option_array {
                                Some(ref mut arr) => {
                                    match arr.pop() {
                                        Some(var) => Ok(var),
                                        None => Err(format!("Array {} Is Empty", array_name)),
                                    }
                                }
                                None => Err(format!("Array {} Is Empty", array_name)),
                            }
                        }
                        Err(msg) => Err(msg),
                    };
                }

                match popped {
                    Ok(array_item) => {
                        if let Some(ref name) = *var_name {
                            // create or update var
                            if let Err(msg) = state.re_declare_var(name, &array_item) {
                                return Err(msg);
                            }
                        }
                    }
                    Err(msg) => {
                        return Err(msg);
                    }
                };
                Ok(format!("Popped From Array: {}", array_name))
            }
            &ArrayOp::Length {
                ref array_name,
                ref var_name,
            } => {
                let arr_length: usize;
                {
                    let rst = state.get_array(array_name);
                    arr_length = match rst {
                        Ok(option_array) => {
                            match *option_array {
                                Some(ref arr) => arr.len(),
                                None => 0,
                            }
                        }
                        Err(msg) => return Err(msg),
                    };
                }

                if let Err(msg) = state.re_declare_var(var_name, &Var::Size(Some(arr_length))) {
                    return Err(msg);
                }

                Ok(format!("Got Length Of Array: {}", array_name))
            }
        }
    }
}
