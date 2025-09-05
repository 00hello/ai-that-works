class Response {
    res string
}


function GetResponse(my_prompt: string, my_input: string) -> Response {  
    client CustomGPT4o

    prompt #"
        {{ my_prompt }}
        {{ my_input }}

        {{ ctx.output_format }}
    "#
}