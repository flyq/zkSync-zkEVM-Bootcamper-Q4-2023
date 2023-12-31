# zkSync-zkEVM-Bootcamper-Q4-2023

- [zkSync-zkEVM-Bootcamper-Q4-2023](#zksync-zkevm-bootcamper-q4-2023)
  - [Lesson 1 Introduction to Blockchain and Layer 1](#lesson-1-introduction-to-blockchain-and-layer-1)
    - [Course](#course)
    - [Homework](#homework)
  - [Lesson 2 Why Scalability](#lesson-2-why-scalability)
    - [Course](#course-1)
    - [Homework](#homework-1)
  - [Lesson 3 - Introduction to Layer 2](#lesson-3---introduction-to-layer-2)
    - [Course](#course-2)
    - [Homework](#homework-2)
  - [Lesson 4 - Maths and Cryptograph](#lesson-4---maths-and-cryptograph)
    - [Course](#course-3)
    - [Homework](#homework-3)
  - [Lesson 5 - Understanding and Analysing Layer 2](#lesson-5---understanding-and-analysing-layer-2)
    - [Course](#course-4)
    - [Homework](#homework-4)
  - [Lesson 6](#lesson-6)
    - [Course](#course-5)
    - [Homework](#homework-5)
  - [Lesson 7](#lesson-7)
    - [Course](#course-6)
    - [Homework](#homework-6)
  - [Lesson 8](#lesson-8)
    - [Course](#course-7)
    - [Homework](#homework-7)
  - [Lesson 9](#lesson-9)
    - [Course](#course-8)
    - [Homework](#homework-8)
  - [Lesson 10](#lesson-10)
    - [Course](#course-9)
    - [Homework](#homework-9)
  - [Lesson 11](#lesson-11)
    - [Course](#course-10)
    - [Homework](#homework-10)
  - [Lesson 12](#lesson-12)
    - [Course](#course-11)
    - [Homework](#homework-11)
  - [Lesson 13](#lesson-13)
    - [Course](#course-12)
    - [Homework](#homework-12)
  - [Lesson 14](#lesson-14)
    - [Course](#course-13)
    - [Homework](#homework-13)
  - [Lesson 15](#lesson-15)
    - [Course](#course-14)
    - [Homework](#homework-14)
  - [Lesson 16](#lesson-16)
    - [Course](#course-15)
    - [Homework](#homework-15)
  - [Lesson 17](#lesson-17)
    - [Course](#course-16)
    - [Homework](#homework-16)

## Lesson 1 Introduction to Blockchain and Layer 1

### Course

It mainly talks about how Bitcoin was born and how Ethereum operates, from execution/consensus separation to modular blockchain.

I feel like I'm dreaming back to 2017. I just came into contact with the rabbit hole of blockchain. I read the BTC/ETH white paper and read some legendary stories before 2015. Everything started with BTC, but it has branched out, and there are all kinds of new things. Ideas, new technologies emerge.

### Homework

https://zeroknowledge.fm/268-2/

## Lesson 2 Why Scalability

### Course
Explain the architecture of each layer2

### Homework
1. Why is client diversity important for Ethereum
Client diversity can improve the robustness of the entire Ethereum blockchain. For example, when a bug occurs in a certain client, other clients can run normally.

2. How would you describe the following concepts and how do you rank them in terms of their importance to you
   * TPS and transaction cost
   * Finality
   * Privacy
   * Decentralisation
   * Security


## Lesson 3 - Introduction to Layer 2

### Course

some rust, detail of evm

### Homework

https://github.com/flyq/zkEVMBootcamp

## Lesson 4 - Maths and Cryptograph

### Course

modular arithmetic

poly and soon

### Homework
1. Working with the following set of Integers S= {0,1,2,3,4,5,6}
What is 
a) 4 + 4 = 1 mod 7
b) 3x5 = 1 mod 7
c) what is the inverse of 3 under
  i) addition -3 = 4 mod 7
  ii) multiplication $3^{-1} = 5 mod 7$

2. For S = {0,1,2,3,4,5,6}
Can we consider 'S' and the operation '+' to be a group?
yes

3. What is -13 mod 5 ?
   2
4. Polynomials
For the polynomial $×^3 - x^2 + 4х — 12$
Find a positive root
2
What is the degree of this polynomial?
3

## Lesson 5 - Understanding and Analysing Layer 2

### Course
通用 Layer2:
Arbitrum one
Optimism
Boba
Starknet

特定 Layer2:
Loopring
zkSync
Aztec

不同阶段：
0. 完整训练轮：完全由操作员操作
1. 有限训练轮：存在中心化治理，解决潜在错误
2. 无辅助阶段：完全由智能合约管理


### Homework

https://zeroknowledge.fm/151-2/

## Lesson 6

### Course

L2 components and transaction lifecycle
L2 rollup process

### Homework

see `./src/bootcamp`

## Lesson 7

### Course

what is optimistic rollup vs zk rollup?
what is Snark

### Homework

https://github.com/flyq/zkEVMBootcamp

## Lesson 8

### Course
What's next in Layer 2 part 1: Decentralised Sequencers

### Homework

$p(x) = x^3 - 5x^2 - 4x + 20$
   
a. find an integer root $a$, i.e. $p(a) = 0$(clue $a<7$)

using sagemath: 
```sage
x = var("x")
p = x^3 - 5*x^2 - 4*x + 20
p.roots()
# [(2, 1), (-2, 1), (5, 1)]
```
so $a = 5 \ \text{or} \ 2 \ \text{or} \ -2$

b. $p(x) = (x-a)q(x)$, the degree?

$q(x) = x^2 -4$, and degree of $q(x)$ is $2$, and the degree of $p(x)$ is $3$

## Lesson 9

What's next in Layer 2 part 2: L3s/Hyperchains

### Course

zkSync's zkStack, hyperchain, customized chain.

### Homework

https://zeroknowledge.fm/299-2/

## Lesson 10

Privacy in Layer 2

### Course

Aztec
Zama.ai
Namada
Obscuro
Penumbra
Anoma

### Homework

https://github.com/flyq/zkEVMBootcamp/commit/f1a8b6593bc25031039c788d8ad19f691bec7a17

## Lesson 11

What are the ZK EVMs part 1 - overview

### Course

### Homework

https://www.youtube.com/watch?v=bGEXYpt3sj0&list=PLS01nW3Rtgor_yJmQsGBZAg5XM4TSGpPs&index=3

## Lesson 12

What are the ZK EVMs part 2 - universal circuits/circuit compiler

### Course

### Homework
1. https://www.youtube.com/watch?v=AX7eAzfSB6w
2. answer
   1. $(w+1)\times (w+3) = O_3$
   2. w = 3 / -7
   3. G1: $w + 1 = O_1$ G2: $w + 3 = O_2$ G3: $O_1 \times O_2 = O_3$
   4. $$\left\{\begin{matrix} (w + 1)S_1 + w \cdot 1 \cdot (1-S_1) - O_1 = 0 \\ (w + 3)S_2 + w \cdot 3 \cdot (1-S_2) - O_2 = 0 \\ (O_1 + O_2) S_3 + O_1 \cdot O_2 \cdot (1 - S_3) - O_3 = 0 \end{matrix}\right.$$ 
   and $S_1 = 1, S_2 = 1, S_3 = 0$ 

## Lesson 13

### Course

### Homework

https://github.com/flyq/zkEVMBootcamp/commit/e9613efa0015063cd7f5a29a1e5289bfb6bb1f88

## Lesson 14

### Course

### Homework

[hw14](./src/hw14/)

## Lesson 15

### Course

### Homework

[hw15](./src/hw15/)


## Lesson 16

### Course

### Homework
also in: [hw15](./src/hw15/)

## Lesson 17

### Course

### Homework
