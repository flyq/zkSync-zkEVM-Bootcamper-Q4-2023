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
What's next in Layer 2 part 1 : Decentralised Sequencers

### Homework

$p(x) = x^3 - 5x^2 - 4x + 20$
   
a. find an integer root $a$, i.e. $p(a) = 0$(clue $a<7$)

using sagemath: 
```sage
x = var("x")
p = x^3 - 5*x^2 -4*x +20
p.roots()
# [(2, 1), (-2, 1), (5, 1)]
```
so $a = 5 \ \text{or} \ 2 \ \text{or} \ -2$

b. $p(x) = (x-a)q(x)$, the degree?

$q(x) = x^2 -4$, and degree of $q(x)$ is $2$, and the degree of $p(x)$ is $3$
