(function() {
    var type_impls = Object.fromEntries([["dynasmrt",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-UncommittedModifier%3C'a%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/dynasmrt/lib.rs.html#1047\">Source</a><a href=\"#impl-Debug-for-UncommittedModifier%3C'a%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"dynasmrt/struct.UncommittedModifier.html\" title=\"struct dynasmrt::UncommittedModifier\">UncommittedModifier</a>&lt;'a&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/dynasmrt/lib.rs.html#1047\">Source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.85.0/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.85.0/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/1.85.0/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.85.0/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","dynasmrt::x64::UncommittedModifier","dynasmrt::x86::UncommittedModifier","dynasmrt::aarch64::UncommittedModifier","dynasmrt::riscv::UncommittedModifier"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-DynasmApi-for-UncommittedModifier%3C'a%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/dynasmrt/lib.rs.html#1088-1106\">Source</a><a href=\"#impl-DynasmApi-for-UncommittedModifier%3C'a%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a&gt; <a class=\"trait\" href=\"dynasmrt/trait.DynasmApi.html\" title=\"trait dynasmrt::DynasmApi\">DynasmApi</a> for <a class=\"struct\" href=\"dynasmrt/struct.UncommittedModifier.html\" title=\"struct dynasmrt::UncommittedModifier\">UncommittedModifier</a>&lt;'a&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.offset\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/dynasmrt/lib.rs.html#1089-1091\">Source</a><a href=\"#method.offset\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"dynasmrt/trait.DynasmApi.html#tymethod.offset\" class=\"fn\">offset</a>(&amp;self) -&gt; <a class=\"struct\" href=\"dynasmrt/struct.AssemblyOffset.html\" title=\"struct dynasmrt::AssemblyOffset\">AssemblyOffset</a></h4></section></summary><div class='docblock'>Report the current offset into the assembling target</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.push\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/dynasmrt/lib.rs.html#1093-1096\">Source</a><a href=\"#method.push\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"dynasmrt/trait.DynasmApi.html#tymethod.push\" class=\"fn\">push</a>(&amp;mut self, value: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.u8.html\">u8</a>)</h4></section></summary><div class='docblock'>Push a byte into the assembling target</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.align\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/dynasmrt/lib.rs.html#1098-1105\">Source</a><a href=\"#method.align\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"dynasmrt/trait.DynasmApi.html#tymethod.align\" class=\"fn\">align</a>(&amp;mut self, alignment: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.usize.html\">usize</a>, with: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.u8.html\">u8</a>)</h4></section></summary><div class='docblock'>Push filler until the assembling target end is aligned to the given alignment.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.push_i8\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/dynasmrt/lib.rs.html#200-202\">Source</a><a href=\"#method.push_i8\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"dynasmrt/trait.DynasmApi.html#method.push_i8\" class=\"fn\">push_i8</a>(&amp;mut self, value: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.i8.html\">i8</a>)</h4></section></summary><div class='docblock'>Push a signed byte into the assembling target</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.push_i16\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/dynasmrt/lib.rs.html#205-207\">Source</a><a href=\"#method.push_i16\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"dynasmrt/trait.DynasmApi.html#method.push_i16\" class=\"fn\">push_i16</a>(&amp;mut self, value: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.i16.html\">i16</a>)</h4></section></summary><div class='docblock'>Push a signed word into the assembling target</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.push_i32\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/dynasmrt/lib.rs.html#210-212\">Source</a><a href=\"#method.push_i32\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"dynasmrt/trait.DynasmApi.html#method.push_i32\" class=\"fn\">push_i32</a>(&amp;mut self, value: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.i32.html\">i32</a>)</h4></section></summary><div class='docblock'>Push a signed doubleword into the assembling target</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.push_i64\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/dynasmrt/lib.rs.html#215-217\">Source</a><a href=\"#method.push_i64\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"dynasmrt/trait.DynasmApi.html#method.push_i64\" class=\"fn\">push_i64</a>(&amp;mut self, value: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.i64.html\">i64</a>)</h4></section></summary><div class='docblock'>Push a signed quadword into the assembling target</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.push_u16\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/dynasmrt/lib.rs.html#220-222\">Source</a><a href=\"#method.push_u16\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"dynasmrt/trait.DynasmApi.html#method.push_u16\" class=\"fn\">push_u16</a>(&amp;mut self, value: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.u16.html\">u16</a>)</h4></section></summary><div class='docblock'>Push an usigned word into the assembling target</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.push_u32\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/dynasmrt/lib.rs.html#225-227\">Source</a><a href=\"#method.push_u32\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"dynasmrt/trait.DynasmApi.html#method.push_u32\" class=\"fn\">push_u32</a>(&amp;mut self, value: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.u32.html\">u32</a>)</h4></section></summary><div class='docblock'>Push an usigned doubleword into the assembling target</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.push_u64\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/dynasmrt/lib.rs.html#230-232\">Source</a><a href=\"#method.push_u64\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"dynasmrt/trait.DynasmApi.html#method.push_u64\" class=\"fn\">push_u64</a>(&amp;mut self, value: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.u64.html\">u64</a>)</h4></section></summary><div class='docblock'>Push an usigned quadword into the assembling target</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.runtime_error\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/dynasmrt/lib.rs.html#235-237\">Source</a><a href=\"#method.runtime_error\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"dynasmrt/trait.DynasmApi.html#method.runtime_error\" class=\"fn\">runtime_error</a>(&amp;self, msg: &amp;'static <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.str.html\">str</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.never.html\">!</a></h4></section></summary><div class='docblock'>This function is called in when a runtime error has to be generated. It panics.</div></details></div></details>","DynasmApi","dynasmrt::x64::UncommittedModifier","dynasmrt::x86::UncommittedModifier","dynasmrt::aarch64::UncommittedModifier","dynasmrt::riscv::UncommittedModifier"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Extend%3C%26u8%3E-for-UncommittedModifier%3C'a%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/dynasmrt/lib.rs.html#1116-1120\">Source</a><a href=\"#impl-Extend%3C%26u8%3E-for-UncommittedModifier%3C'a%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, 'b&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;&amp;'b <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.u8.html\">u8</a>&gt; for <a class=\"struct\" href=\"dynasmrt/struct.UncommittedModifier.html\" title=\"struct dynasmrt::UncommittedModifier\">UncommittedModifier</a>&lt;'a&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.extend\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/dynasmrt/lib.rs.html#1117-1119\">Source</a><a href=\"#method.extend\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.85.0/core/iter/traits/collect/trait.Extend.html#tymethod.extend\" class=\"fn\">extend</a>&lt;T&gt;(&amp;mut self, iter: T)<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/iter/traits/collect/trait.IntoIterator.html\" title=\"trait core::iter::traits::collect::IntoIterator\">IntoIterator</a>&lt;Item = &amp;'b <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.u8.html\">u8</a>&gt;,</div></h4></section></summary><div class='docblock'>Extends a collection with the contents of an iterator. <a href=\"https://doc.rust-lang.org/1.85.0/core/iter/traits/collect/trait.Extend.html#tymethod.extend\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.extend_one\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/1.85.0/src/core/iter\\traits/collect.rs.html#420\">Source</a><a href=\"#method.extend_one\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.85.0/core/iter/traits/collect/trait.Extend.html#method.extend_one\" class=\"fn\">extend_one</a>(&amp;mut self, item: A)</h4></section></summary><span class=\"item-info\"><div class=\"stab unstable\"><span class=\"emoji\">🔬</span><span>This is a nightly-only experimental API. (<code>extend_one</code>)</span></div></span><div class='docblock'>Extends a collection with exactly one element.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.extend_reserve\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/1.85.0/src/core/iter\\traits/collect.rs.html#428\">Source</a><a href=\"#method.extend_reserve\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.85.0/core/iter/traits/collect/trait.Extend.html#method.extend_reserve\" class=\"fn\">extend_reserve</a>(&amp;mut self, additional: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.usize.html\">usize</a>)</h4></section></summary><span class=\"item-info\"><div class=\"stab unstable\"><span class=\"emoji\">🔬</span><span>This is a nightly-only experimental API. (<code>extend_one</code>)</span></div></span><div class='docblock'>Reserves capacity in a collection for the given number of additional elements. <a href=\"https://doc.rust-lang.org/1.85.0/core/iter/traits/collect/trait.Extend.html#method.extend_reserve\">Read more</a></div></details></div></details>","Extend<&'b u8>","dynasmrt::x64::UncommittedModifier","dynasmrt::x86::UncommittedModifier","dynasmrt::aarch64::UncommittedModifier","dynasmrt::riscv::UncommittedModifier"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Extend%3Cu8%3E-for-UncommittedModifier%3C'a%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/dynasmrt/lib.rs.html#1108-1114\">Source</a><a href=\"#impl-Extend%3Cu8%3E-for-UncommittedModifier%3C'a%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/iter/traits/collect/trait.Extend.html\" title=\"trait core::iter::traits::collect::Extend\">Extend</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.u8.html\">u8</a>&gt; for <a class=\"struct\" href=\"dynasmrt/struct.UncommittedModifier.html\" title=\"struct dynasmrt::UncommittedModifier\">UncommittedModifier</a>&lt;'a&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.extend\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/dynasmrt/lib.rs.html#1109-1113\">Source</a><a href=\"#method.extend\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.85.0/core/iter/traits/collect/trait.Extend.html#tymethod.extend\" class=\"fn\">extend</a>&lt;T&gt;(&amp;mut self, iter: T)<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/iter/traits/collect/trait.IntoIterator.html\" title=\"trait core::iter::traits::collect::IntoIterator\">IntoIterator</a>&lt;Item = <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.u8.html\">u8</a>&gt;,</div></h4></section></summary><div class='docblock'>Extends a collection with the contents of an iterator. <a href=\"https://doc.rust-lang.org/1.85.0/core/iter/traits/collect/trait.Extend.html#tymethod.extend\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.extend_one\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/1.85.0/src/core/iter\\traits/collect.rs.html#420\">Source</a><a href=\"#method.extend_one\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.85.0/core/iter/traits/collect/trait.Extend.html#method.extend_one\" class=\"fn\">extend_one</a>(&amp;mut self, item: A)</h4></section></summary><span class=\"item-info\"><div class=\"stab unstable\"><span class=\"emoji\">🔬</span><span>This is a nightly-only experimental API. (<code>extend_one</code>)</span></div></span><div class='docblock'>Extends a collection with exactly one element.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.extend_reserve\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/1.85.0/src/core/iter\\traits/collect.rs.html#428\">Source</a><a href=\"#method.extend_reserve\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.85.0/core/iter/traits/collect/trait.Extend.html#method.extend_reserve\" class=\"fn\">extend_reserve</a>(&amp;mut self, additional: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.usize.html\">usize</a>)</h4></section></summary><span class=\"item-info\"><div class=\"stab unstable\"><span class=\"emoji\">🔬</span><span>This is a nightly-only experimental API. (<code>extend_one</code>)</span></div></span><div class='docblock'>Reserves capacity in a collection for the given number of additional elements. <a href=\"https://doc.rust-lang.org/1.85.0/core/iter/traits/collect/trait.Extend.html#method.extend_reserve\">Read more</a></div></details></div></details>","Extend<u8>","dynasmrt::x64::UncommittedModifier","dynasmrt::x86::UncommittedModifier","dynasmrt::aarch64::UncommittedModifier","dynasmrt::riscv::UncommittedModifier"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-UncommittedModifier%3C'a%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/dynasmrt/lib.rs.html#1054-1086\">Source</a><a href=\"#impl-UncommittedModifier%3C'a%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a&gt; <a class=\"struct\" href=\"dynasmrt/struct.UncommittedModifier.html\" title=\"struct dynasmrt::UncommittedModifier\">UncommittedModifier</a>&lt;'a&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.new\" class=\"method\"><a class=\"src rightside\" href=\"src/dynasmrt/lib.rs.html#1056-1062\">Source</a><h4 class=\"code-header\">pub fn <a href=\"dynasmrt/struct.UncommittedModifier.html#tymethod.new\" class=\"fn\">new</a>(\n    buffer: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.85.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.u8.html\">u8</a>&gt;,\n    base_offset: <a class=\"struct\" href=\"dynasmrt/struct.AssemblyOffset.html\" title=\"struct dynasmrt::AssemblyOffset\">AssemblyOffset</a>,\n) -&gt; <a class=\"struct\" href=\"dynasmrt/struct.UncommittedModifier.html\" title=\"struct dynasmrt::UncommittedModifier\">UncommittedModifier</a>&lt;'_&gt;</h4></section></summary><div class=\"docblock\"><p>create a new uncommittedmodifier</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.goto\" class=\"method\"><a class=\"src rightside\" href=\"src/dynasmrt/lib.rs.html#1065-1067\">Source</a><h4 class=\"code-header\">pub fn <a href=\"dynasmrt/struct.UncommittedModifier.html#tymethod.goto\" class=\"fn\">goto</a>(&amp;mut self, offset: <a class=\"struct\" href=\"dynasmrt/struct.AssemblyOffset.html\" title=\"struct dynasmrt::AssemblyOffset\">AssemblyOffset</a>)</h4></section></summary><div class=\"docblock\"><p>Sets the current modification offset to the given value</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.check\" class=\"method\"><a class=\"src rightside\" href=\"src/dynasmrt/lib.rs.html#1070-1076\">Source</a><h4 class=\"code-header\">pub fn <a href=\"dynasmrt/struct.UncommittedModifier.html#tymethod.check\" class=\"fn\">check</a>(&amp;mut self, offset: <a class=\"struct\" href=\"dynasmrt/struct.AssemblyOffset.html\" title=\"struct dynasmrt::AssemblyOffset\">AssemblyOffset</a>) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.85.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.unit.html\">()</a>, <a class=\"enum\" href=\"dynasmrt/enum.DynasmError.html\" title=\"enum dynasmrt::DynasmError\">DynasmError</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Checks that the current modification offset is not larger than the specified offset.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.check_exact\" class=\"method\"><a class=\"src rightside\" href=\"src/dynasmrt/lib.rs.html#1079-1085\">Source</a><h4 class=\"code-header\">pub fn <a href=\"dynasmrt/struct.UncommittedModifier.html#tymethod.check_exact\" class=\"fn\">check_exact</a>(&amp;mut self, offset: <a class=\"struct\" href=\"dynasmrt/struct.AssemblyOffset.html\" title=\"struct dynasmrt::AssemblyOffset\">AssemblyOffset</a>) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.85.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.unit.html\">()</a>, <a class=\"enum\" href=\"dynasmrt/enum.DynasmError.html\" title=\"enum dynasmrt::DynasmError\">DynasmError</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Checks that the current modification offset is exactly the specified offset.</p>\n</div></details></div></details>",0,"dynasmrt::x64::UncommittedModifier","dynasmrt::x86::UncommittedModifier","dynasmrt::aarch64::UncommittedModifier","dynasmrt::riscv::UncommittedModifier"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[21197]}