# 🦀 Rust Learning Journey (22 Days Challenge)

Moje postępy w nauce języka Rust podczas praktyk zawodowych.

Celem jest przerobienie całej książki **"The Rust Programming Language"** w 22 dni.

## 🚀 Status Wyzwania

-   [x] **Dzień 1:** Rozdziały 1 & 2 (Instalacja, Hello World, Gra w zgadywanie)
-   [x] **Dzień 2:** Rozdział 3 (Podstawowe pojęcia, Typy danych, Funkcje, Pętle + Boss Rush)
-   [x] **Dzień 3:** Rozdział 4 (Ownership - Własność, Referencje, Slices)
-   [x] **Dzień 4:** Rozdział 5 (Struktury - Grupowanie powiązanych danych, Metody)
-   [x] **Dzień 5:** Rozdział 6 (Enumy, Option<T>, Match i if let - Pancerne typy danych)
-   [x] **Dzień 6:** Rozdział 7 (Pakiety, Crates i Moduły - Zarządzanie strukturą i prywatnością kodu)
-   [ ] **Dzień 7:** Rozdział 8 (Wspólne kolekcje - Vec, String, HashMap)

## 📂 Struktura Projektów
-   **Projects/**
    -   `vice_tracker/` - (W TRAKCIE) Projekt CLI do śledzenia nawyków i nałogów.
-   **Chapter1/** - Podstawy (Hello World, Cargo).
-   **Chapter2/** - Projekt: `guessing_game` (Gra w zgadywanie).
-   **Chapter3/**
    -   `variables/` - Mutowalność i stałe.
    -   `data_types/` - Skalary, krotki i tablice.
    -   `chapter3_challenges/` - Konwerter temp., Fibonacci, Kolęda.
-   **Chapter4/**
    -   `ownership/` - Jeden projekt zawierający wszystko: Ownership, Referencje i Slices.
-   **Chapter5/**
    -   `structures/` - Definiowanie struktur i bloki `impl`.
-   **Chapter6/**
    -   `enums/` - Definiowanie enumów, potęga `Option<T>` oraz bezpieczna obsługa danych przez `match` i `if let`.
-   **Chapter7/**
    -   `architecture/` - Organizacja kodu: podział na pliki, foldery (`mod.rs`), sterowanie dostępem (`pub`) i re-eksportowanie ścieżek.

## 📝 Notatki i ściągi

-   `CHAPTER_04.md` - Kompleksowa ściąga z zasad Ownershipu i zarządzania pamięcią.

## 🛠️ Jak uruchomić?

Każdy podfolder to osobny projekt Cargo. Aby uruchomić wybrany przykład:

1.  Wejdź do folderu: `cd ChapterX/nazwa_projektu`
2.  Uruchom: `cargo run`

_🦀::Done with Chapter 7 - Project Structured (Architecture is Key!)::✅_