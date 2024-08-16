(function() {var type_impls = {
"fastcrypto":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Default-for-HashFunctionWrapper%3CVariant,+DIGEST_LEN%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/fastcrypto/hash.rs.html#119\">source</a><a href=\"#impl-Default-for-HashFunctionWrapper%3CVariant,+DIGEST_LEN%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;Variant: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.80.1/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>, const DIGEST_LEN: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.80.1/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"fastcrypto/hash/struct.HashFunctionWrapper.html\" title=\"struct fastcrypto::hash::HashFunctionWrapper\">HashFunctionWrapper</a>&lt;Variant, DIGEST_LEN&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.default\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/fastcrypto/hash.rs.html#119\">source</a><a href=\"#method.default\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.80.1/core/default/trait.Default.html#tymethod.default\" class=\"fn\">default</a>() -&gt; <a class=\"struct\" href=\"fastcrypto/hash/struct.HashFunctionWrapper.html\" title=\"struct fastcrypto::hash::HashFunctionWrapper\">HashFunctionWrapper</a>&lt;Variant, DIGEST_LEN&gt; <a href=\"#\" class=\"tooltip\" data-notable-ty=\"HashFunctionWrapper&lt;Variant, DIGEST_LEN&gt;\">ⓘ</a></h4></section></summary><div class='docblock'>Returns the “default value” for a type. <a href=\"https://doc.rust-lang.org/1.80.1/core/default/trait.Default.html#tymethod.default\">Read more</a></div></details></div></details>","Default","fastcrypto::hash::Sha256","fastcrypto::hash::Sha3_256","fastcrypto::hash::Sha512","fastcrypto::hash::Sha3_512","fastcrypto::hash::Keccak256","fastcrypto::hash::Blake2b256"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-HashFunction%3CDIGEST_LEN%3E-for-HashFunctionWrapper%3CVariant,+DIGEST_LEN%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/fastcrypto/hash.rs.html#133-146\">source</a><a href=\"#impl-HashFunction%3CDIGEST_LEN%3E-for-HashFunctionWrapper%3CVariant,+DIGEST_LEN%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;Variant: Digest + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.80.1/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>, const DIGEST_LEN: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"fastcrypto/hash/trait.HashFunction.html\" title=\"trait fastcrypto::hash::HashFunction\">HashFunction</a>&lt;DIGEST_LEN&gt; for <a class=\"struct\" href=\"fastcrypto/hash/struct.HashFunctionWrapper.html\" title=\"struct fastcrypto::hash::HashFunctionWrapper\">HashFunctionWrapper</a>&lt;Variant, DIGEST_LEN&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.update\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/fastcrypto/hash.rs.html#136-138\">source</a><a href=\"#method.update\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"fastcrypto/hash/trait.HashFunction.html#tymethod.update\" class=\"fn\">update</a>&lt;Data: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.80.1/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.u8.html\">u8</a>]&gt;&gt;(&amp;mut self, data: Data)</h4></section></summary><div class='docblock'>Process the given data, and update the internal of the hash function.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.finalize\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/fastcrypto/hash.rs.html#140-145\">source</a><a href=\"#method.finalize\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"fastcrypto/hash/trait.HashFunction.html#tymethod.finalize\" class=\"fn\">finalize</a>(self) -&gt; <a class=\"struct\" href=\"fastcrypto/hash/struct.Digest.html\" title=\"struct fastcrypto::hash::Digest\">Digest</a>&lt;DIGEST_LEN&gt;</h4></section></summary><div class='docblock'>Retrieve result and consume hash function.</div></details><details class=\"toggle\" open><summary><section id=\"associatedconstant.OUTPUT_SIZE\" class=\"associatedconstant trait-impl\"><a class=\"src rightside\" href=\"src/fastcrypto/hash.rs.html#82\">source</a><a href=\"#associatedconstant.OUTPUT_SIZE\" class=\"anchor\">§</a><h4 class=\"code-header\">const <a href=\"fastcrypto/hash/trait.HashFunction.html#associatedconstant.OUTPUT_SIZE\" class=\"constant\">OUTPUT_SIZE</a>: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.usize.html\">usize</a> = DIGEST_LENGTH</h4></section></summary><div class='docblock'>The length of this hash functions digests in bytes.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.new\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/fastcrypto/hash.rs.html#85-87\">source</a><a href=\"#method.new\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"fastcrypto/hash/trait.HashFunction.html#method.new\" class=\"fn\">new</a>() -&gt; Self</h4></section></summary><div class='docblock'>Create a new hash function of the given type</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.digest\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/fastcrypto/hash.rs.html#96-100\">source</a><a href=\"#method.digest\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"fastcrypto/hash/trait.HashFunction.html#method.digest\" class=\"fn\">digest</a>&lt;Data: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.80.1/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.u8.html\">u8</a>]&gt;&gt;(data: Data) -&gt; <a class=\"struct\" href=\"fastcrypto/hash/struct.Digest.html\" title=\"struct fastcrypto::hash::Digest\">Digest</a>&lt;DIGEST_LENGTH&gt;</h4></section></summary><div class='docblock'>Compute the digest of the given data and consume the hash function.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.digest_iterator\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/fastcrypto/hash.rs.html#103-107\">source</a><a href=\"#method.digest_iterator\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"fastcrypto/hash/trait.HashFunction.html#method.digest_iterator\" class=\"fn\">digest_iterator</a>&lt;K: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.80.1/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.u8.html\">u8</a>]&gt;, I: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.80.1/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&lt;Item = K&gt;&gt;(\n    iter: I,\n) -&gt; <a class=\"struct\" href=\"fastcrypto/hash/struct.Digest.html\" title=\"struct fastcrypto::hash::Digest\">Digest</a>&lt;DIGEST_LENGTH&gt;</h4></section></summary><div class='docblock'>Compute a single digest from all slices in the iterator in order and consume the hash function.</div></details></div></details>","HashFunction<DIGEST_LEN>","fastcrypto::hash::Sha256","fastcrypto::hash::Sha3_256","fastcrypto::hash::Sha512","fastcrypto::hash::Sha3_512","fastcrypto::hash::Keccak256","fastcrypto::hash::Blake2b256"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-ReverseWrapper-for-HashFunctionWrapper%3CVariant,+DIGEST_LEN%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/fastcrypto/hash.rs.html#127-131\">source</a><a href=\"#impl-ReverseWrapper-for-HashFunctionWrapper%3CVariant,+DIGEST_LEN%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;Variant: CoreProxy + OutputSizeUser, const DIGEST_LEN: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"fastcrypto/hash/trait.ReverseWrapper.html\" title=\"trait fastcrypto::hash::ReverseWrapper\">ReverseWrapper</a> for <a class=\"struct\" href=\"fastcrypto/hash/struct.HashFunctionWrapper.html\" title=\"struct fastcrypto::hash::HashFunctionWrapper\">HashFunctionWrapper</a>&lt;Variant, DIGEST_LEN&gt;</h3></section></summary><div class=\"impl-items\"><section id=\"associatedtype.Variant\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Variant\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"fastcrypto/hash/trait.ReverseWrapper.html#associatedtype.Variant\" class=\"associatedtype\">Variant</a> = Variant</h4></section></div></details>","ReverseWrapper","fastcrypto::hash::Sha256","fastcrypto::hash::Sha3_256","fastcrypto::hash::Sha512","fastcrypto::hash::Sha3_512","fastcrypto::hash::Keccak256","fastcrypto::hash::Blake2b256"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Write-for-HashFunctionWrapper%3CVariant,+DIGEST_LEN%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/fastcrypto/hash.rs.html#149-160\">source</a><a href=\"#impl-Write-for-HashFunctionWrapper%3CVariant,+DIGEST_LEN%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;Variant: Digest + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.80.1/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>, const DIGEST_LEN: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.80.1/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a> for <a class=\"struct\" href=\"fastcrypto/hash/struct.HashFunctionWrapper.html\" title=\"struct fastcrypto::hash::HashFunctionWrapper\">HashFunctionWrapper</a>&lt;Variant, DIGEST_LEN&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.write\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/fastcrypto/hash.rs.html#152-155\">source</a><a href=\"#method.write\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.80.1/std/io/trait.Write.html#tymethod.write\" class=\"fn\">write</a>(&amp;mut self, buf: &amp;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.u8.html\">u8</a>]) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/1.80.1/std/io/error/type.Result.html\" title=\"type std::io::error::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.usize.html\">usize</a>&gt;</h4></section></summary><div class='docblock'>Write a buffer into this writer, returning how many bytes were written. <a href=\"https://doc.rust-lang.org/1.80.1/std/io/trait.Write.html#tymethod.write\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.flush\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/fastcrypto/hash.rs.html#157-159\">source</a><a href=\"#method.flush\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.80.1/std/io/trait.Write.html#tymethod.flush\" class=\"fn\">flush</a>(&amp;mut self) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/1.80.1/std/io/error/type.Result.html\" title=\"type std::io::error::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.unit.html\">()</a>&gt;</h4></section></summary><div class='docblock'>Flush this output stream, ensuring that all intermediately buffered\ncontents reach their destination. <a href=\"https://doc.rust-lang.org/1.80.1/std/io/trait.Write.html#tymethod.flush\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.write_vectored\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.36.0\">1.36.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.80.1/src/std/io/mod.rs.html#1622\">source</a></span><a href=\"#method.write_vectored\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.80.1/std/io/trait.Write.html#method.write_vectored\" class=\"fn\">write_vectored</a>(&amp;mut self, bufs: &amp;[<a class=\"struct\" href=\"https://doc.rust-lang.org/1.80.1/std/io/struct.IoSlice.html\" title=\"struct std::io::IoSlice\">IoSlice</a>&lt;'_&gt;]) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.80.1/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.usize.html\">usize</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.80.1/std/io/error/struct.Error.html\" title=\"struct std::io::error::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Like <a href=\"https://doc.rust-lang.org/1.80.1/std/io/trait.Write.html#tymethod.write\" title=\"method std::io::Write::write\"><code>write</code></a>, except that it writes from a slice of buffers. <a href=\"https://doc.rust-lang.org/1.80.1/std/io/trait.Write.html#method.write_vectored\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.is_write_vectored\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/1.80.1/src/std/io/mod.rs.html#1637\">source</a><a href=\"#method.is_write_vectored\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.80.1/std/io/trait.Write.html#method.is_write_vectored\" class=\"fn\">is_write_vectored</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.bool.html\">bool</a></h4></section></summary><span class=\"item-info\"><div class=\"stab unstable\"><span class=\"emoji\">🔬</span><span>This is a nightly-only experimental API. (<code>can_vector</code>)</span></div></span><div class='docblock'>Determines if this <code>Write</code>r has an efficient <a href=\"https://doc.rust-lang.org/1.80.1/std/io/trait.Write.html#method.write_vectored\" title=\"method std::io::Write::write_vectored\"><code>write_vectored</code></a>\nimplementation. <a href=\"https://doc.rust-lang.org/1.80.1/std/io/trait.Write.html#method.is_write_vectored\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.write_all\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.80.1/src/std/io/mod.rs.html#1699\">source</a></span><a href=\"#method.write_all\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.80.1/std/io/trait.Write.html#method.write_all\" class=\"fn\">write_all</a>(&amp;mut self, buf: &amp;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.u8.html\">u8</a>]) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.80.1/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.80.1/std/io/error/struct.Error.html\" title=\"struct std::io::error::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Attempts to write an entire buffer into this writer. <a href=\"https://doc.rust-lang.org/1.80.1/std/io/trait.Write.html#method.write_all\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.write_all_vectored\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/1.80.1/src/std/io/mod.rs.html#1761\">source</a><a href=\"#method.write_all_vectored\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.80.1/std/io/trait.Write.html#method.write_all_vectored\" class=\"fn\">write_all_vectored</a>(&amp;mut self, bufs: &amp;mut [<a class=\"struct\" href=\"https://doc.rust-lang.org/1.80.1/std/io/struct.IoSlice.html\" title=\"struct std::io::IoSlice\">IoSlice</a>&lt;'_&gt;]) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.80.1/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.80.1/std/io/error/struct.Error.html\" title=\"struct std::io::error::Error\">Error</a>&gt;</h4></section></summary><span class=\"item-info\"><div class=\"stab unstable\"><span class=\"emoji\">🔬</span><span>This is a nightly-only experimental API. (<code>write_all_vectored</code>)</span></div></span><div class='docblock'>Attempts to write multiple buffers into this writer. <a href=\"https://doc.rust-lang.org/1.80.1/std/io/trait.Write.html#method.write_all_vectored\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.write_fmt\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.80.1/src/std/io/mod.rs.html#1814\">source</a></span><a href=\"#method.write_fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.80.1/std/io/trait.Write.html#method.write_fmt\" class=\"fn\">write_fmt</a>(&amp;mut self, fmt: <a class=\"struct\" href=\"https://doc.rust-lang.org/1.80.1/core/fmt/struct.Arguments.html\" title=\"struct core::fmt::Arguments\">Arguments</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.80.1/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.80.1/std/io/error/struct.Error.html\" title=\"struct std::io::error::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Writes a formatted string into this writer, returning any error\nencountered. <a href=\"https://doc.rust-lang.org/1.80.1/std/io/trait.Write.html#method.write_fmt\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.by_ref\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.80.1/src/std/io/mod.rs.html#1874-1876\">source</a></span><a href=\"#method.by_ref\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.80.1/std/io/trait.Write.html#method.by_ref\" class=\"fn\">by_ref</a>(&amp;mut self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.reference.html\">&amp;mut Self</a><div class=\"where\">where\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.80.1/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</div></h4></section></summary><div class='docblock'>Creates a “by reference” adapter for this instance of <code>Write</code>. <a href=\"https://doc.rust-lang.org/1.80.1/std/io/trait.Write.html#method.by_ref\">Read more</a></div></details></div></details>","Write","fastcrypto::hash::Sha256","fastcrypto::hash::Sha3_256","fastcrypto::hash::Sha512","fastcrypto::hash::Sha3_512","fastcrypto::hash::Keccak256","fastcrypto::hash::Blake2b256"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()