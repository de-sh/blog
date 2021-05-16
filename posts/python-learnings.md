---
title: 'Awesome new Python learnings'
slug: 'python-learnings'
summary: 'A brief on all the awesome new stuff I am learning in the world of Python programming'
date: 'March 8, 2019'
---
<p>Of late, I have been working on a few Open Source Python projects, thus I have been exposed to a lot of new syntax and coding styles that were once pretty alien to me, all my sureties about having been an awesome Pythonista have since come crashing down. While this might seem to be a really sad realization, I'd rather say it was a pleasent learning experience and more of a surprise than a rude shock.</p>

<p>I am really glad that I started working on these projects, they are helping me make the kind of mental models for the purpose of writing code, that I otherwise wouldn't have been able to do in even a corporate setup. Some of these realizations are being covered here, in what I would prefer to remain, a short note.</p>

<h2 id="numerical-seperators">Numerical Seperators</h2>

<p>I seriously don't know what to call these and I didn't know that they existed, yet here I am. So the thing is, I never worked with enormous numbers, I seriously am bad at dealing with big ints and the one time in my life where I did really big calculations, I used a lot of JS and the whole thing flopped, must have been just me or it was just bad timing.</p>

<p>Anyways, I have learnt over the last few days that within Python you can seperated numbers that are large into the "place notation", i.e. similar to how you write million as 1,000,000 instead of 1000000. This can be done with the seperators being underscores "_" defined in <a href="https://www.python.org/dev/peps/pep-0515/">PEP 515</a>.</p>
<div class="highlight">
	<pre style="color:#f8f8f2;background-color:#272822;-moz-tab-size:4;-o-tab-size:4;tab-size:4">
<code class="language-python" data-lang="python">
M <span style="color:#f92672">=</span> <span style="color:#ae81ff">1</span>_000_000
<span style="color:#66d9ef">print</span>(M) <span style="color:#75715e"># Outputs ~ 1000000</span>
</code>
	</pre>
</div>

<h2 id="function-annotations">Function Annotations</h2>

<p>I had thought all through my 6 years coding with Python that it was just for those who wanted to be type agnostic, with data being preferably just of a single type. But this I have since come to learn is very tough an ask, esepecially since data is of too many types to not have a caveat when storing large numbers or in storing strings that are not as large, but use very complex characters, how are we to represent the same in memory?</p>

<p>These issues are taken care of by the use of the Python Annotations that came to be from <a href="https://www.python.org/dev/peps/pep-3107/">PEP 3107</a></p>
<div class="highlight">
	<pre style="color:#f8f8f2;background-color:#272822;-moz-tab-size:4;-o-tab-size:4;tab-size:4">
<code class="language-python" data-lang="python"><span style="color:#66d9ef">def</span> <span style="color:#a6e22e">func</span>(val:str <span style="color:#f92672">=</span> <span style="color:#e6db74">"Hello"</span>) <span style="color:#f92672">-></span> str: <span style="color:#75715e"># "Hello" being default value</span>
    <span style="color:#66d9ef">return</span> val
</code>
	</pre>
</div>

<p>The main aim of this technique is to take away some of the decisions regarding data types from the interpreter and give it back to the programmer. While you can argue that there is no use to make the whole thing a standard practice, I think it was a good step nonetheless to include it.</p>

<h2 id="function-decorators">Function Decorators</h2>

<p>With my knowledge of the language being extremely narrow and confused, with a wide variety of misconceptions having formed, especially about what is and what is not pythonic code, it is tough for me to judge what is the best way to solve a problem. I am a person who prefers complex, yet performant code over the easy to write code, but Python has thought me that you can have the cake and eat it too, seldom slowly. As is also pointed out in the <a href="https://www.python.org/dev/peps/pep-0318/">PEP 318</a>.</p>
<div class="highlight">
	<pre style="color:#f8f8f2;background-color:#272822;-moz-tab-size:4;-o-tab-size:4;tab-size:4">
<code class="language-python" data-lang="python">
<span style="color:#a6e22e">@callable</span>(var) <span style="color:#75715e"># callable() is being called on another function, func()</span>
<span style="color:#66d9ef">def</span> <span style="color:#a6e22e">func</span>(val:type) <span style="color:#f92672">-></span> str:
    <span style="color:#66d9ef">pass</span> <span style="color:#75715e"># this statement does nothing</span>
</code>
	</pre>
</div>

<h2 id="conclusions">Conclusions</h2>

<p>There are a lot many new features in the ever-expanding arsenal of the Python developer and this helps programmers write performant code that is <strong>pythonic</strong> in nature.</p>
