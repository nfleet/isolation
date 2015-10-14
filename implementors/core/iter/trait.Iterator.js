(function() {var implementors = {};
implementors['log'] = ["impl&lt;I&gt; <a class='trait' href='http://doc.rust-lang.org/nightly/core/iter/trait.Iterator.html' title='core::iter::Iterator'>Iterator</a> for <a class='struct' href='http://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html' title='alloc::boxed::Box'>Box</a>&lt;I&gt; <span class='where'>where I: <a class='trait' href='http://doc.rust-lang.org/nightly/core/iter/trait.Iterator.html' title='core::iter::Iterator'>Iterator</a> + ?<a class='trait' href='http://doc.rust-lang.org/nightly/core/marker/trait.Sized.html' title='core::marker::Sized'>Sized</a></span>",];implementors['petgraph'] = ["impl&lt;'a, N&gt; <a class='trait' href='http://doc.rust-lang.org/nightly/core/iter/trait.Iterator.html' title='core::iter::Iterator'>Iterator</a> for <a class='struct' href='petgraph/graphmap/struct.Nodes.html' title='petgraph::graphmap::Nodes'>Nodes</a>&lt;'a, N&gt;","impl&lt;'a, N&gt; <a class='trait' href='http://doc.rust-lang.org/nightly/core/iter/trait.Iterator.html' title='core::iter::Iterator'>Iterator</a> for <a class='struct' href='petgraph/graphmap/struct.Neighbors.html' title='petgraph::graphmap::Neighbors'>Neighbors</a>&lt;'a, N&gt;","impl&lt;'a, N, E&gt; <a class='trait' href='http://doc.rust-lang.org/nightly/core/iter/trait.Iterator.html' title='core::iter::Iterator'>Iterator</a> for <a class='struct' href='petgraph/graphmap/struct.Edges.html' title='petgraph::graphmap::Edges'>Edges</a>&lt;'a, N, E&gt; <span class='where'>where N: 'a + <a class='trait' href='petgraph/graphmap/trait.NodeTrait.html' title='petgraph::graphmap::NodeTrait'>NodeTrait</a>, E: 'a</span>","impl&lt;'a, N: 'a, Ty, Ix&gt; <a class='trait' href='http://doc.rust-lang.org/nightly/core/iter/trait.Iterator.html' title='core::iter::Iterator'>Iterator</a> for <a class='struct' href='petgraph/graph/struct.WithoutEdges.html' title='petgraph::graph::WithoutEdges'>WithoutEdges</a>&lt;'a, N, Ty, Ix&gt; <span class='where'>where Ty: <a class='trait' href='petgraph/trait.EdgeType.html' title='petgraph::EdgeType'>EdgeType</a>, Ix: <a class='trait' href='petgraph/graph/trait.IndexType.html' title='petgraph::graph::IndexType'>IndexType</a></span>","impl&lt;'a, E, Ix&gt; <a class='trait' href='http://doc.rust-lang.org/nightly/core/iter/trait.Iterator.html' title='core::iter::Iterator'>Iterator</a> for <a class='struct' href='petgraph/graph/struct.Neighbors.html' title='petgraph::graph::Neighbors'>Neighbors</a>&lt;'a, E, Ix&gt; <span class='where'>where Ix: <a class='trait' href='petgraph/graph/trait.IndexType.html' title='petgraph::graph::IndexType'>IndexType</a></span>","impl&lt;'a, E, Ix&gt; <a class='trait' href='http://doc.rust-lang.org/nightly/core/iter/trait.Iterator.html' title='core::iter::Iterator'>Iterator</a> for <a class='struct' href='petgraph/graph/struct.Edges.html' title='petgraph::graph::Edges'>Edges</a>&lt;'a, E, Ix&gt; <span class='where'>where Ix: <a class='trait' href='petgraph/graph/trait.IndexType.html' title='petgraph::graph::IndexType'>IndexType</a></span>","impl&lt;'a, N, Ix&gt; <a class='trait' href='http://doc.rust-lang.org/nightly/core/iter/trait.Iterator.html' title='core::iter::Iterator'>Iterator</a> for <a class='struct' href='petgraph/graph/struct.NodeWeightsMut.html' title='petgraph::graph::NodeWeightsMut'>NodeWeightsMut</a>&lt;'a, N, Ix&gt; <span class='where'>where Ix: <a class='trait' href='petgraph/graph/trait.IndexType.html' title='petgraph::graph::IndexType'>IndexType</a></span>","impl&lt;'a, E, Ix&gt; <a class='trait' href='http://doc.rust-lang.org/nightly/core/iter/trait.Iterator.html' title='core::iter::Iterator'>Iterator</a> for <a class='struct' href='petgraph/graph/struct.EdgeWeightsMut.html' title='petgraph::graph::EdgeWeightsMut'>EdgeWeightsMut</a>&lt;'a, E, Ix&gt; <span class='where'>where Ix: <a class='trait' href='petgraph/graph/trait.IndexType.html' title='petgraph::graph::IndexType'>IndexType</a></span>","impl&lt;'a, G: 'a + <a class='trait' href='petgraph/visit/trait.Visitable.html' title='petgraph::visit::Visitable'>Visitable</a>&gt; <a class='trait' href='http://doc.rust-lang.org/nightly/core/iter/trait.Iterator.html' title='core::iter::Iterator'>Iterator</a> for <a class='struct' href='petgraph/visit/struct.DfsIter.html' title='petgraph::visit::DfsIter'>DfsIter</a>&lt;'a, G&gt; <span class='where'>where G: <a class='trait' href='petgraph/visit/trait.NeighborIter.html' title='petgraph::visit::NeighborIter'>NeighborIter</a>&lt;'a&gt;</span>","impl&lt;'a, G: 'a + <a class='trait' href='petgraph/visit/trait.Visitable.html' title='petgraph::visit::Visitable'>Visitable</a>&gt; <a class='trait' href='http://doc.rust-lang.org/nightly/core/iter/trait.Iterator.html' title='core::iter::Iterator'>Iterator</a> for <a class='struct' href='petgraph/visit/struct.BfsIter.html' title='petgraph::visit::BfsIter'>BfsIter</a>&lt;'a, G&gt; <span class='where'>where G::NodeId: <a class='trait' href='http://doc.rust-lang.org/nightly/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a>, G: <a class='trait' href='petgraph/visit/trait.NeighborIter.html' title='petgraph::visit::NeighborIter'>NeighborIter</a>&lt;'a&gt;</span>",];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
