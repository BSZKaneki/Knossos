
---

```markdown
# Dokumentacja: Dynamiczny Generator i Solver Labiryntów

Ten dokument opisuje architekturę, konfigurację i możliwości rozbudowy projektu.

## 1. Cel Projektu
---
Głównym celem projektu jest stworzenie elastycznej platformy do generowania labiryntów oraz wizualnego porównywania wydajności dowolnej liczby algorytmów wyszukiwania ścieżki. Dzięki modularnej budowie, dodawanie nowych algorytmów jest proste i nie wymaga modyfikacji głównej logiki symulacji.

## 2. Struktura Projektu
---
Kod został zaprojektowany z myślą o maksymalnej elastyczności, oddzielając dane konfiguracyjne, logikę i wizualizację.

*   **`Config` (struct):**
    Centralny obiekt konfiguracyjny. Kluczowym polem jest `algorithms_to_run: Vec<Algorithm>`, które pozwala zdefiniować listę algorytmów do uruchomienia w jednej symulacji.

*   **`Simulation` (struct):**
    Główny kontroler aplikacji. Jego pętla dynamicznie iteruje po liście algorytmów z konfiguracji, uruchamiając dla każdego obliczenia i wizualizację, a na końcu wyświetlając zbiorcze wyniki.

*   **`Visualization` (struct):**
    Moduł odpowiedzialny za renderowanie. Rysuje stan labiryntu, animacje poszukiwań oraz dynamicznie generowany ekran statystyk końcowych.

*   **`Maze` (struct):**
    Reprezentacja labiryntu. Zawiera logikę generowania oraz implementacje poszczególnych algorytmów wyszukiwania ścieżki.

*   **System Algorytmów:**
    *   `Algorithm` (enum): Identyfikator dla każdego algorytmu (np. `Bfs`, `Dfs`).
    *   `AlgorithmInfo` (struct): Przechowuje wszystkie metadane algorytmu: jego nazwę, przypisane kolory oraz wskaźnik na funkcję implementującą jego logikę.
    *   `get_algorithm_info()` (funkcja): Centralna "rejestracja" algorytmów.
    *   `PathfindingResult` (struct): Przechowuje wyniki działania jednego algorytmu w celu późniejszego wyświetlenia.

## 3. Generowanie Labiryntu
---
Aplikacja wspiera dwa tryby generowania:

1.  **Idealny Labirynt (Perfect Maze):** Generowany przez DFS, bez cykli, z jedną unikalną ścieżką między dowolnymi dwoma punktami.
2.  **Niedoskonały Labirynt (Imperfect Maze):** Po stworzeniu idealnego labiryntu, usuwana jest część ścian, co tworzy pętle i alternatywne drogi.

## 4. System Algorytmów Znajdowania Ścieżki
---
System jest w pełni dynamiczny i sterowany konfiguracją. Główna pętla symulacji pobiera zdefiniowaną przez użytkownika listę algorytmów i dla każdego z nich:
1.  Pobiera jego metadane (nazwę, kolory, funkcję) z `get_algorithm_info`.
2.  Wykonuje obliczenia, wywołując wskazaną funkcję.
3.  Uruchamia wizualizację (jeśli nie jest pomijana).
4.  Zapisuje wyniki do późniejszego wyświetlenia.

## 5. Wizualizacja
---
-   Wizualizacja jest w pełni sterowana danymi z `AlgorithmInfo`. Tytuł animacji i kolory są pobierane dynamicznie dla każdego algorytmu.
-   Po zakończeniu animacji jednego algorytmu, widok labiryntu jest resetowany przed uruchomieniem kolejnego.
-   Ekran statystyk końcowych jest generowany dynamicznie i wyświetla podsumowanie dla wszystkich uruchomionych algorytmów.

## 6. Konfiguracja
---
Głównym punktem konfiguracyjnym jest pole `algorithms_to_run` w strukturze `Config`, modyfikowane w funkcji `main` pliku `main.rs`.

**Przykłady użycia:**

```rust
// main.rs

fn main() {
    let config = Config {
        // Uruchom tylko BFS
        // algorithms_to_run: vec![Algorithm::Bfs],

        // Uruchom tylko DFS
        // algorithms_to_run: vec![Algorithm::Dfs],

        // Porównaj DFS z BFS (DFS będzie pierwszy)
        algorithms_to_run: vec![Algorithm::Dfs, Algorithm::Bfs],

        ..Default::default()
    };

    let mut simulation = Simulation::new(&config);
    simulation.run();
}
```

## 7. Rozszerzalność (Jak Dodać Nowy Algorytm)
---
Struktura projektu sprawia, że dodanie nowego algorytmu (np. A*) jest niezwykle proste i sprowadza się do 3 kroków:

1.  **Dodaj identyfikator do enuma `Algorithm`**:
    ```rust
    pub enum Algorithm { Bfs, Dfs, AStar }
    ```

2.  **Zarejestruj jego metadane w funkcji `get_algorithm_info`**:
    Dodaj nową gałąź `match` dla `AStar`, podając jego nazwę, kolory i wskaźnik na funkcję.
    ```rust
    fn get_algorithm_info(algo: Algorithm) -> AlgorithmInfo {
        match algo {
            // ... istniejące gałęzie
            Algorithm::AStar => AlgorithmInfo {
                id: Algorithm::AStar,
                name: "A*",
                function: Maze::path_finding_a_star, // Przykładowa nazwa funkcji
                search_color: 0xAAFF8C00, // np. pomarańczowy
                path_color: 0xAA00FA9A,   // np. zielony
            },
        }
    }
    ```

3.  **Zaimplementuj logikę w `impl Maze`**:
    Stwórz nową funkcję `fn path_finding_a_star(&self) -> ...`, która zawiera jego implementację.

To wszystko. Główna pętla symulacji i system wizualizacji automatycznie obsłużą nowy algorytm.

## 8. Podsumowanie
---
Projekt jest elastyczną platformą do testowania i wizualizacji algorytmów grafowych. Główne zalety jego architektury to:
-   **Skalowalność:** Można łatwo dodawać nowe algorytmy bez ingerencji w rdzeń programu.
-   **Elastyczność:** Użytkownik może w prosty sposób zdefiniować, które algorytmy chce porównać.
-   **Czystość kodu:** Oddzielenie danych (metadane algorytmów) od logiki (pętla symulacji) sprawia, że kod jest bardziej czytelny i łatwiejszy w utrzymaniu.

```