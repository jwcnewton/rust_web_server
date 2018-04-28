pub struct HttpReq {
   pub uri: String,
   pub verb: String,
   pub res_type: String
}

pub fn str_to_request(req:String)->HttpReq {
    let request_lf = req.lines().nth(0).unwrap().to_string();
    return HttpReq { 
        uri: get_request_line_uri(&request_lf),
        verb: get_request_line_verb(&request_lf),
        res_type: get_request_line_type(&req.to_string())
    };
}

fn get_request_line_verb(request_string: &String)->String {
    return request_string.split_whitespace().nth(0).unwrap().trim().to_string();
}

fn get_request_line_uri(request_string: &String)->String {
    return request_string.split_whitespace().nth(1).unwrap().trim().to_string();
}

fn get_request_line_type(request_string: &String)->String {
    let accept_index = request_string
        .find("Accept:")
        .unwrap_or(0);
    let accept_sub: String = request_string.chars()
        .skip(accept_index)
        .take(request_string.len())
        .collect();
    let accept_line = accept_sub.lines().nth(0).unwrap().to_string();
    let accept_val = accept_line.split_whitespace().nth(1).unwrap();
    return accept_val.trim().to_string();
}