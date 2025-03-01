---
title: 'Building a Universal Parser, Improving Hydrus and Demo'
slug: 'gsoc-proposal'
summary: 'Proposal to Hydra Ecosystem(HTTP-APIs), by Devdutt Shenoi(de-sh)'
date: 'April 5, 2019'
feature-image: /assets/federated-hydrus-real-world.png
---
<h2 id="introduction">Introduction</h2>

<p>The Hydrus server and it's associated packages as of now are at a stage where they apply most of the Hydra draft documentation framework, but the suite still needs a lot of work to make it usable in the production environment.</p>

<p>The proposed project is aimed at adding functionalities to hydrus and the hydra-openapi parser. As a main focus, building support for various other APIDoc specs/languages to the HydraAPI spec documentation for use by hydrus or other components of the package, adding HTTP/2 and asyncio support to Hydrus as well as making necessary changes to the core package and updating it to use the latest specs of Hydra. I would also like to contribute by adding to the general Hydra Ecosystem of packages, making updates to hydrus as pointed down below.</p>

<h2 id="project-goals">Project Goals</h2>

<ul>
<li>The hydra-openapi-parser only supports OpenAPI 2.0 features, that needs to change as the latest version of the OAS spec(<em>v3.0</em>) is brought in with support for new features in the updated Documentation format for conversion into HydraDoc</li>
<li>Support can also be further extended to RAML within the parser. This has to be best achieved in a fashion that makes the parser framework agnostic.</li>
<li>Hydra Draft has seen some changes that have not been updated onto Hydrus, it is a goal to update hydra-python-core to implement these changes</li>
<li>Implementation of the HTTP/2 protocol by porting the application from flask to quart, making use of asyncio.</li>
<li>Building an example real-world-app implementation making use of all of the Hydra Ecosystem of packages to demonstrate its capabilities.</li>
</ul>

<h2 id="implementation-plans">Implementation Plans</h2>

