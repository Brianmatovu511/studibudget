<h1 align="center">🎓 studibudget</h1>
<p align="center">
  <em>A student-friendly Rust CLI to track income, expenses, and tuition runway — created by <strong>Brian Doctor</strong> for the Digital Business Skills project at THD.</em>
</p>

<p align="center">
  <a href="https://brianmatovu511.github.io/studibudget/"><img src="https://img.shields.io/badge/🌐%20Live%20Website-Visit-blue?style=for-the-badge" alt="Live Website"></a>
  <a href="https://github.com/Brianmatovu511/studibudget"><img src="https://img.shields.io/badge/💻%20Source%20Code-GitHub-black?style=for-the-badge&logo=github" alt="Source"></a>
</p>

---

### 🧭 Overview
**studibudget** is a lightweight, cross-platform **command-line tool built in Rust** that helps students manage their finances quickly and privately — directly from the terminal.

It tracks income, expenses, and estimates how many months of tuition your current balance can cover ("tuition runway").  
All data is stored locally — no accounts or internet required.

---

### ⚡ Features
- 💰 Add income and expenses with a single command  
- 📊 Show current balance and last transactions  
- 🧾 Export all records to CSV  
- 🎓 Calculate tuition runway  
- 🔒 100% offline and privacy-friendly  
- 🌈 Companion website with modern UI & animated gradient  

---

### 🧰 Tech Stack
| Component | Technology |
|------------|-------------|
| CLI Language | **Rust** |
| Website | **HTML, CSS (custom), JS** |
| Hosting | **GitHub Pages** |
| Build | **Cargo** (Rust package manager) |

---

### 🚀 Quick Start

#### 1️⃣ Clone and build from source
```bash
git clone https://github.com/Brianmatovu511/studibudget
cd studibudget/studibudget-cli
cargo build --release
2️⃣ Add transactions
bash
Copy code
# Income
.\target\release\studibudget.exe add income 450 -n "Kaufland shift"

# Expense
.\target\release\studibudget.exe add expense 300 -n "Groceries"

# Summary
.\target\release\studibudget.exe summary

# Tuition runway (monthly tuition in €)
.\target\release\studibudget.exe runway 250
3️⃣ Export to CSV
bash
Copy code
.\target\release\studibudget.exe export out.csv
4️⃣ Reset all data (type YES to confirm)
bash
Copy code
.\target\release\studibudget.exe reset
🌐 Live Demo
👉 Visit the Live Website →

The website includes:

A professional landing page with an animated gradient background

Usage examples and installation steps

A direct link to download static Linux binaries

🧑‍💻 Author
Brian Doctor Matovu
📧 brianmatovu51@gmail.com
🎓 Deggendorf Institute of Technology (THD) – European Campus Rottal-Inn
🧠 Project: Digital Business Skills – Build & Market Your Command-Line Tool

📜 License
This project is licensed under the MIT License — free for use and modification.

⭐ Acknowledgment
Thanks to the Digital Business Skills course team at THD for inspiring innovation in digital entrepreneurship and technical creativity.