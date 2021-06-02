# Blog

| # | Title 									                                        | Date  			|
| - |:---------------------------------------------------------------------------------:| -----------------:|
| 5 | [A Summer of Code with TiKV](posts/gsoc-2020.md)                                  | August 25, 2020   |
| 4 | [Building a Universal Parser, Improving Hydrus and Demo](posts/gsoc-proposal.md)	    | April 5, 2019		| 
| 3 | [Awesome new Python learnings](posts/python-learnings.md)				            | March 8, 2019		|
| 2 | [How I moved OpenSalve from using Virtualenv to Pipenv](posts/opensalve-pipenv.md)| January 3, 2019 	|
| 1 | [Installing pip on Linux](posts/pip-linux.md)					                | December 14, 2018	|

## Inspiration
You may be wondering why this blog is a GitHub repo and not a webpage, well the simple answer is I was fedup with static site generators, and the need to move away from the absurdities of these brought me to a world where I can write my thoughts without having to think a lot about how I store them :D

## Code Structure

The repo contains rust code to generate a `_posts.json` file that can be used to render on front-end of choice. I intend to write a CI flow to generate the same, maybe even generate single `<slug>.json` files and index them in the `_posts.json` if the number of posts are enough to demand such a work-around.
