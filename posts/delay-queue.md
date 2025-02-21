---
title: 'Delay Queues, an analogous explainer'
slug: 'delay-queue'
summary: 'Documenting the analogy I used during a Kerala Rustaceans meetup to explain my use of DelayQueue '
date: 'January 09, 2023'
---

## The Problem
Think of a Bus travelling between two stops, waiting at the first, there are two naive approaches for the driver to decide on when to start the journey: 
1. The bus starts the moment there is one rider, or 
2. When it is full.

While in case 1, the passenger does not have to wait, buses are plying at too high a frequency and there is very low occupancy. Consider these buses as network traffic and you see how very high frequency of network traffic might be a problem. 

The problem with case 2 though is that depending on the size of bus and other factors, the first passenger to board bus has to wait for the last vacant seat to also be filled before the bus departs, and sometimes the bus may just be left hanging when no such passenger ever decides to take the bus. This may lead to data loss when considering the network equivalent of the problem, as the bus is stranded at the stop for ever. 

Case 2 also happens to be status quo, how we were handling batching in uplink. Each batch would wait to get filled before being flushed, which lead to certain batches never getting dispatched. We needed a solution.

## The Solution
There maybe a few solutions to the problem, probably the simplest is to think of a clock based system, every time the bus is filled, or the clock hits 1hr mark, a bus departs, but then there is a problem in this situation:
> The bus gets filled and leaves the stop a few minutes before the hourly mark(12PM, 1AM, etc.), and even if there aren't that many passengers in the next bus, it has to leave at that designated point, no matter what. This leads to inefficient usage of network resources.

This is where [`DelayQueue`](https://docs.rs/tokio-util/latest/tokio_util/time/delay_queue/struct.DelayQueue.html) allows us to write dynamic schedules that work as follows in the bus analogy: 
> The bus leaves after the first passenger has been waiting for an hour or when it gets filled, whichever comes first. This means that the first passenger always reaches the final stop in not more than +1hr of the expected journey time, while the bus is also filled to the max possible while following the first constraint.

## Further Reading
- Code that implements the solution discussed above to improve batching: [bytebeamio/uplink#20](https://github.com/bytebeamio/uplink/pull/20)
- Similar PR that uses `DelayQueue` to dynamically schedule a task which was being done a bit more manually before hand: [bytebeamio/uplink#104](https://github.com/bytebeamio/uplink/pull/104)
