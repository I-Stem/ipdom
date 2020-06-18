# IpDom
Index pointed Document Object Model

Used to compactly store a DOM and support quick navigation instead of using and following pointers in a rcDOM;

### Structs
+ #### IpDom 
  Contains nodes that represents different nodes in the DOM 
  Each node has:

  ````
    index: usize -> position in nodes array 
    next: Option<usize> -> pointer to next 
    prev: Option<usize> -> pointer to the prev node 
    parent: Option<usize>   -> pointer to the parent node 
    first_child: Option<usize>   -> pointer to the first child 
    last_child: Option<usize>   -> pointer to the last child 
    data: NodeData    -> pointer to the data node
  ```` 


### Usage
Import the library by adding this in your `Cargo.toml` file

```
    [dependencies]
    ipdom = { git = "https://github.com/shaddyshad/ipdom"}
    
```

In your source file 
```
    [main.rs]

    extern crate ipdom;

    use ipdom::{IpDom, parse_file};

    fn main(){
        let filepath = "/path/to/file.xml";

        let dom = parse_file(filepath).unwrap();  // IpDom structure
    }

```