# 📦 AutoFileCreate v3.0.0 (for Jupyter Notebook)

Automated tool that generates structured Jupyter Notebook project folders with practice and project files — built for students.

Supports:

- Windows (Installer + PyPI)
- macOS
- Linux

---

## 🚀 Installation

### Recommended (Cross-Platform)

Install via pip:

```bash
pip install autofilecreate
```

Then run:

```bash
autofilecreate
```

---

### 🪟 Windows (Installer Version)

Download from the [Releases](https://github.com/serenebliss0/Testing-repo/releases/tag/AutoFileCreate-v3.0.0) section:

```
autofilecreate-py.exe
```

Double click and install.

It will:

- Install into Program Files
- Add itself to PATH
- Register command globally

After installation:

```powershell
autofilecreate-py
```

---

### 🍎 macOS

**Option 1 — Pip (Easiest)**

```bash
pip3 install autofilecreate
```

Run:

```bash
autofilecreate
```

**Option 2 — From Source**

```bash
[Go to Releases](https://github.com/serenebliss0/Testing-repo/releases/download/AutoFileCreate-v3.0.0/autofilecreate-3.0.0.tar.gz)

# download the .tar.gz file
# extract it
cd AutoFileCreate
pip install .
```

---

### 🐧 Linux

Same as Mac:

```bash
pip install autofilecreate
```

Run:

```bash
autofilecreate
```

If PATH fails:

```bash
python -m autofilecreate.main
```

---

## ⚡ Usage

### First Time Setup

Run:

```bash
autofilecreate --config
```

It will create:

```
config.serenity
```

You enter your main COS102 folder path.  
Ensure that you don't leave any extra spaces or newlines

---

### Create Files for a Week

```bash
autofilecreate --week 1
```

It will:

- Create `week_1`
- Ask how many practice files
- Ask how many project files
- Generate `.ipynb` files automatically

---

### Interactive Mode

Run:

```bash
autofilecreate
```

It will ask for week manually.

---

## ⚙ Configuration

The config file:

```
config.serenity
```

Stores:

```
Path to your main project folder
```

Edit anytime using:

```bash
autofilecreate --config
```

---


https://github.com/user-attachments/assets/0f96265d-58a1-4db7-8a1f-92cdc90111f0


## Project Structure After Running

Example:

```
COS102/
 └── week_1/
      ├── practice_1.ipynb
      ├── practice_2.ipynb
      ├── project_1.ipynb
```

---

## Development

If contributing:

```bash
git clone https://github.com/serenebliss0/Testing-repo/
cd AutoFileCreate_python
pip install -e .
```

Run locally:

```bash
python src/main.py
```

---

## Tech Stack

- Python
- argparse
- pathlib
- PyInstaller
- Inno Setup (Windows)
- PyPI Packaging

---

> Built by [Semire Designs](https://github.com/serenebliss0)
