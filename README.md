# 🦀 Rust Learning Journey (22 Days Challenge - Extended)

Moje postępy w nauce języka Rust.
Celem jest przerobienie całej książki **"The Rust Programming Language"**.

## Status Wyzwania

### Podstawy (Rozdziały 1 - 6)
- [x] **Dzień 1:** Rozdziały 1 & 2 (Instalacja, Gra w zgadywanie)
- [x] **Dzień 2:** Rozdział 3 (Podstawowe pojęcia, Typy danych, Pętle)
- [x] **Dzień 3:** Rozdział 4 (Ownership - Własność, Referencje)
- [x] **Dzień 4:** Rozdział 5 (Struktury i Metody)
- [x] **Dzień 5:** Rozdział 6 (Enumy i Option<T>)

### Architektura i Dane (Rozdziały 7 - 11)
- [x] **Dzień 6:** Rozdział 7 (Pakiety, Crates i Moduły)
- [x] **Dzień 7:** **Buffer Day** (Odpoczynek + re-kalibracja)
- [x] **Dzień 8:** Rozdział 8 (Kolekcje: Vec, String, HashMap)
- [x] **Dzień 9:** Rozdział 9 (Obsługa błędów - panic!, Result, ? operator)
- [x] **Dzień 10:** Rozdział 10 (Generics, Traits i Lifetimes)
- [x] **Dzień 11:** Rozdział 11 (Pisanie testów automatycznych)

### Projekty i zaawansowane
- [x] **Dzień 12:** Projekt `vice-tracker` (Zarządzanie stanem i iteratory)
- [x] **Dzień 13-14:** Przerwa
- [x] **Dzień 15:** Rozdział 12 (Projekt I/O: Budujemy własne `minigrep`)
- [x] **Dzień 16:** Rozdziały 13 - 15 (Closures, Iterators, Smart Pointers)
- [x] **Dzień 17:** Przerwa
- [x] **Dzień 18-19:** Rozdział 16 (Wielowątkowość)
- [x] **Dzień 20:** Rozdziały 17 - 18 (Asynchroniczność, OOP, Wzorce)
- [x] **Dzień 21:** Rozdział 19 (Zaawansowane wzorce dopasowań)
- [x] **Dzień 22:** Przerwa
- [x] **Dzień 23:** Rozdział 20 (Advanced Features: Unsafe, Traity, Typy, Makra)
- [ ] **Dzień 24:** Finał! Rozdział 21 (Wielowątkowy serwer webowy)

---

##  Struktura Projektów

- **Projects/**
  - `vice_tracker/` - System śledzenia nawyków.
  - `chip8_display/` - Eksperymenty z wyświetlaczem CHIP-8.
- **Chapter1-11/** - Podstawy, Ownership, Moduły, Kolekcje, Błędy, Testy.
- **Chapter12/** - `minigrep/` - Narzędzie CLI do przeszukiwania plików.
- **Chapter13-15/** - Domknięcia, iteratory, Smart Pointers.
- **Chapter16/** - Wielowątkowość (Arc, Mutex, Channels).
- **Chapter17-18/** - Asynchroniczność i OOP.
- **Chapter19/** - Zaawansowane wzorce dopasowań.
- **Chapter20/** - Zaawansowane funkcje (Unsafe, Associated Types, Macros).

---

## Jak uruchomić? 🛠️

Każdy podfolder to osobny projekt Cargo. Aby uruchomić wybrany przykład:
1. Wejdź do folderu: `cd ChapterX/`
2. Uruchom: `cargo run`

> *"Talk is cheap. Show me the code."* — Linus Torvalds