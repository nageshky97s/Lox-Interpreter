use super::{expr::Accept, token,lox,expr,stmt,stmt::StmtAccept,environment};
use std::panic::AssertUnwindSafe;

pub struct RuntimeError{
    pub tok:token::Token,
    pub mess:String
}

pub struct Interpreter{
   environment:environment::Environment
}

impl Interpreter {
    pub fn new()->Self{
        Interpreter{environment:environment::Environment::new()}
    }
    fn evaluate(&mut self,exp:&expr::Expr)->token::Literals{
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
fn check_number_operands(&mut self,tok:token::Token,operand:token::Literals){
    if let token::Literals::NumLit { numval:_ } =operand{
        return ;
    }else{
        std::panic::panic_any(RuntimeError{tok:tok.clone(),mess:"Operand must be a number".to_string()});
    }
}
fn check_number_operands_(&mut self,tok:token::Token,left:token::Literals,right:token::Literals){
    if let token::Literals::NumLit { numval:_} =left{
        if let token::Literals::NumLit { numval:_ } =right {
            return;
        }else{
            std::panic::panic_any(RuntimeError{tok:tok.clone(),mess:"Operand must be two numbers or two strings".to_string()});
        }
    }else{
        std::panic::panic_any(RuntimeError{tok:tok.clone(),mess:"Operand must be two numbers or two strings".to_string()});
    }
}

// pub fn interpret(&mut self,exp:expr::Expr,lox_obj:&mut lox::Lox){
//     let res = std::panic::catch_unwind(AssertUnwindSafe(|| self.evaluate(&exp)));
//     match res {
//         Ok(x)=>{
//             println!("{}",self.stringify(x));
//         } 
//         Err(payload)if payload.is::<RuntimeError>()=>{
//             println!("Runtime Error");
//             lox_obj.runtimeerror(*payload.downcast().expect("The value must be of type RuntimeError"));
//         },
//         Err(payload) => std::panic::resume_unwind(payload),
        
//     }
// }
pub fn interpret_new(&mut self, statements:Vec< stmt::Stmt>,lox_obj:&mut lox::Lox){
    let res = std::panic::catch_unwind(AssertUnwindSafe(|| {
        for statement in statements.iter(){
            self.execute(&statement);
        }
    }));
    match res {
        Ok(_x)=>{
            return
        } 
        Err(payload)if payload.is::<RuntimeError>()=>{
            println!("Runtime Error");
            lox_obj.runtimeerror(*payload.downcast().expect("The value must be of type RuntimeError"));
        },
        Err(payload) => std::panic::resume_unwind(payload),
        
    }
}
fn execute(&mut self,stm:&stmt::Stmt){
    return stm.accept(self);
}

fn stringify(&mut self,value:token::Literals)->String{
    match value {
        token::Literals::BooleanLit { boolval }=>{return boolval.to_string() ;}
        token::Literals::NumLit { numval }=>{return numval.to_string();}
        token::Literals::Nil =>{return "nil".to_string();}
        token::Literals::StringLit { stringval }=>{return stringval;}
    }
}

}

impl stmt::StmtVisitor<()> for Interpreter{

    fn visit_expression_stmt(&mut self, stm: &stmt::Expression) -> () {
        self.evaluate(&stm.expression);
    }
    fn visit_print_stmt(&mut self, stm: &stmt::Print) -> () {
        let value=self.evaluate(&stm.expression);
        println!("{}",self.stringify(value));
    }

    fn visit_var_stmt(&mut self, stm: &stmt::Var) -> () {
        let mut value =token::Literals::Nil;
        if stm.initializer!=expr::Expr::Literal(Box::new(expr::Literal{value:token::Literals::Nil})){
            value=self.evaluate(&stm.initializer);
        }
        self.environment.define(stm.name.lexeme.clone(), value);
    }

}


impl expr::AstVisitor<token::Literals> for Interpreter{
    fn visit_literal(&mut self, visitor: &expr::Literal) -> token::Literals{
        return visitor.value.clone()
    }
    fn visit_grouping(&mut self, visitor: &expr::Grouping) -> token::Literals{
        return self.evaluate(&*visitor.expression)
    }
    fn visit_unary(&mut self, visitor: &expr::Unary) -> token::Literals{
        let right:token::Literals=self.evaluate(&visitor.right);
       match visitor.operator.token_type  {
        token::TokenType::Bang => {
            
            if !self.is_truthy(right){
                return token::Literals::BooleanLit{boolval:true} ;
            }else{
                return token::Literals::BooleanLit{boolval:false};
            }
        }
        token::TokenType::Minus=>{

        self.check_number_operands(visitor.operator.clone(),right.clone());
           if let token::Literals::NumLit { numval } = right {
               return token::Literals::NumLit { numval: -numval };  
           }
           else {
            self.error_exit();
           }                  
        }
        _=>{
            return token::Literals::Nil;
        }
           
       } 
      
    }
    
