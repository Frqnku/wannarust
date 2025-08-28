# 🦀 WannaRust

![Rust](https://img.shields.io/badge/rust-%23FF3E00.svg?style=for-the-badge&logo=rust&logoColor=white) ![Docker](https://img.shields.io/badge/docker-%23049fd9.svg?style=for-the-badge&logo=docker&logoColor=white)

## ⚠️ Disclaimer

**This project is strictly for educational and research purposes only.**  
It demonstrates how ransomware like *WannaCry* can be simulated in Rust.  
The author does **not** take any responsibility for misuse.  
❌ **Do not use this in production or against real systems.**

## 📖 Overview

**WannaRust** is a ransomware **simulation** written in Rust, inspired by the infamous *WannaCry* malware.  
It is designed to explore:  
- 🔐 File encryption using AES and RSA  
- 📂 Recursive folders infection simulation  
- 📦 Containerization via Docker  
- 🦀 Modern Rust programming practices  

The goal is to provide a safe environment for studying malware mechanics and defensive strategies.
The files to infect are located in /home/infection.

## 🗂️ Project Structure

- 📁 **infection/** → Folder containing mockup files to be infected  
- 🦀 **wannarust/** → Main Rust executable simulating ransomware behavior  
- 🐳 **Dockerfile** → Build and run the project in a containerized environment  
- 🔧 **Makefile** → Provides helper commands for building and running the project  

## 🚀 Installation

Clone the repository:

```bash
git clone https://github.com/Frqnku/wannarust.git
cd wannarust
```

### 〽️ Using Makefile

Build the container with the malware:

```bash
make
```

Run the container:

```bash
make run
```

Run the malware:

```bash
./wannarust
```

Infected files are located at /home/infection

Run the malware with -r to reverse infection:

```bash
./wannarust -r
```

---

Made with ❤️ and 🦀 Rust — for **learning purposes only**!
