use std::{collections::HashMap, fmt};
use super::values::RtValue;

#[derive(Debug)]
pub struct EnvironmentScope {
    pub parent: usize,
    pub variables : HashMap<String,RtValue>
}

impl EnvironmentScope {
    pub fn new( parent : usize  ) -> Self {
        Self {
            parent,
            variables : HashMap::new(),
        }
    }

    pub fn contains(self : &Self, name : &str) -> bool {
        self.variables.contains_key(name)
    }


    pub fn assign(self : &mut Self, name : &str, value : RtValue) {
        *self.variables.get_mut(name).unwrap() = value;         
    }

    pub fn list(self: &Self, pre_str: &str) {
        for v in &self.variables {
            println!("{} {} = {:?}", pre_str, v.0, v.1 )
        }
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

    
    pub fn declare_variable(self : &mut Self, scope: usize, name : &str, value : RtValue ) -> RtValue{
        
        let scope = self.scopes.get_mut(&scope).expect(&format!("invalid scope: {}", scope));

        if !scope.variables.contains_key(name) {
            scope.variables.insert(name.to_string(), value.clone());
        } else {
            panic!("Variable already exists: {}", name);
        }
        
        return value
    }

    pub fn get_value(self : &Self, scope: usize, name : &str) -> RtValue{
        let scope_nr = self.find_scope(scope, name);
        if scope_nr == 0 {
            RtValue::NullVal
        } else {
            self.scopes[&scope_nr].variables[name].clone()
        }
    }

    pub fn assign_value(self : &mut Self, scope: usize, name : &str, value : RtValue) {
        let scope_nr = self.find_scope(scope, name);
        if scope_nr == 0 {
            panic!("Variable {} not found in scope {}", name, scope);
        }
        let scope = self.scopes.get_mut(&scope_nr).unwrap();
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

