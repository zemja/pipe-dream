# Pipe Dream

A Rust graphical front-end for Nushell, to visually follow your thought process by evaluating pipelines step by step.

## What is this?

At the moment, this program is very basic.

Essentially, I understand a shell as a means of instructing my computer. You can also write re-usable programs, but first and foremost it's to tell your computer to do one specific thing, exactly one time. Pipe Dream is a graphical shell, based on Nushell, designed to make this as slick and ergonomic as possible.

How? When you run a command, a new buffer is created, showing its output. (I mean "buffer" in the sense of Vim/Emacs.) If there is something in the buffer when you run a command, the buffer's contents are piped into it, and replaced with the output. Thus, by writing each stage in your pipeline step-by-step, you can whittle down the output of the first command to get what you want, in a nice visual way, without having to repeatedly evaluate the whole pipeline.

I imagine having a thumbnail generated for each stage in the pipeline at the top of the screen, and navigating backwards/forwards in the pipeline with the keyboard. Given this, and the ability to edit each stage again, I think you'd have a nice intuitive way of instructing your computer, which accurately represents your thought process.

- This program is inspired by [the playground window of Pharo](https://github.com/pharo-open-documentation/pharo-wiki/blob/master/General/Playground.md), specifically that `self` section at the bottom.
- See [the to-do list](notes/todo.org) for some information on future plans (though it's not a great description).

## Example workflow

Suppose you want to look recursively in the current directory, for whichever text file contains the longest line, out of boredom. You could do it in Bash, but you probably don't know how to do it off the top of your head, so it'll take a bit of thinking and experimenting. In Pipe Dream, I imagine the workflow like:

`ls **/*`

Then you see everything under the current directory. You only want the files, so you type:

`where type == file`

And you have just the files. Now you notice that many of them are binary files. So, you type:

`each { |it| {name: $it.name, type: (file $it.name)} }`

So that you can filter them by file type. But, you realise that perhaps the MIME type would be more robust. So you go back, and change it to:

`each { |it| {name: $it.name, type: (file --mime $it.name)} }`

And it filters the output again, running ONLY that stage of the pipeline, saving you lots of time since it's not recursively listing the directory again. Now you have all the files and their MIME types, you get just the text files:

`where type =~ text/`

Now you want to find the length of the longest line for each of them. So, just to see what happens, you do:

`each { |it| {name: $it.name, longest: (open --raw $it.name | lines)} }`

You get a lot of `[list XYZ items]`. To investigate, you run:

`get longest.0`

To see what's there. You see that it's a list of every line in the first file. You go back to the previous stage, and refine it with:

`each { |it| {name: $it.name, longest: ($it.longest | str length | math max)} }`

Now it's a list of names of text files, and the length of their longest line. Now simply:

`sort-by longest`

And you'll have your answer. Or, you probably just click the table heading in the UI to sort by that column. (If you must, you can do `last`, then you'll **really** have your answer.)

So, the highlights:

- Easy and lazy.

  This is not the most efficient way of doing things, but that's not the point. You'll only ever want to run this once, and now that you have your answer, you don't really need the code you typed ever again.
- Saves time avoiding having to run the whole pipeline again every time.

  Plus, it improves the irritating workflow where you run a command a second time, adding `| less` if the output is long. Sure, you can scroll back, but only if your prompt really stands out visually so you can tell where the command started in the output. In Pipe Dream, each command is in its own graphical window with a scroll bar, so output is already "paged".

## Brainstorm

Here's a paste of a brainstorm I had with myself when I first came up with the idea. Perhaps it will help you understand the point of this program.

> It seems like Nushell is missing a cool feature, which is to run a command on the last output, to further refine it. I don't think it stores the output of the last command in a variable or anything, so you can't do that.
>
>How about taking this to the extreme? I imagine having a pipeline by having a graphical window with a prompt at the bottom. You run a command in the bottom prompt, and the window above fills with text (the output). (Actually it needn't be text, you can display the structured data output by Nushell in a cooler way probably, but whatever.) Then you type another command at the bottom, and the previous command is piped into it, and the output fills the window now. So rather than running a command like `ls /dir/ | where type == file`, you start with an empty screen, type `ls dir` to get the files, then `where type == file` to pare that down. So you construct a pipeline without using the `|` character, by just typing each individual component.
>
>I imagine having a thumbnail generated for each stage in the pipeline at the top of the screen, and you can navigate backwards/forwards in the pipeline with `C-b` and `C-f`. You can run a new command, to re-do the pipeline from that point. Perhaps all the stages after that are run again at that point and have their "buffers" updated, or perhaps it should just delete all the stages after that. Who knows.
>
>You can go backwards and forwards in the history of pipelines with `C-n` and `C-p`. There's always an empty pipeline at the bottom of the list, so to start a whole new pipeline, just press `C-n`. To delete all your pipelines (to save RAM probably), press `C-x`.
>
>You should probably also be able to type a pipeline in the prompt and have it still work. So you can still type `ls dir | where type == file` if you're sure that's what you want, to save time.
>
>You should probably have some option to copy your whole pipeline as text, so it can be put in a script or something. Or, you could copy your entire command history as a script actually, since it's just a series of pipelines in Nushell - just like a script. (Maybe you could even open a script in this program? Wow!)
>
>The point of this is that most commands that your run are only ever run once, so everything doesn't need to be written like a script. You should be able to graphically whittle data down to what you want.
>
>There should also be a way to start a new pipeline with the contents of the clipboard. Perhaps pressing `C-v` with the output section focused should do that.
>
>Does this have to be a graphical front-end? Is there any way you could integrate this into Emacs? 🙂
>
>- How to read from standard input? It will be hard to enter your `sudo` password like this.
>- Is there any reason why it needs to be Nushell only?
>
>  Perhaps you could support any other shell by just treating the type of the output of every command as `string`. This would be particularly important if you do integrate this in Emacs, in which case many people won't be using Nu.
>
>**A great name for this program would be "Pipe Dream".** 🙂
