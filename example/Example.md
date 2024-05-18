# MarkDowncuments example

|Rev    |Date       |Author     |Notes          |
|---    |---        |---        |---            |
|1      |2024/03/19 |R. Porpora |first release  |


## This is a second level title

If you want to customize appearence and layout, you can write raw html and css directly into the markdown document.

<p class="custom-styling">This paragraph is custom styled with html.<br>Its css code is just below this pargraph, but it is hidden in the pdf export.</p>

<style>
.custom-styling {
  color: red;
  font-family: "Lucida Console", "Courier New", monospace;
  text-align: center;
  }
</style>


### Third-level Title

Inline stuff:

This is a test text paragraph in **Markdown**. You can use *italics*, ~~strikethrough~~, and even `inline code`.
This sentence is rendered on the same line of the previous one.  
This one breaks, instead, because there is a double space before it.

Lists:

- Unordered list with "-"
  + Item one with "+"
  + Item two with "+"
    - Nested item
    - [ ] Uncompleted to do list element
    - [x] Completed to do list element

1. Ordered list
2. Second item
   1. Sublist
   2. Another item
3. Third item

You can also insert [links](https://www.example.com).


There's a page separator (***) below this line.

***

You can insert images like this:

![Here's the optional alt text](img/imgtest.jpg)

Tables:

|Header 1	|Header 2	|Header 3	|
|---		|---		|---		|
|Cell 1		|Cell 2		|Cell 3		|
|Cell 4		|Cell 5		|Cell 6		|

Quote:

> This is a block quote.


***

Example code:

```rust
fn main() {
    println!("Hello, world!");
}
```

Math formulas (rendered with MathJax):

$$
\int_{-\infty}^{\infty} e^{-x^2} \, dx = \sqrt{\pi}
$$