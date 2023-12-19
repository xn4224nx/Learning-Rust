fn main() {
    let ctx_lines = 2;
    let needle = "oo";
    let haystack = "\
Every face, every shop,
bedroom window, public-house, and
dark square is a picture
feverishly turned--in search of what?
dark square is a picture
feverishly turned--in search of what?
It is the same with books.
What do we seek
through millions of pages?";
    
    /* Record the line number of matches. */
    let mut tags: Vec<usize> = vec![];
    
    /* Vector of each match to hold the context matches. */
    let mut ctx: Vec<Vec<(usize, String)>> = vec![];
    
    /* Iterate over the text and find the matches. */
    for (i, line) in haystack.lines().enumerate() {
        
        if line.contains(needle) {
            tags.push(i);
            
            let v = Vec::with_capacity(2*ctx_lines + 1);
            ctx.push(v);
        }
    }
    
    /* End the program if nothing has been found. */
    if tags.is_empty() {return;}
    
    for (i, line) in haystack.lines().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            
            let lower_bound = tag.saturating_sub(ctx_lines);
            let upper_bound = tag + ctx_lines;
            
            /* Makes a copy of the line and stores it. */
            if (i >= lower_bound) && (i <= upper_bound) {
                let line_as_string = String::from(line);
                let local_ctx = (i, line_as_string);
                ctx[j].push(local_ctx);
            }
        }
    }
    
    /* Print out the matches. */
    for local_ctx in ctx.iter() {
        for &(i, ref line) in local_ctx.iter() {
            println!("{}: {}", i + 1, line);
        }
    }
}
