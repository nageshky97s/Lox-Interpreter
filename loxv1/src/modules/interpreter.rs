use super::loxcallable::Callable;
use super::{expr::Accept, token,lox,expr,stmt,stmt::StmtAccept,environment,loxfunction,loxcallable,loxcallable::LoxCallable,loxclass};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;


pub enum Exit {
    RuntimeErr(RuntimeError),
    Return(ReturnVal),
    
}

pub struct RuntimeError{
    pub tok:token::Token,
    pub mess:String
}
#[derive(Debug,Clone)]
pub struct ReturnVal{
    pub value:token::Literals,
}


pub struct Interpreter{
    pub globals:Rc<RefCell<environment::Environment>>,
    pub environment: Rc<RefCell<environment::Environment>>,
    locals:HashMap<expr::Expr,usize>,
}

impl Interpreter {
    pub fn new()->Self{       
            let globals = Rc::new(RefCell::new(environment::Environment::new()));            
            Interpreter {
                globals: Rc::clone(&globals),
                environment: Rc::clone(&globals),
                locals:HashMap::new(),
               }
    
}
    fn evaluate(&mut self,exp:&expr::Expr)->Result<token::Literals,Exit>{
        return exp.accept(self)
    }
    fn is_truthy(&mut self,val:token::Literals)->bool{
        match val {
            token::Literals::Nil=>{
                return false;
            }
            token::Literals::BooleanLit { boolval }=>{return boolval}
            _=>{
                
                return true;
            }
        }
    }
    fn is_equal(&mut self,left:&token::Literals,right:&token::Literals)->bool{
        if  let token::Literals::Nil =left{
            if let token::Literals::Nil =right{
                return true;
            }else{
                println!("Massive Logical Error cannot be a type other than Number here");
                std::process::exit(1);
            }
        } else if let token::Literals::Nil =left {
            return false;
        }else{
            if let token::Literals::StringLit { stringval:x } =left{
                if let token::Literals::StringLit { stringval:y } =right{
                    if x==y{
                        return true
                    }else{
                        return false
                    }
                }else{
                    println!("Massive Logical Error cannot be a type other than Number here");
                    std::process::exit(1);
                }
        }else{
            self.error_exit();
        }
    }
}
fn error_exit(&mut self,)->!{
    println!("Massive Logical Error cannot be a type other than Number here");
    std::process::exit(1);
}
fn check_number_operands(&mut self,tok:token::Token,operand:token::Literals)->Result<(),Exit>{
    if let token::Literals::NumLit { numval:_ } =operand{
        return Ok(()) ;
    }else{
        return Err(Exit::RuntimeErr(RuntimeError{tok:tok.clone(),mess:"Operand must be a number".to_string()}));
       
    }
}
fn check_number_operands_(&mut self,tok:token::Token,left:token::Literals,right:token::Literals)->Result<(),Exit>{
    if let token::Literals::NumLit { numval:_} =left{
        if let token::Literals::NumLit { numval:_ } =right {
            return Ok(()) ;
        }else{
            return Err(Exit::RuntimeErr(RuntimeError{tok:tok.clone(),mess:"Operand must be two numbers or two strings".to_string()}));
            
        }
    }else{
        return Err(Exit::RuntimeErr(RuntimeError{tok:tok.clone(),mess:"Operand must be two numbers or two strings".to_string()}));
    }
}

pub fn interpret_new(&mut self, lox_obj:&mut lox::Lox)-> Result<(), Exit>{
    
   
        match &mut lox_obj.allstatements {
            Some(x)=>{
                for statement in x.iter_mut(){
                   self.execute(&statement)?;
                   
                }
            }
            _=>{}
        }
        
            Ok(())
        
   
}
fn execute(&mut self,stm:&stmt::Stmt)->Result<(), Exit>{
    
    return stm.accept(self);
}

pub fn resolve(&mut self,exp:expr::Expr,depth:usize){
    self.locals.insert(exp.clone(),depth);
}

fn stringify(&mut self,value:token::Literals)->String{
    match value {
        token::Literals::BooleanLit { boolval }=>{return boolval.to_string() ;}
        token::Literals::NumLit { numval }=>{return numval.to_string();}
        token::Literals::Nil =>{return "nil".to_string();}
        token::Literals::StringLit { stringval }=>{return stringval;}
        token::Literals::Callable(c) => match c {
           loxcallable::Callable::Function(func) => func.to_string(),
           loxcallable::Callable::Class(class)=>class.to_string(),
            //_ => "callable".to_string(),
        },
    }
}

pub fn execute_block(&mut self,statements:& Vec< stmt::Stmt>,environment: environment::Environment,)-> Result<(), Exit> {
    let previous = Rc::clone(&self.environment);
    self.environment = Rc::new(RefCell::new(environment)); 
   
    let result = statements.iter().try_for_each(|stat| self.execute(stat));

    self.environment = previous;
    result   
    
}
pub fn look_up_variable(&mut self,name:token::Token,exp:expr::Expr)->Result<token::Literals,Exit>{
    let distance = self.locals.get(&exp);
    if distance.is_some(){
        return self.environment.borrow_mut().get_at(distance.unwrap().clone(),&name);
    }
    else{
        return self.globals.borrow_mut().get(&name);
    }
}

}

