(function() {var implementors = {
"either":[["impl&lt;L, R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"enum\" href=\"either/enum.Either.html\" title=\"enum either::Either\">Either</a>&lt;L, R&gt;<div class=\"where\">where\n    L: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a>,\n    R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a>&lt;Item = L::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>&gt;,</div>"],["impl&lt;L, R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"either/struct.IterEither.html\" title=\"struct either::IterEither\">IterEither</a>&lt;L, R&gt;<div class=\"where\">where\n    L: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a>,\n    R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a>,</div>"]],
"itertools":[["impl&lt;A&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"itertools/structs/struct.RepeatN.html\" title=\"struct itertools::structs::RepeatN\">RepeatN</a>&lt;A&gt;<div class=\"where\">where\n    A: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</div>"],["impl&lt;I&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"itertools/structs/struct.RcIter.html\" title=\"struct itertools::structs::RcIter\">RcIter</a>&lt;I&gt;<div class=\"where\">where\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a>,</div>"],["impl&lt;I, F&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"itertools/structs/struct.PadUsing.html\" title=\"struct itertools::structs::PadUsing\">PadUsing</a>&lt;I, F&gt;<div class=\"where\">where\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/exact_size/trait.ExactSizeIterator.html\" title=\"trait core::iter::traits::exact_size::ExactSizeIterator\">ExactSizeIterator</a>,\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.usize.html\">usize</a>) -&gt; I::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>,</div>"],["impl&lt;I, J&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"itertools/structs/struct.Flatten.html\" title=\"struct itertools::structs::Flatten\">Flatten</a>&lt;I, J&gt;<div class=\"where\">where\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a>,\n    I::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/collect/trait.IntoIterator.html\" title=\"trait core::iter::traits::collect::IntoIterator\">IntoIterator</a>&lt;IntoIter = J, Item = J::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\" title=\"type core::iter::traits::iterator::Iterator::Item\">Item</a>&gt;,\n    J: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a>,</div>"],["impl&lt;T, U&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"itertools/structs/struct.ZipLongest.html\" title=\"struct itertools::structs::ZipLongest\">ZipLongest</a>&lt;T, U&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/exact_size/trait.ExactSizeIterator.html\" title=\"trait core::iter::traits::exact_size::ExactSizeIterator\">ExactSizeIterator</a>,\n    U: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/exact_size/trait.ExactSizeIterator.html\" title=\"trait core::iter::traits::exact_size::ExactSizeIterator\">ExactSizeIterator</a>,</div>"],["impl&lt;X, Iter, B, C, D, E, F, G, H&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"itertools/structs/struct.ConsTuples.html\" title=\"struct itertools::structs::ConsTuples\">ConsTuples</a>&lt;Iter, (<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.tuple.html\">(B, C, D, E, F, G, H)</a>, X)&gt;<div class=\"where\">where\n    Iter: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a>&lt;Item = (<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.tuple.html\">(B, C, D, E, F, G, H)</a>, X)&gt;,</div>"],["impl&lt;X, Iter, C, D, E, F, G, H&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"itertools/structs/struct.ConsTuples.html\" title=\"struct itertools::structs::ConsTuples\">ConsTuples</a>&lt;Iter, (<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.tuple.html\">(C, D, E, F, G, H)</a>, X)&gt;<div class=\"where\">where\n    Iter: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a>&lt;Item = (<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.tuple.html\">(C, D, E, F, G, H)</a>, X)&gt;,</div>"],["impl&lt;X, Iter, D, E, F, G, H&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"itertools/structs/struct.ConsTuples.html\" title=\"struct itertools::structs::ConsTuples\">ConsTuples</a>&lt;Iter, (<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.tuple.html\">(D, E, F, G, H)</a>, X)&gt;<div class=\"where\">where\n    Iter: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a>&lt;Item = (<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.tuple.html\">(D, E, F, G, H)</a>, X)&gt;,</div>"],["impl&lt;X, Iter, E, F, G, H&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"itertools/structs/struct.ConsTuples.html\" title=\"struct itertools::structs::ConsTuples\">ConsTuples</a>&lt;Iter, (<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.tuple.html\">(E, F, G, H)</a>, X)&gt;<div class=\"where\">where\n    Iter: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a>&lt;Item = (<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.tuple.html\">(E, F, G, H)</a>, X)&gt;,</div>"],["impl&lt;X, Iter, F, G, H&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"itertools/structs/struct.ConsTuples.html\" title=\"struct itertools::structs::ConsTuples\">ConsTuples</a>&lt;Iter, (<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.tuple.html\">(F, G, H)</a>, X)&gt;<div class=\"where\">where\n    Iter: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a>&lt;Item = (<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.tuple.html\">(F, G, H)</a>, X)&gt;,</div>"],["impl&lt;X, Iter, G, H&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"itertools/structs/struct.ConsTuples.html\" title=\"struct itertools::structs::ConsTuples\">ConsTuples</a>&lt;Iter, (<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.tuple.html\">(G, H)</a>, X)&gt;<div class=\"where\">where\n    Iter: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a>&lt;Item = (<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.tuple.html\">(G, H)</a>, X)&gt;,</div>"]],
"syn":[["impl&lt;'a, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"syn/punctuated/struct.Iter.html\" title=\"struct syn::punctuated::Iter\">Iter</a>&lt;'a, T&gt;"],["impl&lt;'a, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"syn/punctuated/struct.IterMut.html\" title=\"struct syn::punctuated::IterMut\">IterMut</a>&lt;'a, T&gt;"],["impl&lt;'a, T, P&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"syn/punctuated/struct.Pairs.html\" title=\"struct syn::punctuated::Pairs\">Pairs</a>&lt;'a, T, P&gt;"],["impl&lt;'a, T, P&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"syn/punctuated/struct.PairsMut.html\" title=\"struct syn::punctuated::PairsMut\">PairsMut</a>&lt;'a, T, P&gt;"],["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"syn/punctuated/struct.IntoIter.html\" title=\"struct syn::punctuated::IntoIter\">IntoIter</a>&lt;T&gt;"],["impl&lt;T, P&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"syn/punctuated/struct.IntoPairs.html\" title=\"struct syn::punctuated::IntoPairs\">IntoPairs</a>&lt;T, P&gt;"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()