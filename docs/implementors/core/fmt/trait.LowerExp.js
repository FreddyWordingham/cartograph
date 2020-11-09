(function() {var implementors = {};
implementors["ansi_rgb"] = [{"text":"impl&lt;T&gt; LowerExp for WithBackground&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: LowerExp,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; LowerExp for WithForeground&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: LowerExp,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["console"] = [{"text":"impl&lt;D:&nbsp;LowerExp&gt; LowerExp for StyledObject&lt;D&gt;","synthetic":false,"types":[]}];
implementors["itertools"] = [{"text":"impl&lt;'a, I&gt; LowerExp for Format&lt;'a, I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: Iterator,<br>&nbsp;&nbsp;&nbsp;&nbsp;I::Item: LowerExp,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["nalgebra"] = [{"text":"impl&lt;N, R:&nbsp;Dim, C:&nbsp;Dim, S&gt; LowerExp for Matrix&lt;N, R, C, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar + LowerExp,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Storage&lt;N, R, C&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;usize, R, C&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["ndarray"] = [{"text":"impl&lt;'a, A:&nbsp;LowerExp, S, D:&nbsp;Dimension&gt; LowerExp for ArrayBase&lt;S, D&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Data&lt;Elem = A&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["noisy_float"] = [{"text":"impl&lt;F:&nbsp;Float + LowerExp, C:&nbsp;FloatChecker&lt;F&gt;&gt; LowerExp for NoisyFloat&lt;F, C&gt;","synthetic":false,"types":[]}];
implementors["num_complex"] = [{"text":"impl&lt;T&gt; LowerExp for Complex&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: LowerExp + Num + PartialOrd + Clone,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["num_rational"] = [{"text":"impl&lt;T:&nbsp;LowerExp + Clone + Integer&gt; LowerExp for Ratio&lt;T&gt;","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()