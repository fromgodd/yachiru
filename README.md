# Yachiru.rs

## ✨✨ Just another static site generator optimized for microblogging/articles

<p align="center"><img src="https://github.com/fromgodd/yachiru/assets/97128346/94df95e7-aa67-4363-9592-f4acd3b0915a" alt="logoYachiru" width="55%"></p>

---

## Introduction

Yachiru.rs is a static site generator designed for microblogging and managing articles. It simplifies the process of creating and maintaining a website by generating HTML pages from Markdown content. If you're familiar with Jekyll, you'll find Yachiru.rs to be a friendly tool for your static site needs.

## Documentation

🚧 **Work in Progress** 🚧

Documentation is currently a work in progress, and I am continuously improving it. Would be glad for any kind of PR's and critics. Feel free to open issues if you notices some bugs and etc.
Right now you can start experimenting with the `/content` folder to manage the content for your future website. Write your articles and microblog posts in Markdown format; if you've used Jekyll before, you'll find it a breeze!

## Quick Start

To get started with Yachiru.rs, follow these steps:

1. Clone this repository to your local machine.

2. Open your terminal and navigate to the project's root directory.

3. Edit the Markdown files in the `/content` folder to create your articles and microblog posts.

4. Run the following command to build your website:

   ```sh
   cargo run
    ```

## To-Do List

Plans and current work:

- [ ] Complete and improve documentation.
- [ ] Experimental LaTeX render (Researching)
- [ ] ADD TABLE SUPPORT!!! TABLES WON'T CONVERT TO HTML
- [ ] Develop a user-friendly command-line interface (CLI), CLI "make", "remove", "new foobar --article" and etc. Need to work on arguments and make easier.
- [ ] Enhance the website template and styling.
- [ ] Implement additional features based on user feedback.
- [ ] Syntax highligting support. Comrak supports syntax highlighting through syntect, so it should be easy to support. Some links
https://docs.rs/comrak/latest/comrak/fn.markdown_to_html_with_plugins.html
https://docs.rs/comrak/latest/comrak/plugins/syntect/struct.SyntectAdapter.html

