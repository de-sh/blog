---
title: 'Installing pip on Linux'
slug: 'pip-linux'
summary: 'The right way, for I\'ve been doing it wrong the whole time...'
date: 'December 14, 2018'
feature-image: /assets/installing-pip.png
---
<p><img src="/assets/installing-pip.png" alt="Installing pip on Ubuntu" style="width: 100em" /></p>

<p>So, have you been in a situation where you are not any more new to python, building stuff is easy, but not quite up to your expectations...</p>

<p>Then you end up discoverng the beautiful <a href="https://pypi.org">Python Package Index</a>, and you just fall in love with how easy it is to install all kinds of packages, be it Django, Flask or even TensorFlow! Well, that's exactly the situation I was in after 4 years of using python!</p>

<p>I have installed pip and done many wonderful things with it... That is until I found out that I was doing it all wrong.</p>

<p>Installing pip wiht <code>sudo apt install python3-pip</code> was, to say the least, easy. But as is with anything in <a href="http://t.me/BEARlySec">InfoSec</a> not the safe or the correct way, also not how I'd prefer and adhere to now... I hope you don't make the same mistake, so I bring to you, this pretty short gist of what I have learnt from my experience with PIP, and python packages in general!</p>

<h2 id="installing-pip">Installing pip</h2>

<p>The way most of my friends install pip usually goes like this...</p>

<div class="highlight">
	<pre style="color:#f8f8f2;background-color:#272822;-moz-tab-size:4;-o-tab-size:4;tab-size:4">
<code class="language-bash" data-lang="bash">
sudo apt install python3-pip
</code>
	</pre>
</div>

<p>Yes, they all use pip3 and that's what I do too... This tutorial considers the same version. Just grow up, python2.7 is almost dead!</p>

<p>But why do that when there's a much simpler way to do the same?</p>
                
<div class="highlight">
	<pre style="color:#f8f8f2;background-color:#272822;-moz-tab-size:4;-o-tab-size:4;tab-size:4">
<code class="language-bash" data-lang="bash">
wget https://bootstrap.pypa.io/get-pip.py
python3 get-pip.py --user
</code>
	</pre>
</div>

<p>And yes, I used the <code>--user</code> flag because for most usecases, I am fed up of running pip in sudo. There's the off chance that a hacker might take my carelessness as a boon to run havoc on my machine. Believe me, even linux machines are not safe from the tyrany that is malware. What I do next is make sure that my computer notices that I have pip insatlled by adding it to the PATH environment variable.</p>

<div class="highlight">
	<pre style="color:#f8f8f2;background-color:#272822;-moz-tab-size:4;-o-tab-size:4;tab-size:4">
<code class="language-bash" data-lang="bash">
PATH<span style="color:#f92672">=</span>$HOME/.local/bin:$PATH
</code>
	</pre>
</div>

<p>You can also add the line to your .bash(/zsh/fish)rc file so that the pip command works without your having to use <code>python -m pip -</code></p>

<p><a href="https://www.reddit.com/r/Python/comments/aa7yn5/i_see_a_lot_of_people_installing_pip_in_an/">Comments</a></p>
