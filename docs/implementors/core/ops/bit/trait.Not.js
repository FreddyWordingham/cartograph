(function() {var implementors = {};
implementors["ndarray"] = [{"text":"impl&lt;A, S, D&gt; Not for ArrayBase&lt;S, D&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Clone + Not&lt;Output = A&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: DataOwned&lt;Elem = A&gt; + DataMut,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: Dimension,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, A, S, D&gt; Not for &amp;'a ArrayBase&lt;S, D&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;&amp;'a A: 'a + Not&lt;Output = A&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Data&lt;Elem = A&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: Dimension,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["netcdf"] = [{"text":"impl Not for Options","synthetic":false,"types":[]}];
implementors["simba"] = [{"text":"impl Not for AutoSimd&lt;[bool; 1]&gt;","synthetic":false,"types":[]},{"text":"impl Not for AutoSimd&lt;[bool; 2]&gt;","synthetic":false,"types":[]},{"text":"impl Not for AutoSimd&lt;[bool; 4]&gt;","synthetic":false,"types":[]},{"text":"impl Not for AutoSimd&lt;[bool; 8]&gt;","synthetic":false,"types":[]},{"text":"impl Not for AutoSimd&lt;[bool; 16]&gt;","synthetic":false,"types":[]},{"text":"impl Not for AutoSimd&lt;[bool; 32]&gt;","synthetic":false,"types":[]}];
implementors["typenum"] = [{"text":"impl Not for B0","synthetic":false,"types":[]},{"text":"impl Not for B1","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()