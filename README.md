# Blog

| # | Title 									                                        | Date  			|
| - |:---------------------------------------------------------------------------------:| -----------------:|
| 5 | [A Summer of Code with TiKV](posts/gsoc-2020.md)                                  | August 25, 2020   |
| 4 | [Building a Universal Parser, Improving Hydrus and Demo](posts/pip-linux.md)	    | April 5, 2019		| 
| 3 | [Awesome new Python learnings](posts/opensalve-pipenv.md)				            | March 8, 2019		|
| 2 | [How I moved OpenSalve from using Virtualenv to Pipenv](posts/python-learnings.md)| January 3, 2019 	|
| 1 | [Installing pip on Linux](posts/gsoc-proposal.md)					                | December 14, 2018	|


# Code Structure

The repo contains rust code to generate a `_posts.json` file that can be used to render on front-end of choice. I intend to write a CI flow to generate the same, maybe even generate single `<slug>.json` files and index them in the `_posts.json` if the number of posts are enough to demand such a work-around.
