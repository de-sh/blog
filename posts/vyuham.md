---
title: 'Construction Ahead, Hard Hats Required'
slug: 'vyuham'
summary: 'As an undergrad, the last four years have been a journey of realization and final year was no less...'
date: 'June 14, 2021'
---
Being in my final year of college, I wanted to build something interesting and learn a lot in that journey. We ended up doing some really awesome stuff, but might have not been able to meet all our ambitious goals. This is my telling of the [Project Vyhua](https://github.com/vyuham) story, the name for which comes from the Sanskrit word for "battle formation". Given how the goal we set out to achieve was in the direction of learning how multiple computers can be organized like troops going to war against a single enemy. This was our way of learning distributed systems theory and for me to put into practice some really interesting learnings I had gathered from my work with the folks at the [TiKV project](https://github.com/tikv).

### Figures and plans
We planned to write a distributed process scheduler with [raft-consensus](https://raft.github.io), but as of now, we seem to have failed in implementing it. We were doomed you might say, but I'd disagree. As a team, we have achieved something that I like to think our peers would truly be jealous of. We learned to work as a team, trying to build something ambitious, which everyone of us was apprehensive about not be able to ship in time. And while we were at it, we also did some stuff that truly amazed us, let me recount them here.


### Raft Consensus
Having read about raft, I wanted to try implementing an idea that had popped into mind. We were at the start of our penultimate year of undergrad, and as final year students do, we were supposed to submit a project by the end of the year. As I was out of ideas for more interesting stuff and it only seemed to be becoming the more sensible one to work on, we chose to combine our interests in some random ways, to do something that we'd like to write home about. We were at home ofcourse, this is during the pandemic, what I mean to say is that we were trying to go out with a bang, but I digress. The idea came to take shape at one of initial brainstorming sessions as follows. We had no guidance outside our own research, so any misconceptions are our own, or of random people on the internet.

![project design](https://raw.githubusercontent.com/vyuham/raex/main/docs/project_design.svg)

We were also pretty much doing more design at this stage than coding, and we probably started that stage of developments really late. As is common in college, most of us had a hard time dealing with procrastination, and we still do, maybe we'd be world's ahead without it, but then wouldn't everyone? Here is a prictorial representation of what we meant to implement with raft-consensus.

![raft scheduler](https://raw.githubusercontent.com/vyuham/raft-scheduler/main/docs/scheduler.svg)

Whatever, we had a fun time planning and I got to do some really cool diagrams. We were heading to the implementation stage, slowly, but surely. We tried to start out with making something in python, and for reasons that we still don't know, we didn't find it right for what we were trying to do. We were faced with a choice, I feel like this needs some explanation, so here goes.

### Rust? Maybe let's not...
So this is a recurring theme, I was insistent we build our project with [rust-lang](https://rust-lang.org) and I like to think we had some really big achievements because of this. Could we have achieved better results with "language X"? Maybe, but then no one can be sure. I have to announce my clear bias here, I am probably in love with the language, and I sure do love the community that comes with it. I am not the best programmer, but rust has made me much better as one and I find myself thinking in terms of systems more often now, about the inputs and the flows and the outputs, because of it. Rust has been just a breeze to work with, and I think my team would bear testimony.

Though there was some initial apprehensions, it's not always that you consider shifting the way you think about programming, we hit the ground running. Around September, I started work on a distributed memory store module - [dstore](https://vyuham.github.io/dstore) - to emulate the way [FAASM](https://lsds.doc.ic.ac.uk/projects/faasm), a project we were exposed to through their [USENIX presentation](https://lsds.doc.ic.ac.uk/sites/default/files/faasm-atc20.pdf), does memory access and placement.

### Async Hell() -> Future<Proof> // Not real code :P
Working on the dstore componenet was really just reading up on a lot of stuff that was new to me. I had never written or thought about async code and hence it posed a challenge, but while there were [a few bugs](https://github.com/vyuham/dstore/commit/b49ab40b223aaea4f3449106323be1df0152457b) [in the code](https://github.com/vyuham/dstore/commit/1fc1890c357e25b628a479a67c98ebd11a9fc0ea), we were able to fix them in decent time. Another challenge I faced was learning about RPCs, pursuing gRPC made it easier, and I would surely owe a lot of thanks to the people who built [Tonic](https://docs.rs/tonic/) and [Tokio](https://tokio.rs), for they made it a literal walk in the park for us. I also have a lot of praise for the folks who wrote up all the wonderful documentation, the [Tokio tutorial](https://tokio.rs/tokio/tutorial) will alway remain a turning point in my journey as a developer.

My understanding of how systems work and how large projects get built changed a lot in the course of this project. We were setting up con-calls to decide on deadlines we'd set to deliver our part, 2 hour long streams for pair programming and most importantly some really deep and thoughtful brainstorming sessions to figure out what we were doing wrong. And one result from this exercise was [rtrcs](https://github.com/vyuham/rtrcrs), a ray-tracer we built in a few weeks, by reading the book [Ray Tracing in One Weekend](https://https://raytracing.github.io/books/RayTracingInOneWeekend.html) by Peter Shirley.

### Build rtrcrs and teaching rust
Both my teammates [Jerry](https://github.com/Firebreath1001) and [Bharath](https://github.com/bkp31415), having had very little prior exposure to code at the systems level, took little to no time getting into tune with using the sample C++ code and their understanding of the book, to write the rust code for this component. I had to step in only [once in a weeks or so](https://github.com/vyuham/rtrcrs/commits/parallelize) to help them with the complex stuff, but this experience has changed how I percieved rust being complex and intimidating for beginners and I think we need to review that notion and put it to rest.

I just find it amusing, how we ended up in a place much closer to where we were heading than we earlier thought possible. Now with a few days left for the final submissions, I am considering cheating on rust-lang to write some golang :P. Maybe an FFI bridge between the two would be the answer, anything that could just help to bring this project to completion and to put the beast to sleep would be fine. Till then... Keep safe, keep dreaming :D