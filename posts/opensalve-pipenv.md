---
title: 'How I moved OpenSalve from using Virtualenv to Pipenv'
slug: 'opensalve-pipenv'
summary: 'Not being able to make things happen at the first try, I was frustrated with virtualenv...'
date: 'January 3, 2019'
feature-image: /move-pipenv-opensalve.png
---
<p>This semester in college, we had a Social Problems Solving Hackathon -<a href="http://hackfortomorrow.excelmec.org">HackForTomorrow</a>- in which my team was contesting within the topic category of "Disaster Management". We were pushed by the emotion of fear brewing among the people of our locality towards what eventually turned out to be a horrendous disaster. Though we couldn"t get it all together till after the event, we sure did get into the final stages of the parent competition a Social initiative of the College fest by the name of <a href="http://ibeto.excelmec.org">iBeTo</a> with our project <a href="https://github.com/subins2000/OpenSalve">OpenSalve</a>.</p>

<p>
	<img src="/assets/move-pipenv-opensalve.png" style="width: 100em" alt="Moving OpenSalve To Pipenv" />
</p>

<p>This though is not going to be the story of how we built the project. It is a log of how I moved the project from using a virtualenv setup to one that"s way easier to setup as is shown in the PR <a href="https://github.com/subins2000/OpenSalve/pull/9"><strong>subins2000/OpenSalve#9</strong>: virtualenv ~> pipenv and other minor changes</a>. In addition, now out project also adheres to most standards set by <a href="https://python.org">Python.org</a>!</p>

<h2 id="reasons">Reasons</h2>

<p>I had started work on the project at the behest of <a href="http://subinsb.com">Subin Siby</a>(<a href="https://github.com/subins2000">@subins2000</a>) for building a <strong>blockchain</strong> to validate transactions between volunteers and the govt. I did not work on the same as I had strongly disagreed on two fronts, first that I would not work on a centralized blockchain as its can instead be implemented as a database, albeit more easily, but also because I was unable to build a system to make things both decentralized and reliable.</p>

<p>
        <div style="position: relative; padding-bottom: 56.25%; height: 0; overflow: hidden;">
                <iframe src="//www.youtube.com/embed/GBQAKldqgZs" style="position: absolute; top: 0; left: 0; width: 100%; height: 100%; border:0;" allowfullscreen title="YouTube Video"></iframe>
        </div>
        <strong>Kenneth Reitz - Pipenv: The Future of Python Dependency Management - PyCon 2018</strong>
</p>

<p>But in parallel the setup procedure for the development of this project wasn"t helping either. At this point the project was using <a href="https://virtualenvwrapper.readthedocs.io/en/latest/"><em>virtualenvwrapper</em></a> to do most of it"s environment setup. I agree, it is a great tool, and as such <em>Virtualenv</em> is also provided by the same people as is Pip and Pipenv, but I was also sure the project would greatly benefit from the change. Pipenv as such provides the abstraction of the setup process and also allows for a shell to be setup within the environment.</p>

<blockquote>
        <p>Two targets, one tool! Who wouldn"t fall for such a charming product?</p>
</blockquote>

<p>- <em>Devdutt Shenoi</em> (I know, I"m quoting myself :P)</p>

<h2 id="changes">Changes</h2>

<p>Prior to the change, the process for install was pretty complex, here is the comand one needs to execute, to get started:</p>
<div class="highlight">
        <pre style="color:#f8f8f2;background-color:#272822;-moz-tab-size:4;-o-tab-size:4;tab-size:4">
<code class="language-bash" data-lang="bash">
export VIRTUALENVWRAPPER_PYTHON<span style="color:#f92672">=</span>/usr/bin/python3
source ~/.local/bin/virtualenvwrapper.sh <span style="color:#75715e"># Better add this to .bashrc</span>
mkvirtualenv opensalve
rm $VIRTUAL_ENV/bin/postactivate
ln -s <span style="color:#e6db74"></span>realpath .env/postactivate<span style="color:#e6db74"></span> $VIRTUAL_ENV/bin/postactivate
</code>
        </pre>
</div>
                
<p>Where as now, its a bit less complicated <strong>;-;</strong></p>
<div class="highlight">
        <pre style="color:#f8f8f2;background-color:#272822;-moz-tab-size:4;-o-tab-size:4;tab-size:4">
<code class="language-bash" data-lang="bash">pipenv install</code>
        </pre>
</div>
                
<p>I don"t kid you, that"s it. Now you know why saying <code>its less complicated</code> is an understatement. XD</p>

<blockquote>
        <p>Hashes are used everywhere, always. Security. Automatically expose security vulnerabilities.</p>
</blockquote>

<p>- <a href="https://github.com/pypa/pipenv#pipenv-python-development-workflow-for-humans">README.md</a></p>

<p>While you might be asking "where is the process for linking the environment variables file?", let me console you with the fact that all these variables can now be housed within the <code>.env</code> file. So in our case <em>.env</em> looks like this:</p>
<div class="highlight">
        <pre style="color:#f8f8f2;background-color:#272822;-moz-tab-size:4;-o-tab-size:4;tab-size:4">
<code class="language-python" data-lang="python">
DJANGO_SECRET_KEY<span style="color:#f92672">=</span><span style="color:#e6db74">'key here'</span>
</code>
	</pre>
</div>

<p>Whereas in the past this used to be done either manually, or as is pointed out in the above code within a file <code>.env/postactivate</code> which looked like:</p>
<div class="highlight">
        <pre style="color:#f8f8f2;background-color:#272822;-moz-tab-size:4;-o-tab-size:4;tab-size:4">
<code class="language-bash" data-lang="bash"><span style="color:#75715e"># Generate using https://gist.github.com/ndarville/3452907#gistcomment-1172571</span>
export DJANGO_SECRET_KEY<span style="color:#f92672">=</span><span style="color:#e6db74">'key here'</span>
</code>
        </pre>
</div>

<p>Even the requirements.txt file is now retired in favour of a more <strong>"Strict"</strong> <code>Pipfile</code> and <code>Pipfile.lock</code> combo, much like there exists within node/npm files that define dependencies and their requierments in either the <em>development</em> or <em>production</em> stages.</p>

<p><strong>Conclusion:</strong> Pipenv is the tool that every Python developer must learn, we finally are in the age of better code. I leave you with a quote from <strong>Jannis Leidel</strong>, <em>former pip maintainer</em>.</p>

<blockquote>
        <dl>
                <dd>Pipenv is the porcelain I always wanted to build for pip. It fits my brain and mostly replaces virtualenvwrapper and manual pip calls for me. Use it.<br></dd>
        </dl>
</blockquote>

<p>
	<a href="https://pipenv.readthedocs.io/en/latest/">Pipenv.org</a> • <a href="https://github.com/de-sh/dev.log/issues/1">Comments</a>
</p>
