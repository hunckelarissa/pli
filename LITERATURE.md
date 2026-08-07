# Литература и исходные ориентиры

**Статус:** черновик к Техническому заданию и Формализованной спецификации  
**Связь:** [`MANIFEST.md`](MANIFEST.md)  
**Правило приоритета документов:** при конфликте побеждает **наша** Формализованная спецификация; литература — источник смысла, не «закон копировать целиком».

---

## 1. Исходники в репозитории — да, оставляем как ориентир

| Артефакт | Роль | Не роль |
|----------|------|---------|
| **`research/pl1gcc-0.0.15/`** (и zip) | Справочник по грамматике, scanner tricks (keywords-as-names), scopes/DCL, preprocessor-наброски, тестовые `.pl1`, исторический GCC-glue «как не надо» | Продуктовая кодовая база, объект порта на GCC 15+ |
| **`MANIFEST.md`** | Канон концепции | Спека языка |
| Будущие `TZ` / `SPEC` | Юридически-инженерный закон проекта | — |

**Вердикт:** исходники pl1gcc **хранить и читать** как археологию и ускоритель проектирования парсера. Реализацию писать **заново**. При желании позже — отдельный каталог `refs/` с выписанными фрагментами грамматики и комментариями «что берём / что отвергаем».

Вторичный ориентир (не в этом дереве, знать о существовании): [HSorensen/gccpl1](https://github.com/HSorensen/gccpl1) — поздняя попытка того же автора на ANTLR; тоже справочник, не база.

---

## 2. Слои литературы (как читать)

```text
  A. Закон горизонта     → Subset G (стандарты)
  B. Вкус классики       → IBM F / Checkout+Optimizer LRM
  C. Что не обещаем      → Enterprise LRM (карта «out-of-scope»)
  D. Практика subset     → DRI / DEC / Iron Spring / subsetg-book
  E. Компиляторная инженерия → книги и LLVM/C как backend-знание
  F. Археология парсера  → pl1gcc + NOTES/CHANGELOG
```

Порядок вычитки на берегу: **A → B → D → F**, параллельно лёгкий просмотр **C** (чтобы не утащить Enterprise в scope), **E** — по мере проектирования TZ/стека.

---

## 3. Стандарты (горизонт G и полный язык)

| Документ | Зачем нам |
|----------|-----------|
| **ANSI X3.74-1981** / **X3.74-1987** — *PL/I General-Purpose Subset* (Subset G) | Северная звезда совместимости; каркас «что должно быть в G» |
| **ISO/IEC 6522:1992** — *PL/I general purpose subset* | Международный близнец G; сверять терминологию |
| **ANSI X3.53-1976** (R1998) — полный PL/I | Граница «full vs G»; не цель реализации |
| **ISO 6160:1979** — полный PL/I | То же |
| **ECMA-50** (1976) — PL/I | Исторический европейский след стандартизации |

*Где взять:* стандарты часто платные (ANSI/ISO). Для работы на берегу — легальная копия или библиотечный доступ; содержание G дополнительно раскрыто в открытых вторичных источниках (§5).

---

## 4. IBM — классика (вкус языка, must-parse)

Искать на **[Bitsavers](https://bitsavers.org/)** (`pdf/ibm/360/pli/`, `pdf/ibm/370/pli/`) и в IBM Docs (для нового).

| Документ (ориентиры номеров) | Зачем |
|------------------------------|--------|
| **GC33-0009-x** — *OS PL/I Checkout and Optimizing Compilers: Language Reference* (напр. GC33-0009-4, Oct 1976) | **Главный классический LRM** пары Checkout/Optimizer; эталон идиом и семантики «того» PL/I |
| **SC33-0009** ранние издания | Эволюция того же LRM |
| *IBM System/360 Operating System PL/I (F) Language Reference* (серия GC28-… / предшественники) | F-level; наследие и сравнение с Optimizer |
| **GC33-0001** — Optimizing Compiler General Information | Контекст компилятора, не грамматика |
| **GC33-0003** — Checkout Compiler General Information | То же + отличия от F |
| **SC33-0025** — Optimizing *Execution Logic* | Модель выполнения, storage, library — важно для runtime-концепции |
| **SC33-0032** — Checkout *Execution Logic* | То же для checkout-модели |
| Programmer’s Guide / TSO / CMS User’s Guides (SC33-0029, -0033, -0037, -0047…) | I/O, JCL/окружение — выборочно, не тащить z/OS в MVP |

**Как использовать:** источник **семантики и примеров** для классики и keywords-as-names. Не копировать весь объём в MVP. В SPEC каждый must-run пункт желательно якорить: «как в GC33-0009 §…» или «как в G §…».

---

## 5. IBM — современность (карта out-of-scope)

| Документ | Зачем |
|----------|--------|
| **Enterprise PL/I for z/OS — Language Reference** (актуальный PDF на ibm.com/docs) | Видеть расширения Enterprise; помечать **out-of-scope** |
| Enterprise *Programming Guide* / *Compiler and Runtime Migration* | Понимать, чем «их» мир отличается от G/классики |
| Сравнение лимитов OS PL/I / VA / Enterprise (IBM Docs) | Реалистичные пределы реализаций |

**Правило:** Enterprise читаем, чтобы **не** обещать. Цитировать в SPEC только как «исключено» или «отложено за горизонт G».

---

## 6. Subset G и портируемые реализации (практика)

| Источник | Зачем |
|----------|--------|
| Paul Abrahams — *Subset/G PL/I and the PL/I Standard* (ACM, ~1983) | Зачем G существует; философия урезания |
| [johnwcowan/subsetg-book](https://github.com/johnwcowan/subsetg-book) | Живое открытое описание G для программистов (intro + dictionary) |
| Digital Research **PL/I-80 / PL/I-86** Language Reference (Subset G + их отличия) | Маленькая реализация G; полезные упрощения |
| DEC / VAX / Kednos PL/I docs (если найдутся) | G + надстройки; опыт «strict supersets of G» |
| **Iron Spring PL/I** — Programming Guide / FAQ / Release Notes ([iron-spring.com](http://www.iron-spring.com/)) | Современный pragmatic subset; что реально тяжело; **не** копировать закрытый компилятор |

---

## 7. Соседние диалекты (выборочно, не закон)

| Источник | Зачем осторожно смотреть |
|----------|---------------------------|
| Multics PL/I / EPL материалы | Системный диалект; идеи, не целевая совместимость |
| Учебные SP/k, PL/C (статьи/мануалы) | Лестницы subsets для обучения — метафора уровней L0–L2 |
| Wikipedia *PL/I* + ссылки на стандарты | Навигация, не нормативная база |

---

## 8. Инженерия компиляторов и backend (не PL/I-спека)

| Источник | Зачем |
|----------|--------|
| Aho et al. — *Compilers: Principles, Techniques, and Tools* (Dragon Book) | Классика фаз компилятора |
| Cooper & Torczon — *Engineering a Compiler* | Инженерный взгляд на IR, семантики |
| Appel — *Modern Compiler Implementation* (C/Java/ML — по вкусу) | Компактные каркасы |
| Документация **языка C** (C11/C17) как **целевого** IR транспиляции | Контракт эмита |
| LLVM Language Reference / Programmer’s Manual | **Позже**, второй backend |
| ANTLR mega-tutorial / Terence Parr — *The Definitive ANTLR 4 Reference* | Если парсерный стек = ANTLR |
| Материалы по flex/bison **только** как фон к чтению pl1gcc | Не обязательный стек продукта |

ULM Modula-2 (ваш опыт) — **внутренний** ориентир дисциплины большого фронтенда; в библиографию продукта как внешний PDF можно не включать, в TZ — да («параллельный проект, разные контракты codegen»).

---

## 9. Археология pl1gcc (что именно вычитывать)

В `research/pl1gcc-0.0.15/pl1/`:

| Файл / зона | Зачем |
|-------------|--------|
| `Announce-0.0.15`, `README`, `INSTALL` | Честные границы (нет codegen) |
| `NOTES`, `CHANGELOG` | Дневник решений, TODO, грабли keywords/`^`/DCL |
| `pl1-scanner.l`, `pl1-parser.y`, `pl1pp-parser.y` | Грамматика и трюки |
| `libpl1/` (scopes, DCL, messages) | Модели структур — идеи для нашего AST |
| `tests/*.pl1` | Зародыши must-parse / регрессии |
| `pl11.c`, `pl1tree.c`, `Make-lang.in` | Образец **устаревшего** GCC glue — антипаттерн для нас |

---

## 10. Машиночитаемый источник правды

Отобранное must-have сконвертировано в Markdown:

- каталог: [`docs/`](docs/README.md)
- пересборка: `python tools/build_docs_truth.py`

Туда входят: Subset G (HTML/Kednos-draft), ANSI X3.53 full, GC33-0009-4, SC33-0025-3, SC26-3114, GC28-8201-4, Iron Spring guide, subsetg-book.

---

## 11. Открытый доступ и скрипт загрузки

| Источник | Открыто? | Как брать |
|----------|----------|-----------|
| Исторические IBM LRM на **Bitsavers** | Да (архив) | `python tools/fetch_literature.py` |
| Iron Spring docs (сайт вендора) | Да | тот же скрипт |
| [subsetg-book](https://github.com/johnwcowan/subsetg-book) | Да (MIT) | git clone через скрипт |
| **ANSI / ISO** стандарты (X3.74, 6522, …) | **Нет** (платно) | Купить / библиотека; скрипт **не** качает |
| ACM Abrahams | Обычно paywall | Легальный доступ |
| Enterprise PL/I LRM | Часто PDF с **ibm.com/docs** | Вручную с официального сайта |
| Коммерческие книги (Dragon Book и т.п.) | Нет | Купить |

Скрипт: [`tools/fetch_literature.py`](tools/fetch_literature.py)  
Каталог полки (сырьё): `research/refs/literature/`  
Архив всего сырья: [`research/`](research/README.md)

```text
python tools/fetch_literature.py --dry-run
python tools/fetch_literature.py --only core
python tools/fetch_literature.py
```

После OCR кладите машиночитаемый текст рядом с PDF (например `*.pdf.txt` или `ocr/`).

---

## 12. Минимальный «береговой» набор PDF/текстов на первый месяц

Обязательный каркас вычитки (сырые копии в `research/refs/literature/` и books-папке; читать из `docs/`):

1. Subset G — стандарт **или** subsetg-book + Abrahams  
2. **GC33-0009-4** (или ближайшее полное издание LRM Checkout/Optimizer)  
3. Один Execution Logic (**SC33-0025** или аналог) — набросок runtime-модели  
4. Iron Spring Programming Guide — pragmatic checklist  
5. Enterprise LRM — только оглавление + список «не наше» (официальный IBM Docs)  
6. Дерево **research/pl1gcc-0.0.15** — в архиве сырья  

Без этого комплекта Формализованную спецификацию писать рано; с ним — можно якорить каждый раздел SPEC.

---

## 13. Политика цитирования в будущей SPEC

- Каждый нормативный пункт языка: ссылка **G** и/или **GC33-0009** (или явный «диалект проекта»).  
- Каждый отказ: ссылка на Enterprise или full, пометка `out-of-scope`.  
- Каждый трюк парсера, вдохновлённый pl1gcc: «идея из pl1gcc, реализация своя».  
- Не смешивать уровни документов: манифест → TZ → SPEC → код.

---

*Документ дополняется по мере появления легальных копий и выбора парсерного стека. Манифест при этом можно не трогать.*
