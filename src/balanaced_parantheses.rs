use std::collections::HashMap;

struct Response{
    open_expression:Vec<char>,
    continue_parse:bool,

}

impl Response{

    fn new(open_expression:Vec<char>)->Response{
        Response{
            open_expression,
            continue_parse:true,
        }
    }
    fn is_valid(self)->bool{
        return self.open_expression.is_empty() && self.continue_parse;
    }


    fn handle_new_character(self, input:char) ->Response{

        let mut cloned_response = self;

        if is_open_expression(input){
            cloned_response.open_expression.push(input);
        }
        if is_closed_expression(input){
            let  last_opened_expression = cloned_response.open_expression.last().unwrap().clone();
            cloned_response.continue_parse = is_matching_open_and_close(last_opened_expression, input);
            cloned_response.open_expression.pop();


        }
        return cloned_response;

    }

}

pub fn is_balanced(expression: &str) ->bool{



    let  default_value=Response::new(Vec::new());

    return  expression.chars().enumerate()
                    .fold(default_value,
                          |accum,(_ ,c)| {
                                          if accum.continue_parse == false { accum } else { accum.handle_new_character(c) }
                                })
                    .is_valid();


}

fn is_matching_open_and_close(open:char, close: char) ->bool{

    let mut valid_expressions = HashMap::new();
    valid_expressions.insert('(', ')');
    valid_expressions.insert('{', '}');
    valid_expressions.insert('[', ']');

    valid_expressions.get(&open)
        .filter(|&x| *x == close)
        .map_or(false,|_| true)



}


fn is_open_expression(inp: char)->bool{
    String::from("({[").contains(inp)

}
fn is_closed_expression(inp: char)->bool{
    String::from(")}]").contains(inp)

}

#[cfg(test)]
mod tests {
    use crate::balanaced_parantheses::*;

    #[test]
    fn it_works() {
        assert!(is_balanced("()"));
        assert!(is_balanced("([]{})"));
        assert_eq!(is_balanced("([{})"), false);
        let shader = r#"
    #version 330

    in vec4 v_color;
    out vec4 color;

    void main() {
        color = v_color;
    };
"#;

        assert_eq!(is_balanced(shader), true);

    }
    #[test]
    fn is_open_expression_should_work_properly() {


        let open_expressions = String::from("({[");

        for (_, c) in open_expressions.chars().enumerate() {
            assert_eq!(is_open_expression(c),true);
        }
        assert_eq!(is_open_expression(')'),false);
    }

    #[test]
    fn is_valid_close_should_work_appropriately_for_valid_values() {

           assert!(is_matching_open_and_close('(', ')'));
           assert!(is_matching_open_and_close('[', ']'));
           assert!(is_matching_open_and_close('{', '}'));
           assert_eq!(is_matching_open_and_close('(', '}'), false);


    }


    #[test]
    fn is_closed_expression_should_work_properly() {


        let open_expressions = String::from(")]}");

        for (_, c) in open_expressions.chars().enumerate() {
            assert_eq!(is_closed_expression(c),true);
        }



        assert_eq!(is_open_expression('2'),false);



    }
}
