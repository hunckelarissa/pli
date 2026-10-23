# 1.3 Keywords vs names (keywords-as-names)

**Статус:** черновик SPEC (нормативный для парсера)  
**Уровень:** L0+ (правило лексики/разбора на всех уровнях)  
**Якоря:** G-ref («PL/I keywords are not reserved»); Classic LRM; ADR-0004  
**Тесты:** `L0_MP_001` и расширенный набор must-parse

---

## Statement

В диалекте **PL/I-G** ключевые слова **не зарезервированы**. Одна и та же последовательность букв может быть **keyword в контексте** или **идентификатором пользователя**, в зависимости от синтаксической позиции и правил дисамбигуации лексера/парсера.

Это свойство **must-parse** с L0 и сохраняется на всех уровнях. Исполнение программ, где имена совпадают с keywords, — must-run только когда семантика L0 (и выше) это покрывает.

---

## Lexical form

1. Лексер выдаёт токены вида:
   - `Ident(text)` — сырая форма имени/потенциального keyword (регистр сохраняется для эмита/сообщений; сравнение — case-insensitive в ASCII, ADR-0010);
   - литералы, числа, строки, разделители, операторы.
2. **Нет** отдельного жёсткого класса «зарезервированное слово, всегда keyword».
3. Классификация Ident → KeywordToken vs Name происходит в **парсере** (и при необходимости в scanner lookahead), по правилам ниже.

---

## Disambiguation rules (норматив L0)

### R1 — Keyword in statement position

Если по грамматике в данной позиции ожидается **начало statement** с keyword-формой (например `IF`, `DO`, `END`, `RETURN`, `PUT`, `GET`, `SELECT`, `BEGIN`, `DECLARE`/`DCL`, `PROCEDURE`/`PROC`, …), то Ident, совпадающий с этим keyword (case-insensitive), принимается как **keyword**.

### R2 — Name in name position

Если по грамматике ожидается **имя** (объявляемое, ссылаемое, метка, поле, параметр в списке имён и т.п.), Ident остаётся **именем**, даже если текст совпадает с keyword.

Примеры позиций имени:
- в `DCL` / `DECLARE` списках имён;
- левая часть присваивания;
- операнд выражения, где грамматика ждёт reference;
- метка перед statement (`THEN:` как label — см. R4).

### R3 — Lookahead for statement keywords

Там, где keyword statement и reference неоднозначны (классическая боль PL/I), парсер использует **ограниченный lookahead** (идеи — pl1gcc `TokenBasedLookahead`, реализация своя):

- пример духа: после имени в начале statement решить, это `IF`-statement или присваивание/вызов;
- конкретные таблицы lookahead ведутся в реализации и закрепляются тестами must-parse; при изменении — регрессия.

### R4 — Labels

Метка — Ident (в т.ч. совпадающий с keyword) + `:` в позиции label prefix.  
`THEN: A = 1;` — метка `THEN`, не keyword THEN.  
Конфликт с `IF … THEN` решается грамматикой `IF`-statement (keyword THEN только в ветке IF).

### R5 — Abbreviations

Допустимые сокращения keywords (`PROC`/`PROCEDURE`, `DCL`/`DECLARE`, …) — по таблице G-ref/Classic для **нашего** L0-набора statements; полный список — приложение к главе 2.3. Сокращение в **name position** — обычное имя.

---

## Canonical example (must-parse)

```pli
X: PROC OPTIONS(MAIN);
  DCL (IF, THEN, ELSE) FIXED BIN(31);
  IF THEN THEN THEN = ELSE;
  ELSE ELSE = THEN;
END X;
```

Нормативное прочтение (`IF THEN THEN THEN = ELSE`):
- `IF` — keyword statement;
- первый `THEN` — **имя** (условие-выражение);
- второй `THEN` — keyword `THEN` конструкции IF;
- третий `THEN` — **имя** (lvalue присваивания);
- `ELSE` после `=` — **имя**;
- следующий `ELSE` — keyword ветки ELSE;
- и т.д.

Статус на L0: **must-parse**. Must-run — когда типы/присваивания/IF покрыты главами 3–4 и тест явно включён в must-run.

---

## Constraints

- Запрещено считать любой Ident keyword вне правил R1–R5.
- Запрещено «зарезервировать всё как в Pascal» ради упрощения парсера (противоречит ADR-0001 / манифесту).
- Сообщения об ошибках показывают **исходный** регистр текста.

---

## Implementation status

| Аспект | Статус |
|--------|--------|
| Правила R1–R5 | must (парсер) |
| Пример MP001 | must-parse L0 |
| Полная таблица lookahead | реализация + тесты; уточнение SPEC по мере стабилизации |
| Исполнение MP001 | not-implemented как must-run до отдельного включения |

---

## References

- G-ref: keywords not reserved  
- Classic LRM (идиомы)  
- ADR-0004 parser stack  
- `08-test-catalog-l0.md` MP001
