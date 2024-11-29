use calculator::ast::{ASTree, Node, NodeID};




static test_formula : &str = "1+4 + pow(2, 2) + x1 * x2";
static test_formula2: &str = "1+4+5+1+4+3";
static test_formula3 : &str = "1+4-3+5-2-11"; // -4
static test_formula4 : &str = "1+((4-3+5-2)-11) + 5"; // -1
static test_formula5 : &str = "((-110.95*2 + -40) - (0.05 - 4*0.5))/3 + 333 * 2 - 400 / 3"; //119




fn main() {

    let ts = parse_formula_to_tokenstream(test_formula).unwrap();
    println!("new parser to tokenstream: {} : {:?}", test_formula, ts);
    let ast = parse_tokenstream_to_ast(ts).unwrap();
    println!("new parser to ast: {} : {:?}", test_formula, ast);
    let res = calc_ast_result(&ast);
    println!("new parser to calc: {} = {:?}", test_formula, res);

    let ts = parse_formula_to_tokenstream(test_formula5).unwrap();
    println!("new parser to tokenstream: {} : {:?}", test_formula5, ts);
    let ast = parse_tokenstream_to_ast(ts).unwrap();
    println!("new parser to ast: {} : {:?}", test_formula5, ast);
    let res = calc_ast_result(&ast);
    println!("new parser to calc: {} = {:?}", test_formula5, res);
}

#[derive(Debug, Clone)]
enum MathToken{
    Number(f64),
    Operator(Operator),
    Operand(Operand),
    RndBrackOpen,
    RndBrackClose,
    FnArgSeparator,
}
#[derive(Debug, Clone)]
enum Operator{
    Plus,
    Minus,
    Multiply,
    Divide,
}
#[derive(Debug, Clone)]
enum Operand{
    Number(f64),
    Variable(String),
    Function(String),
}


enum ParseState{
    Number2{full_num: String, minus: bool, dot: bool},
    VariableOrFunction{name: String},
    Function{name: String},
    None,
}

fn parse_formula_to_tokenstream(formula : &str) -> Result<Vec<MathToken>, String> {
    let mut tokenstream : Vec<MathToken> = Vec::new();
    let mut parse_state = ParseState::None;
    let formula_utf8_len : usize = formula.chars().count();
    let mut formula_iter = formula.chars().enumerate();

    while let Some((i, c)) = formula_iter.next(){
        //processing while having some state
        match &mut parse_state{
            ParseState::Number2 { full_num, minus, dot } => {
                if let '0'..='9' | '.' = c{
                }
                else {
                    tokenstream.push(MathToken::Number(full_num.parse().unwrap()));
                    parse_state = ParseState::None;
                }
            },
            ParseState::VariableOrFunction { name } => {
                match c{
                    'a'..='z' | '0'..='9' => {
                        name.push(c);
                        if formula_utf8_len-1 <= i{
                            tokenstream.push(MathToken::Operand(Operand::Variable(name.clone())));
                            parse_state = ParseState::None;
                        }
                        continue;
                    },
                    '(' => {
                        tokenstream.push(MathToken::Operand(Operand::Function(name.clone())));
                        tokenstream.push(MathToken::RndBrackOpen);
                        parse_state = ParseState::None;
                        //parse_state = ParseState::Function { name: name.clone() };
                        continue;
                    },
                    _ => {
                        tokenstream.push(MathToken::Operand(Operand::Variable(name.clone())));
                        parse_state = ParseState::None;
                    },
                }
            },
            ParseState::Function { name } => {
            }
            _ => {},
        }



        //processing without state (e.g. None) --nah maybe not

        match c {
            '0'..='9' | '.'  => {
                let t_dot : bool = c == '.';
                if let ParseState::Number2 { full_num, minus, dot } = &mut parse_state{
                    if *dot && t_dot{
                        return Err(String::from("number can't contain multiple '.' characters"));
                    }
                    else if t_dot{
                        *dot = true;
                    }

                    full_num.push(c);

                }
                else{
                    let mut full_num = String::new();
                    let num_is_minus : bool = if let Some(MathToken::Operator(Operator::Minus)) = tokenstream.last(){
                        let len = tokenstream.len();
                        if len > 1{
                            if let MathToken::Operator(_op) = &tokenstream[len-2]{
                                true
                            }
                            else{false}
                        }
                        else {true}
                    }
                    else {false};
                    if num_is_minus {
                        tokenstream.pop(); // remove last minus operator
                        full_num.push('-'); // because it ist now part of the operand/number
                    }
                    full_num.push(c);
                    if formula_utf8_len-1 > i{
                        parse_state = ParseState::Number2{full_num, minus: num_is_minus, dot: t_dot};
                    }
                    else {
                        tokenstream.push(MathToken::Number(full_num.parse().unwrap()));
                    }
                }
            },
            'a'..='z' => {
                let name = String::from(c);
                parse_state = ParseState::VariableOrFunction { name };
            },
            ',' => {
                tokenstream.push(MathToken::FnArgSeparator);
            },
            '+' => {
                tokenstream.push(MathToken::Operator(Operator::Plus));
            },
            '-' => {
                tokenstream.push(MathToken::Operator(Operator::Minus));
            },
            '*' => {
                tokenstream.push(MathToken::Operator(Operator::Multiply));
            },
            '/' => {
                tokenstream.push(MathToken::Operator(Operator::Divide));
            },


            '(' => {tokenstream.push(MathToken::RndBrackOpen);},
            ')' => {tokenstream.push(MathToken::RndBrackClose);},

            _ => {},
        }
    }

    Ok(tokenstream)
}


