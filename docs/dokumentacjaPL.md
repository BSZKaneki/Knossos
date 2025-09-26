

---

```markdown
# Dokumentacja: Dynamiczny Generator i Solver Labiryntów

Ten dokument opisuje architekturę, konfigurację oraz możliwości rozbudowy projektu generującego i rozwiązującego labirynty.

## 1. Cel Projektu
---
Celem projektu jest stworzenie elastycznej platformy do generowania labiryntów oraz wizualnego porównywania wydajności różnych algorytmów wyszukiwania ścieżki. Modularna architektura umożliwia łatwe dodawanie nowych algorytmów bez konieczności modyfikacji głównej logiki symulacji, a dynamiczna pętla pozwala na uruchamianie ich w dowolnej kolejności.

## 2. Struktura Projektu
---
Kod został zaprojektowany z myślą o modularności i czytelności, oddzielając konfigurację, logikę i wizualizację.

- **`Config` (struct):**  
  Centralny obiekt konfiguracyjny, definiujący parametry symulacji. Kluczowe pole `algorithms_to_run: Vec<Algorithm>` określa sekwencję algorytmów do uruchomienia.

- **`Simulation` (struct):**  
  Główny kontroler aplikacji. Zarządza pętlą symulacji, która dynamicznie iteruje po liście algorytmów, wykonując obliczenia, wizualizację i wyświetlając zbiorcze wyniki.

- **`Visualization` (struct):**  
  Moduł odpowiedzialny za renderowanie labiryntu, animacje poszukiwań oraz ekran statystyk końcowych, generowany dynamicznie dla wszystkich algorytmów.

- **`Maze` (struct):**  
  Reprezentacja labiryntu. Zawiera logikę generowania labiryntu oraz implementacje algorytmów wyszukiwania ścieżki.

- **System Algorytmów:**  
  - `Algorithm` (enum): Identyfikator algorytmów (np. `Bfs`, `Dfs`).  
  - `AlgorithmInfo` (struct): Przechowuje metadane algorytmu: nazwę, kolory wizualizacji i wskaźnik na funkcję implementującą.  
  - `get_algorithm_info` (funkcja): Centralne miejsce rejestracji algorytmów.  
  - `PathfindingResult` (struct): Przechowuje wyniki algorytmu (liczba kroków, czas, długość ścieżki) do wyświetlenia na ekranie statystyk.

## 3. Generowanie Labiryntu
---
Projekt wspiera dwa tryby generowania labiryntów:
1. **Idealny Labirynt (Perfect Maze):** Generowany algorytmem DFS (rekurencyjne cofanie), bez cykli, z jedną unikalną ścieżką między punktami.  
2. **Niedoskonały Labirynt (Imperfect Maze):** Bazuje na idealnym labiryncie, ale usuwa losowo wybrane ściany, tworząc pętle i alternatywne ścieżki.

## 4. System Algorytmów Znajdowania Ścieżki
---
System jest w pełni dynamiczny i sterowany konfiguracją. Pętla symulacji:
1. Pobiera listę algorytmów z `Config::algorithms_to_run`.  
2. Dla każdego algorytmu:  
   - Pobiera metadane z `get_algorithm_info` (nazwa, kolory, funkcja).  
   - Wykonuje obliczenia, wywołując odpowiednią funkcję pathfindingu.  
   - Uruchamia wizualizację (jeśli nie jest pomijana).  
   - Zapisuje wyniki do późniejszego wyświetlenia.  
3. Wyświetla zbiorczy ekran statystyk dla wszystkich algorytmów.

Obecnie zaimplementowane algorytmy:
- **BFS (Breadth-First Search):** Zawsze znajduje najkrótszą ścieżkę.  
- **DFS (Depth-First Search):** Znajduje poprawną ścieżkę, ale niekoniecznie najkrótszą.

## 5. Wizualizacja
---
- Wizualizacja jest sterowana dynamicznie na podstawie danych z `AlgorithmInfo`. Każdy algorytm ma unikalne kolory dla animacji poszukiwań i ścieżki.  
- Po zakończeniu wizualizacji jednego algorytmu labirynt jest resetowany przed uruchomieniem kolejnego.  
- Ekran statystyk końcowych wyświetla podsumowanie dla wszystkich algorytmów, w tym nazwę, liczbę kroków, czas wykonania i długość ścieżki.

## 6. Konfiguracja
---
Konfiguracja odbywa się poprzez modyfikację struktury `Config` w funkcji `main` w pliku `main.rs`. Kluczowe opcje:
- `use_perfect_maze`: `true` dla labiryntu idealnego, `false` dla niedoskonałego.  
- `skip_visualization`: `true` pomija animacje i wyświetla tylko wyniki.  
- `maze_width` i `maze_height`: Wymiary labiryntu.  
- `batch_size`: Liczba komórek przetwarzanych w jednej klatce animacji (kontroluje prędkość).  
- `target_fps`: Docelowa liczba klatek na sekundę.  
- `algorithms_to_run`: Wektor określający algorytmy i ich kolejność.

**Przykłady konfiguracji:**

```rust
// main.rs
fn main() {
    let config = Config {
        // Uruchom tylko BFS
        // algorithms_to_run: vec![Algorithm::Bfs],

        // Uruchom tylko DFS
        // algorithms_to_run: vec![Algorithm::Dfs],

        // Porównaj DFS i BFS (w tej kolejności)
        algorithms_to_run: vec![Algorithm::Dfs, Algorithm::Bfs],

        ..Default::default()
    };

    let mut simulation = Simulation::new(&config);
    simulation.run();
}
```

## 7. Rozszerzalność (Jak Dodać Nowy Algorytm)
---
Dodanie nowego algorytmu (np. A*) wymaga trzech prostych kroków:

1. **Dodaj identyfikator do enuma `Algorithm`:**
   ```rust
   pub enum Algorithm {
       Bfs,
       Dfs,
       AStar,
   }
   ```

2. **Zarejestruj metadane w funkcji `get_algorithm_info`:**
   Dodaj nową gałąź `match` z nazwą, kolorami i wskaźnikiem na funkcję.
   ```rust
   fn get_algorithm_info(algo: Algorithm) -> AlgorithmInfo {
       match algo {
           // ... istniejące gałęzie ...
           Algorithm::AStar => AlgorithmInfo {
               name: "A*",
               function: Maze::path_finding_a_star,
               search_color: 0xAAFF8C00, // np. pomarańczowy
               path_color: 0xAA00FA9A,   // np. zielony
           },
       }
   }
   ```

3. **Zaimplementuj logikę w `impl Maze`:**
   Dodaj nową funkcję, np. `fn path_finding_a_star(&self) -> (usize, u128, Vec<(usize, usize)>, Vec<(usize, usize)>)`, implementującą algorytm.

Po tych krokach nowy algorytm będzie automatycznie obsługiwany przez pętlę symulacji i wizualizację. Wystarczy dodać go do `algorithms_to_run` w `Config`.

## 8. Podsumowanie
---
Projekt stanowi elastyczną platformę do testowania i wizualizacji algorytmów wyszukiwania ścieżki w labiryntach. Główne zalety:
- **Skalowalność:** Łatwe dodawanie nowych algorytmów bez zmian w głównej logice.  
- **Elastyczność:** Możliwość definiowania sekwencji algorytmów w konfiguracji.  
- **Czystość kodu:** Wyraźne oddzielenie konfiguracji, logiki i wizualizacji zwiększa czytelność i łatwość utrzymania.  
- **Wizualizacja:** Intuicyjne animacje i zbiorczy ekran statystyk ułatwiają porównywanie algorytmów.

```

