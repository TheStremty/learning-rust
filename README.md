# 🦀 Rust Learning Journey (22 Days Challenge)

Moje postępy w nauce języka Rust.
Celem jest przerobienie całej książki **"The Rust Programming Language"** w 22 dni.

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
- [x] **Dzień 12:** Praktyczny projekt `vice-tracker` (Zarządzanie stanem i iteratory) - część dalsza
- [x] **Dzień 13:** Przerwa (Sprawy osobiste)
- [x] **Dzień 14:** Przerwa (Sprawy osobiste)
- [x] **Dzień 15:** Rozdział 12 (Projekt I/O: Budujemy własne `minigrep`)
- [x] **Dzień 16:** Rozdziały 13 - 15 (Closures, Iterators, Smart Pointers)
### Nadchodzące
- ...

---

##  Struktura Projektów

- **Projects/**
  - `vice_tracker/` - System śledzenia nawyków.
- **Chapter1-6/** - Podstawy języka, Ownership, Struktury oraz Enumy.
- **Chapter7/** - Podział kodu na moduły i zarządzanie prywatnością.
- **Chapter8/** - Praktyczne zastosowanie dynamicznych list i map.
- **Chapter9/** - Pancerne zarządzanie błędami.
- **Chapter10/** - Generyki, Traity i mechanika Lifetimes.
- **Chapter11/** - Testy jednostkowe i integracyjne.
- **Chapter12/** 
  - `minigrep/` - Narzędzie CLI do przeszukiwania plików (Rozdział 12).
- **Chapter13-15/**
   - `c13/` - Domknięcia, iteratory
   - `c15/` - Smart Pointers
---

## Jak uruchomić? 🛠️

Każdy podfolder to osobny projekt Cargo. Aby uruchomić wybrany przykład:
1. Wejdź do folderu: `cd Projects/nazwa_projektu` lub `cd ChapterX/`
2. Uruchom: `cargo run`

> *"Talk is cheap. Show me the code."* — Linus Torvalds