fn parse_tokenstream_to_ast(tokenstream : Vec<MathToken>) -> Result<ASTree<MathToken>, String>{
    let mut ast : ASTree<MathToken> = ASTree::new();
    ast.node_children.push(Vec::new());
    let mut ast_stack : Vec<usize> = Vec::new();
    ast_stack.push(0);
    //let mut last_token_id : Option<NodeID>= None;
    for token in tokenstream.iter(){
        let last_list_id = *ast_stack.last().unwrap();
        match token{
            MathToken::Number(num) => {
                ast.add_node_to_list(
                    Node::new(MathToken::Number(*num), None, None), last_list_id)
                    .expect("couldn't add number to ast");
            },
            MathToken::Operand(op) => {
                match op{
                    Operand::Variable(name) => {},
                    Operand::Function(name) => {},
                    _ => {},
                }
            },
            MathToken::Operator(op) => {
                ast.add_node_to_list(
                    Node::new(MathToken::Operator(op.clone()), None, None), last_list_id)
                    .expect("couldn't add operator to ast");
            },
            MathToken::RndBrackOpen => {
                let children_list_id = ast.add_node_list(Vec::new());
                ast_stack.push(children_list_id);

                ast.add_node_to_list(
                    Node::new(MathToken::RndBrackOpen, None, Some(children_list_id as u32)), last_list_id)
                    .expect("couldn't add RndBrackOp to ast");
            },
            MathToken::RndBrackClose => {
                ast_stack.pop();
                let last_list_id = *ast_stack.last().unwrap();
                ast.add_node_to_list(Node::new(MathToken::RndBrackClose, None, None), last_list_id).unwrap();
            },
            _ => {},
        }
    }

    Ok(ast)
}

fn calc_ast_result(ast : &ASTree<MathToken>) -> f64{
    calc_ast_result_rec(ast, 0)
}
fn calc_ast_result_rec(ast : &ASTree<MathToken>, list_id : usize) -> f64{
    let mut result : f64 = 0.;
    //let mut ast_stack : Vec<usize> = Vec::new();
    //ast_stack.push(0);
    //

    let mut tokenlist : Vec<MathToken> = Vec::with_capacity(ast.node_children[list_id].len()); //contains recursive calced values

    let node_list = ast.get_node_list(list_id).unwrap();
        for node in node_list.iter(){
            match &node.value{
                MathToken::RndBrackOpen => {
                    if let Some(children_id) = node.children{
                        let inner_result = calc_ast_result_rec(ast, children_id as usize);
                        tokenlist.push(MathToken::Number(inner_result));
                    }
                },
                MathToken::RndBrackClose => {},
                token => {
                    tokenlist.push(token.clone());
                },
            }
        }

    //first calculate dot operators(*, /)

    let mut token_ind : usize = 1;

    while token_ind < tokenlist.len(){
        let token = &mut tokenlist[token_ind];
        match token{
            MathToken::Operator(op) => {
                match op {
                    Operator::Multiply => {
                        let start = token_ind-1;
                        let end = token_ind+1;
                        if let Some(MathToken::Number(factor1)) = tokenlist.get(start){
                            if let Some(MathToken::Number(factor2)) = tokenlist.get(end){
                                tokenlist[start] = MathToken::Number(factor1*factor2);
                                tokenlist.remove(end);
                                tokenlist.remove(token_ind);
                                token_ind = start;
                            }
                            else{
                                //TODO: error, found no operand after operator or found no token at
                                //all
                            }
                        }

                    },
                    Operator::Divide => {
                        let start = token_ind-1;
                        let end = token_ind+1;
                        if let Some(MathToken::Number(divident)) = tokenlist.get(start){
                            if let Some(MathToken::Number(divisor)) = tokenlist.get(end){
                                tokenlist[start] = MathToken::Number(divident/divisor);
                                tokenlist.remove(end);
                                tokenlist.remove(token_ind);
                                token_ind = start;
                            }
                            else{
                                //TODO: error, found no operand after operator or found no token at
                                //all
                            }
                        }
                    },
                    _ => {},

                }
            }
            _ => {},
        }

        token_ind += 1;
    }

    //calculation of +, - Operators in sequence

    let mut last_token : Option<&MathToken> = None;
    for token in tokenlist.iter(){
        match token {
            MathToken::Number(val) => {
                if let Some(last_token) = last_token {
                    if let MathToken::Operator(op) = last_token{
                        match op {
                            Operator::Plus => {
                                result += val;
                            },
                            Operator::Minus => {
                                result -= val;
                            },
                            _ => {},
                        }
                    }
                }
                else{
                    result += val;
                }
            },
            _ => {},
        }
        last_token = Some(token);
    }

    result
}


