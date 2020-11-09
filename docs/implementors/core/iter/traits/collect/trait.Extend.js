(function() {var implementors = {};
implementors["either"] = [{"text":"impl&lt;L, R, A&gt; Extend&lt;A&gt; for Either&lt;L, R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;L: Extend&lt;A&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Extend&lt;A&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["hashbrown"] = [{"text":"impl&lt;K, V, S&gt; Extend&lt;(K, V)&gt; for HashMap&lt;K, V, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: Eq + Hash,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, K, V, S&gt; Extend&lt;(&amp;'a K, &amp;'a V)&gt; for HashMap&lt;K, V, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: Eq + Hash + Copy,<br>&nbsp;&nbsp;&nbsp;&nbsp;V: Copy,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T, S&gt; Extend&lt;T&gt; for HashSet&lt;T, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Eq + Hash,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, T, S&gt; Extend&lt;&amp;'a T&gt; for HashSet&lt;T, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: 'a + Eq + Hash + Copy,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["indexmap"] = [{"text":"impl&lt;K, V, S&gt; Extend&lt;(K, V)&gt; for IndexMap&lt;K, V, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: Hash + Eq,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, K, V, S&gt; Extend&lt;(&amp;'a K, &amp;'a V)&gt; for IndexMap&lt;K, V, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: Hash + Eq + Copy,<br>&nbsp;&nbsp;&nbsp;&nbsp;V: Copy,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T, S&gt; Extend&lt;T&gt; for IndexSet&lt;T, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Hash + Eq,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, T, S&gt; Extend&lt;&amp;'a T&gt; for IndexSet&lt;T, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Hash + Eq + Copy + 'a,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["nalgebra"] = [{"text":"impl&lt;N, R, S&gt; Extend&lt;N&gt; for Matrix&lt;N, R, Dynamic, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar,<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Dim,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Extend&lt;N&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N, S&gt; Extend&lt;N&gt; for Matrix&lt;N, Dynamic, U1, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Extend&lt;N&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N, R, S, RV, SV&gt; Extend&lt;Matrix&lt;N, RV, U1, SV&gt;&gt; for Matrix&lt;N, R, Dynamic, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar,<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Dim,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Extend&lt;Vector&lt;N, RV, SV&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;RV: Dim,<br>&nbsp;&nbsp;&nbsp;&nbsp;SV: Storage&lt;N, RV&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;ShapeConstraint: SameNumberOfRows&lt;R, RV&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N, R:&nbsp;Dim&gt; Extend&lt;N&gt; for VecStorage&lt;N, R, Dynamic&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, N:&nbsp;'a + Copy, R:&nbsp;Dim&gt; Extend&lt;&amp;'a N&gt; for VecStorage&lt;N, R, Dynamic&gt;","synthetic":false,"types":[]},{"text":"impl&lt;N, R, RV, SV&gt; Extend&lt;Matrix&lt;N, RV, U1, SV&gt;&gt; for VecStorage&lt;N, R, Dynamic&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar,<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Dim,<br>&nbsp;&nbsp;&nbsp;&nbsp;RV: Dim,<br>&nbsp;&nbsp;&nbsp;&nbsp;SV: Storage&lt;N, RV&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;ShapeConstraint: SameNumberOfRows&lt;R, RV&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N&gt; Extend&lt;N&gt; for VecStorage&lt;N, Dynamic, U1&gt;","synthetic":false,"types":[]}];
implementors["netcdf"] = [{"text":"impl Extend&lt;Options&gt; for Options","synthetic":false,"types":[]}];
implementors["proc_macro2"] = [{"text":"impl Extend&lt;TokenTree&gt; for TokenStream","synthetic":false,"types":[]},{"text":"impl Extend&lt;TokenStream&gt; for TokenStream","synthetic":false,"types":[]}];
implementors["serde_json"] = [{"text":"impl Extend&lt;(String, Value)&gt; for Map&lt;String, Value&gt;","synthetic":false,"types":[]}];
implementors["syn"] = [{"text":"impl&lt;T, P&gt; Extend&lt;T&gt; for Punctuated&lt;T, P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: Default,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T, P&gt; Extend&lt;Pair&lt;T, P&gt;&gt; for Punctuated&lt;T, P&gt;","synthetic":false,"types":[]},{"text":"impl Extend&lt;Error&gt; for Error","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()