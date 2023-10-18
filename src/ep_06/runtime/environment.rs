use std::{collections::HashMap, collections::HashSet};
use super::values::RtValue;

#[derive(Debug)]
pub struct EnvironmentScope {
    pub parent: usize,
    pub variables : HashMap<String,RtValue>,
    pub constants : HashSet<String>,
}

impl EnvironmentScope {
    pub fn new( parent : usize  ) -> Self {
        Self {
            parent,
            variables : HashMap::new(),
            constants : HashSet::new(),
        }
    }

    pub fn contains(self : &Self, name : &str) -> bool {
        self.variables.contains_key(name)
    }

    pub fn is_constant(self : &Self, name : &str) -> bool {
        self.constants.contains(name)
    }

    pub fn assign(self : &mut Self, name : &str, value : RtValue) {
        *self.variables.get_mut(name).unwrap() = value;         
    }

    pub fn list(self: &Self, pre_str: &str) {
        for v in &self.variables {
            println!("{} {} = {:?}", pre_str, v.0, v.1 )
        }
    }

    pub fn add_constant(self : &mut Self, name : &str) -> bool {
        self.constants.insert(name.into())
    }

}

#[derive(Debug)]
pub struct Environment {
    last_scope : usize,
    pub scopes : HashMap<usize,EnvironmentScope>,
}

impl Environment {

    pub fn new() -> Self {
        Self {
            scopes : HashMap::new(),
            last_scope: 0,
        }
    }

    pub fn create_global_scope( self: &mut Self) -> usize {
        let global_scope = self.add_scope(0);
        self.declare_variable(global_scope, "null", RtValue::NullVal, true);
        self.declare_variable(global_scope, "true", RtValue::Boolean(true), true);
        self.declare_variable(global_scope, "false", RtValue::Boolean(false), true);
        global_scope
    }

    pub fn add_scope( self: &mut Self, parent : usize ) -> usize {
        self.last_scope += 1;
        let new_scope = EnvironmentScope::new(parent);
        self.scopes.insert(self.last_scope, new_scope);
        self.last_scope
    }


    pub fn list_scope(self: &Self, scope_nr: usize) {   
        let pre_str = " ";
        let scope = self.scopes.get(&scope_nr).expect(&format!("invalid scope: {}", scope_nr));
        println!( "{} scope: {} parent {}", pre_str, scope_nr, scope.parent );
        scope.list( pre_str);
    }

    
    pub fn declare_variable(self : &mut Self, scope: usize, name : &str, value : RtValue, constant : bool ) -> RtValue{
        
        let scope = self.scopes.get_mut(&scope).expect(&format!("invalid scope: {}", scope));

        if !scope.variables.contains_key(name) {
            
            scope.variables.insert(name.to_string(), value.clone());
            if constant {
                scope.add_constant(name);
            }
        } else {
            panic!("Variable already exists: {}", name);
        }
        
        return value
    }

    pub fn get_value(self : &Self, scope: usize, name : &str) -> RtValue{
        let scope_nr = self.find_scope(scope, name);
        match scope_nr != 0 {
            true => self.scopes[&scope_nr].variables[name].clone(),
            false => RtValue::NullVal,
        }
    }

    pub fn assign_value(self : &mut Self, scope: usize, name : &str, value : RtValue) {
        let scope_nr = self.find_scope(scope, name);
        if scope_nr == 0 {
            panic!("Variable {} not found in scope {}", name, scope);
        }
        let scope = self.scopes.get_mut(&scope_nr).unwrap();
        if scope.is_constant(name) {
            panic!("Can't change constant value: {}", name);
        }
        scope.assign(name, value);
    }

    fn find_scope(self : &Self , scope: usize, name: &str) -> usize {
        
        let in_scope = self.scopes[ &scope ].contains(name);

        if in_scope {
            scope
        } else {
            let parent = self.scopes[ &scope ].parent; 
            if parent != 0 {
                self.find_scope(parent, name)
            } else {
                parent
            }
        }
    }

    // pub fn assign_variable(self : &mut Self, name : &str, value : RtValue ) -> RtValue{

    //     match self.variables.get_mut(name)  {
    //         Some(x) => {
    //             *x = value.clone();
    //             return value
    //         },
    //         None => 
    //             match self.parent {
    //                 Some(parent) => parent.assign_variable(name, value),
    //                 None => panic!("Can't assign variable: {}", name),
    //             },
    //     }
    // }    
}

