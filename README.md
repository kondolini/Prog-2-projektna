# Project for class Programiranje 2

## Project Overview

This server application facilitates communication between servers and generates various types of sequences. It enables real-time data exchange, making it ideal for distributed systems that require synchronized sequence generation and communication. The program supports multiple sequence generation methods and flexible network configurations.

## Running the Program

1. Open a terminal and use the **cd** command to navigate to the **register** directory.
2. Run the server with the command: **cargo run**
3. If you wish to initiate communication with other servers, open a new terminal, navigate to the **generator1** directory, and run: **cargo run 127.0.0.1 7878**
4. If you wish to communicate without the other servers, you can also use the file **main2.py**. Open another terminal, go to **generator1**, and run: **py main2.py**.
5. Follow the instructions displayed in the terminal for a user-friendly experience.

## Available sequences

**Arithmetic Sequence**
- **Description:** An arithmetic sequence that calculates terms by adding a fixed step to a starting value. The sequence takes two parameters: a starting value and a step, with each subsequent term being the sum of the start and a multiple of the step.
- **Parameters:** 2
- **Sequence:** 0

**Constant Sequence**
- **Description:** A sequence where each term is a fixed constant value. It takes one parameter: the constant value itself.
- **Parameters:** 1
- **Sequences:** 0

**Drop Sequence**
- **Description:** A sequence that modifies another sequence by skipping a fixed number of initial terms. It takes one parameter (the count of terms to drop) and one underlying sequence.
- **Parameters:** 1
- **Sequences:** 1

**Geometric Sequence**
- **Description:** A sequence where each term is derived by multiplying the previous term by a fixed ratio. It takes two parameters: the initial term and the common ratio.
- **Parameters:** 2
- **Sequences:** 0

**Linear Combination Sequence**
- **Description:** A sequence that combines two other sequences linearly using specified coefficients. It takes two parameters (the coefficients for each sequence) and two underlying sequences.
- **Parameters:** 2
- **Sequences:** 2

**Log Sequence**
- **Description:** A logarithmic sequence that takes two sequences. The first sequence is used as the logarithm's argument, and the second as the base of the logarithm. If the logarithm is undefined (e.g., base is 1 or the logarithm's argument is negative), it returns f64::NAN.
- **Parameters:** 0
- **Sequences:** 2

**Operation Sequence**
- **Description:** First, the values of the first and second sequences are calculated and used in an operation that returns the result 𝑎. Then, the values of the first and third sequences are calculated, resulting in 𝑏. Next, the absolute differences between 𝑎 and the parameter 𝑐, as well as between 𝑏 and 𝑐, are compared. The result closer to the parameter 𝑐 is selected as the value of the current sequence member. It takes three sequences. 1st parameter: operation (1 - addition, 2 - subtraction, 3 - multiplication, 4 - division), 2nd parameter: 𝑐, which is the value against which the results will be compared to find the smallest difference.
- **Parameters:** 2
- **Sequences:** 3

**Power Sequence**
- **Description:** A sequence that computes the power of the absolute value of the first sequence's terms raised to the corresponding terms of the second sequence. It takes two underlying sequences as parameters.
- **Parameters:** 0
- **Sequences:** 2

**Produkt Sequence**
- **Description:** The product of two sequences. It takes the first and second sequences and returns the product of their 𝑘-th terms at the 𝑘-th position.
- **Parameters:** 0
- **Sequences:** 2

**Random Sequence**
- **Description:** A random sequence. It takes two sequences and one parameter of probability, which should be a number between 0 and 1 (inclusive). Based on the value of this parameter, the sequence will choose between the terms of the first and second sequences. The higher the number, the more likely it will select terms from the first sequence. (For example, if the probability parameter is set to 1, it will always return terms from the first sequence; if set to 0, it will always select terms from the second sequence; if set to 0.50, it will provide terms from both sequences approximately equally.)
- **Parameters:** 1
- **Sequences:** 2

## Authors:
- Enej Brus
- Anže Gartner

