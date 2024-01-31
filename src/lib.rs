
pub mod flow {
    
    // Base class for a flow
    // A flow is a set of operations
    trait Flow {
        // Add function to flow
        fn add(&self, name: String, func: &fn(i32));
        /// Visualize the flow via console
        fn visualize(&self);
        /// Serialize the flow to a file
        fn serialize(&self);
        /// Deserialize the flow from a file. 
        fn deserialize(&self, file_path: String);
    }

}

