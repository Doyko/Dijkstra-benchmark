#[cfg(test)]
mod tests {
    use crate::add_nodes;
    
    #[test]
    fn test_add_nodes() {
        let mut graph: Vec<Vec<u32>> = Vec::new();
        add_nodes(&mut graph, 3);
        assert_eq!(graph.len(), 3);
        add_nodes(&mut graph, 6);
        assert_eq!(graph.len(), 9);
    }
}