    fn visit_binary(&mut self, visitor: &expr::Binary) -> token::Literals{
        let right:token::Literals=self.evaluate(&visitor.right);
        let left:token::Literals=self.evaluate(&visitor.left);
        match visitor.operator.token_type {
            token::TokenType::Minus=>{
                self.check_number_operands_(visitor.operator.clone(),left.clone(),right.clone());
                if let token::Literals::NumLit { numval:x }=left{
                   if let token::Literals::NumLit { numval:y }=right{
                        return token::Literals::NumLit { numval: x-y };  
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
                self.check_number_operands_(visitor.operator.clone(),left.clone(),right.clone());
                if let token::Literals::NumLit { numval:x }=left{
                    if let token::Literals::NumLit { numval:y }=right{
                         return token::Literals::NumLit { numval: x/y };  
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
                self.check_number_operands_(visitor.operator.clone(),left.clone(),right.clone());
                if let token::Literals::NumLit { numval:x }=left{
                    if let token::Literals::NumLit { numval:y }=right{
                         return token::Literals::NumLit { numval: x*y };  
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
                         return token::Literals::NumLit { numval: x+y };  
                     }else {
                        std::panic::panic_any(RuntimeError{tok:visitor.operator.clone(),mess:"Operand must be two numbers or two strings".to_string()});
                        }
                 }else if let token::Literals::StringLit { stringval:x }=left{
                        if let token::Literals::StringLit { stringval:y }=right{
                            return token::Literals::StringLit { stringval: x+&y};
                        }else {
                            std::panic::panic_any(RuntimeError{tok:visitor.operator.clone(),mess:"Operand must be two numbers or two strings".to_string()});
                           }
                    }else {
                        std::panic::panic_any(RuntimeError{tok:visitor.operator.clone(),mess:"Operand must be two numbers or two strings".to_string()});
                       }
                
                
            },
            token::TokenType::Greater=>{
                self.check_number_operands_(visitor.operator.clone(),left.clone(),right.clone());
                if let token::Literals::NumLit { numval:x }=left{
                    if let token::Literals::NumLit { numval:y }=right{
                         return token::Literals::BooleanLit { boolval: (x>y) } ;  
                     }else {
                        self.error_exit();
                        }
                 }else {
                    self.error_exit();
                   }
            }
            token::TokenType::GreaterEqual=>{
                self.check_number_operands_(visitor.operator.clone(),left.clone(),right.clone());
                if let token::Literals::NumLit { numval:x }=left{
                if let token::Literals::NumLit { numval:y }=right{
                     return token::Literals::BooleanLit { boolval: (x>=y) } ;  
                 }else {
                    self.error_exit();
                    }
             }else {
                self.error_exit();
               }}
            token::TokenType::Lesser=>{
                self.check_number_operands_(visitor.operator.clone(),left.clone(),right.clone());
                if let token::Literals::NumLit { numval:x }=left{
                if let token::Literals::NumLit { numval:y }=right{
                     return token::Literals::BooleanLit { boolval: (x<y) } ;  
                 }else {
                    self.error_exit();
                    }
             }else {
                self.error_exit();
               }}
            token::TokenType::LesserEqual=>{
                self.check_number_operands_(visitor.operator.clone(),left.clone(),right.clone());
                if let token::Literals::NumLit { numval:x }=left{
                if let token::Literals::NumLit { numval:y }=right{
                     return token::Literals::BooleanLit { boolval: (x<=y) } ;  
                 }else {
                    self.error_exit();
                    }
             }else {
                self.error_exit();
               }}
            token::TokenType::BangEqual=>{ return token::Literals::BooleanLit { boolval: !self.is_equal(&left, &right) }}
            token::TokenType::EqualEqual=>{return token::Literals::BooleanLit { boolval: self.is_equal(&left, &right) }}
            _=>{
                return token::Literals::Nil;
            }
        }

    }
    fn visit_variable(&mut self, visitor: &expr::Variable) -> token::Literals {
        self.environment.get(visitor.name.clone())
    }

    fn visit_assign(&mut self, visitor: &expr::Assign) -> token::Literals {
        let value = self.evaluate(&visitor.value);
        self.environment.assign(visitor.name.clone(),value.clone());
        return value;
    }
    
}            



