---
title: 'Building raex, The Technical Take'
slug: 'vyuham-tech'
summary: 'Quite possibly the most complex programming project I have undertaken...'
date: 'June 14, 2021'
---
So, after the publication of my non-technical blog about [Project Vyuha](https://github.com/vyuham) I recieved a few requests to do a technical deepdive of the project, what it was built for and the reasons behind the decisions that we made during it's development. This is my attempt at doing exactly that. Please forgive me for any wrong opinion, I am a novice and have very little experience in the field out of this project.

Let's start with the system design, as I have already mentioned, I am a student, doing my bachelors in Computer Engineering, and this is the first serious academically focused project I have worked on. This also means that I come in with little to no understanding of the state of the art for doing such stuff as distributed computing. I understand that what we need is a scheduler that know's the state of the entire system, what operations are running where, and that is why we chose to use the replicated statemachine approach to doing things. This is where our choice of the raft consensus algorithm comes into play, we chose to go with it because of it's much touted simplicity and robustness.