impl stmt::StmtVisitor<Result<(),Exit>> for Interpreter{

    fn visit_return_stmt(&mut self, visitor: &stmt::Return) -> Result<(),Exit> {
        let mut value:token::Literals=token::Literals::Nil;
        if visitor.value!=None{
            value=self.evaluate(visitor.value.as_ref().unwrap())?;
        }
        return Err(Exit::Return(ReturnVal{value:value}));

    }

    fn visit_function_stmt(&mut self, visitor: &stmt::Function) -> Result<(),Exit> {
        let function= loxfunction::LoxFunction::new(visitor.clone(),Rc::clone(&self.environment),);
        self.environment.borrow_mut().define(visitor.name.lexeme.clone(),
                                token::Literals::Callable(loxcallable::Callable::Function(function)),);
        Ok(())
    }

    fn visit_while_stmt(&mut self, stm: &stmt::While) -> Result<(),Exit> {
        let mut res =self.evaluate(&stm.condition);       
        
        while self.is_truthy(res?) {
            self.execute(&*stm.body)?;
            res=self.evaluate(&stm.condition);
        }
        Ok(())
    }
    fn visit_expression_stmt(&mut self, stm: &stmt::Expression) -> Result<(),Exit> {
        self.evaluate(&stm.expression)?;
        Ok(())
    }
    fn visit_if_stmt(&mut self, stm: &stmt::If) -> Result<(),Exit>{
        let res=self.evaluate(&stm.condition);
        if self.is_truthy(res?){
            self.execute(&stm.then_branch)?;
        }
        match &*stm.else_branch {
            Some(x)=>{
                self.execute(x)?;
            },
            _=>{}
        }
        Ok(())
    }
    fn visit_print_stmt(&mut self, stm: &stmt::Print) -> Result<(),Exit> {
        
        let value=self.evaluate(&stm.expression);
        println!("{}",self.stringify(value?));
        Ok(())
    }

    fn visit_var_stmt(&mut self, stm: &stmt::Var) -> Result<(),Exit>{
        let value = if let expr::Expr::Literal(expr::Literal {
            uuid: _usize,
            value: token::Literals::Nil,
        }) = stm.initializer
        {
            token::Literals::Nil
        } else {
            self.evaluate(&stm.initializer)?
        };
        self.environment
            .borrow_mut()
            .define(stm.name.lexeme.clone(), value);
        Ok(())
    }

    fn visit_block_stmt(&mut self, visitor: &stmt::Block) -> Result<(),Exit> {
        
        self.execute_block(&visitor.statements,environment::Environment::new_env(self.environment.clone()))?;
        Ok(())
    }
    fn visit_class_stmt(&mut self, visitor: &stmt::Class) -> Result<(),Exit> {
        self.environment.borrow_mut().define(visitor.name.lexeme.clone(),token::Literals::Nil);
        let klass=loxclass::LoxClass::new(visitor.name.lexeme.clone());
        self.environment.borrow_mut().assign(&visitor.name, token::Literals::Callable(Callable::Class(klass)))?;
        Ok(())
    }

}


