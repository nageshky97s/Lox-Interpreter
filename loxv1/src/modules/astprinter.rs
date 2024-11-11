// use crate::modules::expr;
// use super::{expr::Accept, token};
// pub struct AstPrinter{
    
// }

// impl AstPrinter{
    
//     pub fn print(&mut self,e:&expr::Expr){
//        println!("{}",e.accept(self));
//     }
  
// }


// impl expr::AstVisitor<String> for AstPrinter{
//     fn visit_binary(&mut self, visitor: &expr::Binary) -> String{
//         let mut out = String::new();
//         out.push_str("(");
//         out.push_str(&visitor.operator.lexeme);
//         out.push_str(" ");
//         out.push_str(&visitor.left.accept(self));
//         out.push_str(" ");
//         out.push_str(&visitor.right.accept(self));
//         out.push_str(")");
//         out
       
//     }
//     fn visit_grouping(&mut self, visitor: &expr::Grouping) -> String{
//         let mut out = String::new();
//         out.push_str("(");
//         out.push_str("group");
//         out.push_str(" ");
//         out.push_str(visitor.expression.accept(self).as_str());
//         out.push_str(")");
//         out
//     }
//     fn visit_literal(&mut self, visitor: &expr::Literal) -> String{
        
//         match &visitor.value {
//             token::Literals::Nil=>{
//                return "nil".to_string()
//             },
//             token::Literals::NumLit{numval}=>{
//                 return numval.to_string();
//             },
//             token::Literals::StringLit{stringval}=>{
//                 return stringval.to_string();
//             }
//             token::Literals::BooleanLit{boolval}=>{
//                 return boolval.to_string();
//             }
            
//         }
        
    
        
//     }
//     fn visit_unary(&mut self, visitor: &expr::Unary) -> String{
//         let mut out = String::new();
//         out.push_str("(");
//         out.push_str(&visitor.operator.lexeme);
//         out.push_str(" ");
//         out.push_str(&visitor.right.accept(self));
//         out.push_str(")");
//         out
//     }
    
// } 
