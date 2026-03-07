# 🦀 Rust Chapter 4: Eksmisja lokatorów ze sterty czyli jak nie zostać z pustym wskaźnikiem

## 📂 1. Pamięć: Gdzie siedzą Twoje dane?

| **Miejsce** | **Cecha** | **Co tam trafia?** |
| --- | --- | --- |
| **Stos (Stack)** | Mega szybki, stały rozmiar (poukładane jak talerze). | `i32`, `f64`, `bool`, `char`, oraz **metadane** (wskaźniki). |
| **Sterta (Heap)** | Wolniejszy, dynamiczny rozmiar (magazyn / sejf). | `String`, `Vec<T>`, dane, które mogą rosnąć lub maleć. |

> **Złota zasada:** Zmienna na stosie to "wizytówka" lub "klucz do sejfu". Sam towar leży na stercie (Heap).

## 🔑 2. Trzy Święte Zasady Ownershipu (Własności)

1.  Każda wartość w Rust ma zmienną, która jest jej **właścicielem**.
2.  Może być tylko **jeden właściciel** na raz.
3.  Gdy właściciel wychodzi poza **zasięg (scope)** `{ }`, wartość jest automatycznie usuwana (`drop`).

## 🚚 3. Przenoszenie (Move) vs Kopiowanie (Copy)

### Move (Dane na stercie)

Kiedy przypisujesz jedną zmienną do drugiej, oddajesz jej "klucz do sejfu". Stary właściciel przestaje istnieć.

```
let s1 = String::from("hello");
let s2 = s1; // s1 oddaje klucz do s2. s1 PRZESTAJE istnieć.
```

### Copy (Dane na stosie)

Typy o stałym rozmiarze są po prostu kopiowane (bity po bitach). Obie zmienne żyją.

```
let x = 5;
let y = x; // x nadal żyje, y dostał kopię "piątki".
```

### Clone

Jeśli naprawdę chcesz skopiować towar w sejfie (wolne!), użyj: `let s2 = s1.clone();`.

## 🤝 4. Referencje i Borrowing (Pożyczanie)

_Czyli: Jak dać sprzątaczce klucze tylko na chwilę, bez przepisywania własności mieszkania w księdze wieczystej._

-   **Niemutowalne (`&T`)**: Możesz tylko czytać. Możesz mieć ich wiele.
-   **Mutowalne (`&mut T`)**: Możesz zmieniać dane. **Tylko jedna na raz w danym zakresie!**

**Zasady komornika (Borrow Checker):**

1.  Możesz mieć albo wiele `&T` (czytelnicy).
2.  Albo dokładnie jedną `&mut T` (pisarz).
3.  Nigdy obu na raz!

## 🍕 5. Slices (Wycinki)

_Czyli: Branie tylko gryza z cudzej kanapki bez przejmowania całej własności._

Służą do pokazywania fragmentu kolekcji.

```
let s = String::from("hello world");
let hello: &str = &s[0..5]; // Wycinek wskazuje na pierwsze 5 bajtów.
```

### Typ `&str` (String Slice)

-   **Literały** (`let s = "Siema";`) to zawsze `&str`.
-   Są "wypalone" w pliku binarnym Twojego programu.
-   Są na stosie (jako wskaźnik + długość), ale celują w pamięć programu (nie Heap).

## 🛠 6. Składnia "Seniora"

-   **`::` (Operator zakresu)**: Dzwonisz do fabryki/typu. Np. `String::from()`.
-   **`.` (Operator metody)**: Masz już przedmiot w ręku i coś z nim robisz. Np. `s.len()`.
-   **Moduł (`mod`)**: Pudełko na kod (folder). Nie myl z klasą! Rust rozdziela Dane (`struct`) od Logiki (`impl`).

### ⚠️ Dangling References (Wiszące odwołania)

Rust nigdy nie pozwoli Ci zwrócić referencji do czegoś, co zaraz zostanie usunięte z pamięci. _Nie możesz dać klucza do domu, który właśnie wyburzasz!_