impl expr::AstVisitor<Result<token::Literals,Exit>> for Interpreter{

    fn visit_call(&mut self, visitor: &expr::Call) -> Result<token::Literals,Exit> {
        let callee=self.evaluate(&visitor.callee)?;
        let mut arguments:Vec<token::Literals>=Vec::new();
        for arg in visitor.arguments.iter(){
            arguments.push(self.evaluate(arg)?);
        }
       
                 
          
        if let token::Literals::Callable(Callable::Function(function)) = callee {
           
            if arguments.len()== function.arity(){
                return function.call(self,&arguments);
            }
            else{
                return Err(Exit::RuntimeErr(RuntimeError{tok:visitor.paren.clone(),
                    mess:"Expected ".to_string()+&function.arity().to_string()+&" arguments but got ".to_string()+
                &arguments.len().to_string()+&".".to_string()}));
               
            }
        }
        return Err(Exit::RuntimeErr(RuntimeError{tok:visitor.paren.clone(),mess:"Can only call functions and classes.".to_string()}));
        
       
    }

    fn visit_logical(&mut self, visitor: &expr::Logical) -> Result<token::Literals,Exit> {
        let left=self.evaluate(&visitor.left)?;
        if visitor.operator.token_type==token::TokenType::Or{
            if self.is_truthy(left.clone()) {return Ok(left);}
        }
        else{
            if !self.is_truthy(left.clone()){
                return Ok(left);
            }
         }
        
        return self.evaluate(&visitor.right);
    }

    fn visit_literal(&mut self, visitor: &expr::Literal) -> Result<token::Literals,Exit> {
        return Ok(visitor.value.clone())
    }
    fn visit_grouping(&mut self, visitor: &expr::Grouping) -> Result<token::Literals,Exit> {
        return self.evaluate(&*visitor.expression)
    }
    fn visit_unary(&mut self, visitor: &expr::Unary) -> Result<token::Literals,Exit> {
        let right:token::Literals=self.evaluate(&visitor.right)?;
       match visitor.operator.token_type  {
        token::TokenType::Bang => {
            
            if !self.is_truthy(right){
                return Ok(token::Literals::BooleanLit{boolval:true}) ;
            }else{
                return Ok(token::Literals::BooleanLit{boolval:false});
            }
        }
        token::TokenType::Minus=>{

        self.check_number_operands(visitor.operator.clone(),right.clone())?;
           if let token::Literals::NumLit { numval } = right {
               return Ok(token::Literals::NumLit { numval: -numval });  
           }
           else {
            self.error_exit();
           }                  
        }
        _=>{
            return Ok(token::Literals::Nil);
        }
           
       } 
      
    }
    
