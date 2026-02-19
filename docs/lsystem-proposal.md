# ITCS 4102-5102 Term Project Proposal

Project Title: Procedural Tree Generation with L-Systems in Rust

Programming Language: Rust

Team Members: [Your names here]

## Project Idea

This project explores using L-systems to procedurally generate natural-looking trees in Rust. An L-system is a rewriting system that applies production rules to a starting symbol over multiple iterations, producing strings that can be interpreted as drawing instructions. By mapping characters to actions like moving forward, rotating, and branching, we can produce surprisingly organic and varied tree structures from a small set of simple rules.

The program will take an L-system grammar as input, expand it through several iterations, and then interpret the resulting string to draw a tree. We plan to render the output either as ASCII art in the terminal or in a 2D graphics window using a crate like minifb or macroquad. The project will let users tweak parameters such as branch angle, iteration depth, and segment length to see how the generated trees change, giving a hands-on look at how a formal grammar can produce complex natural-looking structures from simple rules.

## Papers to Read

- Prusinkiewicz, P. and Lindenmayer, A. (1990). The Algorithmic Beauty of Plants. Springer-Verlag.
- Prusinkiewicz, P. (1986). Graphical applications of L-systems. Proceedings of Graphics Interface 86.