<p>Some means through which I propose to achieve my project goals include, but are not limited to:
- Implement the OpenAPI-Core validator as pointed out in <a href="https://github.com/HTTP-APIs/hydra-openapi-parser/issues/6">hydra-openapi-core#6</a>.
Thus ensuring that all OAS specifications passed to the parser are validated before parsing.
- Make use of the idea behind  api-doc-parser to build a better cross Spec parser so that even RAML(as pointed out in <em><a href="https://github.com/api-platform/api-doc-parser">hydrus#285</a></em> by <strong><a href="https://github.com/soderluk">@soderluk</a></strong>) and other frameworks are supported. Add support for version 3.x of OAS(currently at Swagger 2.0).
- Update <em><a href="https://github.com/HTTP-APIs/hydra-python-core/issues/15">hydra-python-core(#15)</a></em> as pointed out by <strong><a href="https://github.com/chrizandr">@chrizandr</a></strong>, to make use of a current version of the Hydra documentation specification.</p>

<p>All of the above will be achieved in the first phase of the project. In the second phase I’d like to implement the following:
- Build a network of federated servers that implement a social media blogging platform similar in UI to Medium and having interoperability functionalities similar to that seen in the Twitter clone <strong><a href="https://mastadon.social/">mastadon.social</a></strong> that allows it to be a part of a larger social media chain of servers with feeds that are cross instances and distributed in nature. This implementation utilises many features of Hydrus for the purpose of interoperability and federation of the service among the various instances that are basically hydrus based servers.</p>

<p>Also in the second phase simple fixes and other additions I’d like to make in hydrus and its associated packages are listed below:
- Add a simple <em><a href="https://github.com/HTTP-APIs/hydrus/issues/167"><code>--load</code> option to the Hydrus CLI(hydrus#167)</a></em> to initialize the server with data in JSON/CSV format
- Add support for <em><a href="https://github.com/HTTP-APIs/hydrus/issues/300">Push notifications(hydrus#300)</a></em> as suggested by <strong><a href="https://github.com/Mec-iS">@Mec-iS</a></strong>. This can be achieved with HTTP/2 support, also making the flask implementation currently in use within hydrus make use of <em><a href="https://github.com/HTTP-APIs/hydrus/issues/372">asyncio using quart(hydrus#372)</a></em> to call the Flask API.
- To build on this a network making use of a Merkle-DAG to maintain a ledger of changes to the data can also be implemented to keep track of data changes and notify each and every instance of a change. This helps client from making use of stale data.</p>

<h2 id="adding-raml-and-improving-open-api-v3-support">Adding RAML and improving Open API (v3) Support</h2>

<p>The <strong><a href="https://openapis.org">OpenAPI Specification(OAS)</a></strong> as a documentation framework for REST APIs, allows both <em>human and computer based discovery</em> and understanding of the capabilities of a service without requiring access to its implementation or any specific human readable documentation. Hydra and OAS can be considered as sibling specifications with similar objectives, but implemented differently. OAS is quite extensive whereas hydra is lightweight. OAS has a sizable user-base, thus requiring users to port their documentation to Hydra is detrimental to the project. Similarly RAML is a documentation framework that needs to also be supported due to its.</p>

<p>We need to focus on improving the hydra-openapi-parser to make it full-featured, allowing users of OpenAPI, RAML or any other supported API documentation framework to experiment with and to use Hydrus in their projects, thus making the barrier to entry easier for those wanting to use it.</p>

<h2 id="improving-hydrus">Improving Hydrus</h2>

<p>A manner of adding data to the Hydrus database without making <strong>PUT</strong> requests, instead using a simple <code>--load</code> option to load the data from a simple JSON/YAML/CSV
file is to be added to the Hydrus CLI.  Moving to quart over flask calls where possible to implement asyncio can make Hydrus better with handling load as it will move to making  use of the event loop method instead of the current sequential flow of control within Flask.</p>

<p>This will also help implement HTTP/2 and make it possible to create a push notification mechanism to notify clients of changes made to data from one of the client nodes. This network must be signalled and managed from the centralized Hydrus instance that implements a checking mechanism similar to that in git servers utilising a simple Merkle DAG.</p>

<h2 id="hydrus-real-world-app">Hydrus Real World App</h2>

<p>The future of the web is made with <a href="https://en.wikipedia.org/wiki/Distributed_social_network">federated/distributed networks</a> instead of a centralized approach to data storage and retrieval. A new suite of apps are on the rise that make use of network of different servers that are running instances of the same kind. I propose to build a Real World App such as the one suggested in the <a href="https://github.com/HTTP-APIs/hydrus-real-world-app">hydrus-real-world-app#1</a> that implements a federated clone of the Medium blogging platform.</p>

<p><img src="/assets/federated-hydrus-real-world.png" alt="Federated Medium like blogging platform, Hydrus real-world" style="width: 100em"/></p>

<p>As is clear from the above diagram I would like to implement a distributed social media blogging platform similar to Medium. It will contain <strong>multiple instances of Hydra enabled servers</strong> that are interconnected through inter-instance communication APIs that allow for a quick and easy interconnect between the instances on the network. This is further augmented by the addition of a UI on top of hydrus based back-end that allows for the direct user interaction with service. The API can also be consumed by various other clients.</p>

<p>As an example, the user might be looking for a data object stored in instance C and is currently accessing the service through instance A. The service will look for the file first within itself and making use of the **Central list** find the instance C and provide the data. This can be helpful in cases where data is stored on an instance other than the current and needs to be accessed without leaving it through a hyperlink.</p>

<p>In a similar use case the data(eg: weather bot info) might be coming from an API that is accessing Instance B and the user might be availing this data through their own feed as a forecast. The federated network helps the user to make use of the network to make and edit the document the data(blog article) on one instance and to later access it from another where both can be just automated bots sharing documents such as scans or transcripts. The data remains human Understandable/Readable.</p>

<h2 id="schedule">Schedule</h2>

<p>GSoC spans a duration of almost 4 months, with an initial community bonding period continued by 12 weeks of work on the allotted project, thus I would schedule my contributions in such a way that I can make full use of the allotted time.</p>

<ol>
<li><p>Start Implementing changes as proposed on <strong>OpenAPI</strong> parser (May 6-27, community-bonding period)</p>

<ul>
<li>Add validation and other proposed components.</li>
<li>Separate the functionalities into different classes/modules.</li>
<li>Write documentation for the parser.</li>
<li>Unit testing and debugging the code.</li>
</ul></li>

<li><p>Plan and finalize the implementation of RAML~>Hydra parser. (Mid May, community-bonding period)</p></li>

<li><p>Implement plan for a <strong>RAML~>Hydra</strong> parser.(End May-Early June)</p>

<ul>
<li>Make use of learnt concepts from Simple RAML<~Hydra parser to build a RAML~>Hydra parser.</li>
<li>Run tests by converting RAML sample code to HydraDoc.</li>
</ul></li>

<li><p>Integrate the <strong>RAML</strong> and  <strong>OAS</strong> parsers into a single library. (Mid June)</p>

<ul>
<li>Make necessary changes to the parser to support both OAS and RAML.
<br /></li>
</ul>

<p><strong>NOTE:</strong> I will be writing my Semester ending exams during the period of Early-Mid June, but would like to continue contributing. Hence balancing off the load early.</p></li>

<li><p>Building the example <strong>Real World App</strong> with Hydrus in a federated manner</p>

<ul>
<li>Build facility to demonstrate interactions with multiple Hydrus instances and with Dynamic API Paths (June End)</li>
<li>Create an interactive Medium like UI (July)</li>
</ul></li>

<li><p>Federation of instances of the <strong>Real World App</strong> (July)</p>

<ul>
<li>Multiple instances connected together through Hydra API.</li>
<li>API end-points(paths) providing various capabilities.
<br /></li>
</ul>

<p>This is the hard part that I need to learn more about how to implement.</p></li>

<li><p>Implement <strong>HTTP/2</strong> by porting hydrus to Quart. (Early August)</p>

<ul>
<li>Make sure everything is working as is required with Hydrus.</li>
<li>Add push notification as noted.</li>
</ul></li>

<li><p>Test, debug and finish pending work (if any).</p></li>

<li><p>Submit (August 19)</p></li>
</ol>

<h2 id="about-me">About Me</h2>

<ul>
<li>I have been working on bringing various organisation at my college together to open-source their code, be it <a href="https://github.com/ieeemec/ieeemec/wiki">simple websites</a> or complex apps to solve <a href="https://github.com/projectnalanda">unique problems</a> they have. I have also spoken at a FOSS conference on subject matter.</li>
<li>I have been certified by NPTEL as a <a href="https://nptel.ac.in/noc/social_cert/noc17-cs28/NPTEL17CS28S1290480171005480.jpg">Gold medallist(Elite certificate)</a>) in a Python based programming course. Otherwise I have been using Python since school while real interest has only developed lately.</li>
<li>Last year I worked on a Python/Django web-app called <a href="https://github.com/IEEEKeralaSection/rescuekerala/graphs/contributors">rescuekerala</a>(my first real life collaborative work), which was built during the floods in Kerala, India. We didn't focus much on the APIs as we only did a single JSON data-dump. I had a real interest in working on the same due to which we built an iteration called <a href="http://github.com/subins2000/opensalve">OpenSalve</a> that made use of definite end-points instead.</li>
<li>I would love to work on a W3C related project. My dream career involves working with IETF like organisations and that is one reason why I am an IEEE/ISOC Student Member. I am still not really a good engineer, but I would like to make use of this opportunity and am willing to continue contributing to the project, even if not selected.</li>
</ul>

<p><strong>Email</strong>: devdutt@ieee.org</p>

<p><strong>Telegram</strong>: @devduttshenoi</p>

<p><strong>GitHub</strong>: @de-sh</p>

<p><strong>Country</strong>: India</p>

<p><strong>Time Zone</strong>: GMT +5:30</p>

<h4 id="update-6-may-2019-11-45">Update: 6 May 2019 11:45</h4>

<p>It was a refreshing experience to have written such a detailed proposal, sadly this project was <strong>not selected</strong> for a scholarship. I should have devoted more time and research, <em><a href="https://ktu.edu.in">University</a> timings</em> also don't match well with the expectations of GSoC, so not being selected was maybe a boon in itself. I hope to participate again in the next edition with this learning experience. Meanshile I hope to develop myself technically.</p>

<ul>
<li><strong><a href="https://summerofcode.withgoogle.com/organizations/6557492048297984/#projects">Selected Projects under Hydra Ecosystem</a></strong></li>
<li><strong><a href="https://summerofcode.withgoogle.com/projects/#5444359158235136">Aswin M Prabhu's Proposal</a></strong> that was selected, he is a friend.</li>
</ul>