    fn visit_binary(&mut self, visitor: &expr::Binary) -> Result<token::Literals,Exit> {
       
        let right:token::Literals=self.evaluate(&visitor.right)?;
        let left:token::Literals=self.evaluate(&visitor.left)?;
        
        match visitor.operator.token_type {
            token::TokenType::Minus=>{
                self.check_number_operands_(visitor.operator.clone(),left.clone(),right.clone())?;
                if let token::Literals::NumLit { numval:x }=left{
                   if let token::Literals::NumLit { numval:y }=right{
                        return Ok(token::Literals::NumLit { numval: x-y });  
                    }
                    else {
                        self.error_exit();
                       }
                }
                else {
                    self.error_exit();
                    }
                
            },
            token::TokenType::Slash=>{
                self.check_number_operands_(visitor.operator.clone(),left.clone(),right.clone())?;
                if let token::Literals::NumLit { numval:x }=left{
                    if let token::Literals::NumLit { numval:y }=right{
                         return Ok(token::Literals::NumLit { numval: x/y });  
                     }
                     else {
                        self.error_exit();
                        }
                 }
                 else {
                    self.error_exit();
                     }
                 
             },             
            
            token::TokenType::Star=>{
                self.check_number_operands_(visitor.operator.clone(),left.clone(),right.clone())?;
                if let token::Literals::NumLit { numval:x }=left{
                    if let token::Literals::NumLit { numval:y }=right{
                         return Ok(token::Literals::NumLit { numval: x*y });  
                     }else {
                        self.error_exit();
                        }
                 }else {
                    self.error_exit();
                     }                 
             },          
            token::TokenType::Plus=>{
                
                if let token::Literals::NumLit { numval:x }=left{
                    if let token::Literals::NumLit { numval:y }=right{
                         return Ok(token::Literals::NumLit { numval: x+y });  
                     }else {
                        return Err(Exit::RuntimeErr(
                            RuntimeError{tok:visitor.operator.clone(),mess:"Operand must be two numbers or two strings".to_string()}));
                        
                        }
                 }else if let token::Literals::StringLit { stringval:x }=left{
                        if let token::Literals::StringLit { stringval:y }=right{
                            return Ok(token::Literals::StringLit { stringval: x+&y});
                        }else {
                            return Err(Exit::RuntimeErr(
                                RuntimeError{tok:visitor.operator.clone(),mess:"Operand must be two numbers or two strings".to_string()})
                            );
                            
                           }
                    }else {
                        return Err(Exit::RuntimeErr(
                            RuntimeError{tok:visitor.operator.clone(),mess:"Operand must be two numbers or two strings".to_string()})
                        );
                        
                       }
                
                
            },
            token::TokenType::Greater=>{
                self.check_number_operands_(visitor.operator.clone(),left.clone(),right.clone())?;
                if let token::Literals::NumLit { numval:x }=left{
                    if let token::Literals::NumLit { numval:y }=right{
                         return Ok(token::Literals::BooleanLit { boolval: (x>y) }) ;  
                     }else {
                        self.error_exit();
                        }
                 }else {
                    self.error_exit();
                   }
            }
            token::TokenType::GreaterEqual=>{
                self.check_number_operands_(visitor.operator.clone(),left.clone(),right.clone())?;
                if let token::Literals::NumLit { numval:x }=left{
                if let token::Literals::NumLit { numval:y }=right{
                     return Ok(token::Literals::BooleanLit { boolval: (x>=y) }) ;  
                 }else {
                    self.error_exit();
                    }
             }else {
                self.error_exit();
               }}
            token::TokenType::Lesser=>{
                self.check_number_operands_(visitor.operator.clone(),left.clone(),right.clone())?;
                if let token::Literals::NumLit { numval:x }=left{
                if let token::Literals::NumLit { numval:y }=right{
                     return Ok(token::Literals::BooleanLit { boolval: (x<y) }) ;  
                 }else {
                    self.error_exit();
                    }
             }else {
                self.error_exit();
               }}
            token::TokenType::LesserEqual=>{
                self.check_number_operands_(visitor.operator.clone(),left.clone(),right.clone())?;
                if let token::Literals::NumLit { numval:x }=left{
                if let token::Literals::NumLit { numval:y }=right{
                     return Ok(token::Literals::BooleanLit { boolval: (x<=y) }) ;  
                 }else {
                    self.error_exit();
                    }
             }else {
                self.error_exit();
               }}
            token::TokenType::BangEqual=>{ return Ok(token::Literals::BooleanLit { boolval: !self.is_equal(&left, &right) })}
            token::TokenType::EqualEqual=>{return Ok(token::Literals::BooleanLit { boolval: self.is_equal(&left, &right) })}
            _=>{
                return Ok(token::Literals::Nil);
            }
        }

    }
    fn visit_variable(&mut self, visitor: &expr::Variable) -> Result<token::Literals,Exit>  {
        self.look_up_variable(visitor.name.clone(),expr::Expr::Variable(visitor.clone()))
        
    }

    fn visit_assign(&mut self, visitor: &expr::Assign) ->Result<token::Literals,Exit> {
        
        let value = self.evaluate(&visitor.value)?;
        let distance = self.locals.get(&expr::Expr::Assign(visitor.clone()));
        if distance.is_some(){
            self.environment.borrow_mut().assign_at(distance.unwrap().clone(),visitor.name.clone(),value.clone());
        }
        else{
            self.environment.borrow_mut().assign(&visitor.name,value.clone())?;
        }
        
        return Ok(value);
    }
    
}            



